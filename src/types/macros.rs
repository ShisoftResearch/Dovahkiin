macro_rules! gen_primitive_types_io {
    (
        $($t:ty: $tmod:ident $feat_writer: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use std::mem;

                    pub type Slice = &'static [$t];
                    pub type ReadRef = &'static $t;

                    pub fn read(mem_ptr: usize) -> ReadRef {
                        debug_assert!(mem_ptr > 0);
                        unsafe {
                            &*(mem_ptr as *const $t)
                        }
                    }
                    pub fn read_slice(mem_ptr: usize, len: usize) -> (Slice, usize) {
                        unsafe {
                            let slice = std::slice::from_raw_parts(mem_ptr as *const _, len);
                            let size = size(0) * len;
                            (slice, size)
                        }
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        debug_assert!(mem_ptr > 0);
                        unsafe {
                            std::ptr::write(mem_ptr as *mut $t, *val)
                        }
                    }
                    pub fn size(_: usize) -> usize {
                        mem::size_of::<$t>()
                    }
                    pub fn val_size(_: &$t) -> usize {
                        size(0)
                    }
                    pub fn feature(val: &$t) -> [u8; 8] {
                        $feat_writer(*val)
                    }
                    pub fn hash(val: &$t) -> [u8; 8] {
                        feature(val)
                    }
                }
            )*
    );
}

macro_rules! big_end {
    (
        $writer:ident
    ) => {
        |n| {
            use byteorder::WriteBytesExt;
            let mut key_slice = [0u8; 8];
            {
                let mut cursor = ::std::io::Cursor::new(&mut key_slice[..]);
                cursor.$writer::<::byteorder::BigEndian>(n).unwrap();
            };
            key_slice
        }
    };
}

macro_rules! big_end_cast {
    () => {
        |n| {
            let big_end = big_end!(write_i32);
            big_end(n as i32)
        }
    };
}

macro_rules! gen_compound_types_io {
    (
        $($t:ident, $tmod:ident, $feat: expr, $hash: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;

                    pub type Slice = &'static [$t];
                    pub type ReadRef = &'static $t;

                    pub fn read(mem_ptr: usize) -> ReadRef {
                        unsafe {
                            &*(mem_ptr as *const $t)
                        }
                    }
                    pub fn read_slice(mem_ptr: usize, len: usize) -> (Slice, usize) {
                        unsafe {
                            let slice = std::slice::from_raw_parts(mem_ptr as *const _, len);
                            let size = size(0) * len;
                            (slice, size)
                        }
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        unsafe {
                            std::ptr::write(mem_ptr as *mut $t, val.to_owned())
                        }
                    }
                    pub fn size(_: usize) -> usize {
                        std::mem::size_of::<$t>()
                    }
                    pub fn val_size(_: &$t) -> usize {
                        size(0)
                    }
                    pub fn feature(val: &$t) -> [u8; 8] {
                        ($feat)(val)
                    }
                    pub fn hash(val: &$t) -> [u8; 8] {
                        ($hash)(val)
                    }
                }
            )*
    );
}

macro_rules! gen_variable_types_io {
    (
        $(
            $t:ty,
            $rt: ty,
            $tmod:ident,
            $reader:expr,
            $writer: expr,
            $size:expr,
            $val_size:expr,
            $feat: expr,
            $hash: expr
        );*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;

                    pub type Slice = Vec<ReadRef>;
                    pub type ReadRef = &'static $rt;

                    pub fn read(mem_ptr: usize) -> ReadRef {
                        ($reader)(mem_ptr)
                    }
                    pub fn read_slice(mut mem_ptr: usize, len: usize) -> (Slice, usize) {
                        let origin_ptr = mem_ptr;
                        let res = (0..len).map(|_| {
                            let v = read(mem_ptr);
                            mem_ptr += val_size(v);
                            v
                        })
                        .collect::<Vec<_>>();
                        (res, mem_ptr - origin_ptr)
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        ($writer)(val, mem_ptr)
                    }
                    pub fn size(mem_ptr: usize) -> usize {
                        ($size)(mem_ptr)
                    }
                    pub fn val_size(val: &$rt) -> usize {
                        ($val_size)(val)
                    }
                    pub fn feature(val: &$rt) -> [u8; 8] {
                        ($feat)(val)
                    }
                    pub fn hash(val: &$rt) -> [u8; 8] {
                        ($hash)(val)
                    }
                }
            )*
    );
}

macro_rules! get_from_val {
    ($e:ident, $d:ident) => {
        match &$d {
            &OwnedValue::$e(v) => Some(v),
            _ => None,
        }
    };
}

macro_rules! ref_from_val_fn {
    ($e:ident, $fn: ident, $st:ty) => {
        pub fn $fn(&self) -> Option<$st> {
            ref_from_val!($e, self)
        }
    };
}

macro_rules! ref_from_val {
    ($e:ident, $d:ident) => {
        match $d {
            &SharedValue::$e(v) => Some(v),
            _ => None,
        }
    };
}

macro_rules! get_from_val_fn {
    ($e:ident, $fn: ident, $t:ty) => {
        #[allow(non_snake_case)]
        pub fn $fn(&self) -> Option<&$t> {
            get_from_val!($e, self)
        }
    };
}

macro_rules! define_types {
    (
        $(
            [ $( $name:expr ),* ], $id:expr, $t:ty, $e:ident, $io:ident, $fn: ident
         );*
    ) => (

        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Type {
            $(
                $e = $id,
            )*
            Map = 1024, // No matter which id we pick for 'Map' because w/r planners will ignore it when sub_fields is not 'None',
            Default = 0
        }

        $(
            impl ToValue for $t {
                fn value(self) -> OwnedValue {
                    OwnedValue::$e(self)
                }
            }
            impl ToValue for Vec<$t> {
                fn value(self) -> OwnedValue {
                    OwnedValue::PrimArray(OwnedPrimArray::$e(self))
                }
            }
        )*

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub enum OwnedPrimArray {
              $(
                  $e(Vec<$t>),
              )*
        }

        impl OwnedPrimArray {
            pub fn size(&self) -> usize {
                match &self {
                    $(
                        &OwnedPrimArray::$e(vec) => {
                            return vec.iter().map(|v| $io::val_size(v)).sum()
                        }
                    ),*
                }
            }
            pub fn len(&self) -> usize {
                match self {
                    $(
                        OwnedPrimArray::$e(vec) => {
                            return vec.len()
                        }
                    ),*
                }
            }
            pub fn features(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match &self {
                    $(
                        &OwnedPrimArray::$e(vec) => {
                            for v in vec {
                                res.push($io::feature(v));
                            }
                        }
                    ),*
                }
                res
            }
            pub fn data_size(&self) -> u8 {
                match &self {
                   $(
                        &OwnedPrimArray::$e(vec) => $io::val_size(&vec[0]) as u8
                   ),*
                }
            }
            pub fn hashes(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match &self {
                    $(
                        &OwnedPrimArray::$e(vec) => {
                            for v in vec {
                                res.push($io::hash(v));
                            }
                        }
                    ),*
                }
                res
            }
            $(
                pub fn $fn(&self) -> Option<&Vec<$t>> {
                    match self {
                        OwnedPrimArray::$e(ref vec) => Some(vec),
                        _ => None
                    }
                }
            )*
        }


        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub enum OwnedValue {
            $(
                $e($t),
            )*
            Map(OwnedMap),
            Array(Vec<OwnedValue>),
            PrimArray(OwnedPrimArray),
            NA,
            Null
        }

        impl OwnedValue {
            $(
                get_from_val_fn!($e, $fn, $t);
            )*
            #[allow(non_snake_case)]
            pub fn Map(&self) -> Option<&OwnedMap> {
                match self {
                    &OwnedValue::Map(ref m) => Some(m),
                    _ => None
                }
            }
            pub fn base_size(&self) -> usize {
                get_vsize(self.base_type_id(), self)
            }
            pub fn cloned_iter_value(&self) -> Option<IntoIter<OwnedValue>> {
                match self {
                    OwnedValue::Array(ref array) => Some(array.clone().into_iter()),
                    $(OwnedValue::PrimArray(OwnedPrimArray::$e(ref vec)) => {
                        let packed_iter = vec.iter().map(|v| v.clone().value());
                        let packed_vec: Vec<_> = packed_iter.collect();
                        Some(packed_vec.into_iter())
                    },)*
                    _ => None
                }
            }
            pub fn iter_value(&self) -> Option<ValueIter> {
                if let OwnedValue::Array(ref array) = self {
                    Some(ValueIter::new(array))
                } else { None }
            }
            pub fn len(&self) -> Option<usize> {
                match self {
                    OwnedValue::Array(ref array) => Some(array.len()),
                    $(OwnedValue::PrimArray(OwnedPrimArray::$e(ref vec)) => Some(vec.len()),)*
                    _ => None
                }
            }
            pub fn feature(&self) -> [u8; 8] {
                match &self {
                    $(
                        &OwnedValue::$e(v) => $io::feature(&v)
                    ),*,
                    &OwnedValue::Map(_) | &OwnedValue::Array(_) | &OwnedValue::PrimArray(_) => unreachable!(),
                    _ => [0u8; 8]
                }
            }

            pub fn features(&self) -> Vec<[u8; 8]> {
                match self {
                    OwnedValue::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.feature());
                        }
                        res
                    },
                    OwnedValue::PrimArray(ref prim_arr) => {
                        prim_arr.features()
                    },
                    _ => unreachable!()
                }
            }

            pub fn hash(&self) -> [u8; 8] {
                match &self {
                    $(
                        &OwnedValue::$e(v) => $io::hash(&v)
                    ),*,
                    OwnedValue::Map(_) | OwnedValue::Array(_) | OwnedValue::PrimArray(_) => panic!(),
                    _ => [0u8; 8]
                }
            }

            pub fn hashes(&self) -> Vec<[u8; 8]> {
                match self {
                    OwnedValue::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.hash());
                        }
                        res
                    },
                    OwnedValue::PrimArray(ref prim_arr) => {
                        prim_arr.hashes()
                    },
                    _ => unreachable!()
                }
            }
            pub fn base_type_id(&self) -> u32 {
                match self {
                    $(
                        &OwnedValue::$e(ref _v) => $id,
                    )*
                    // &OwnedValue::Array(ref v) => v[0].base_type_id(),
                    $(OwnedValue::PrimArray(OwnedPrimArray::$e(_)) => $id,)*
                    _ => 0
                }
            }
            pub fn prim_array(&self) -> Option<&OwnedPrimArray> {
                match self {
                    &OwnedValue::PrimArray(ref pa) => Some(pa),
                    _ => None
                }
            }
        }
        pub fn get_type_id (name: String) -> u32 {
           match name.as_ref() {
                $(
                    $($name => $id,)*
                )*
                _ => 0,
           }
        }
        pub fn get_id_type (id: u32) -> &'static str {
           match id {
                $(
                    $id => [$($name),*][0],
                )*
                _ => "N/A",
           }
        }
        pub fn get_size (id: u32, mem_ptr: usize) -> usize {
           match id {
                $(
                    $id => $io::size(mem_ptr),
                )*
                _ => 0,
           }
        }
        pub fn get_owned_val (id:u32, mem_ptr: usize) -> OwnedValue {
            match id {
                $(
                    $id => {
                        let val: $t = $io::read(mem_ptr).to_owned().into();
                        OwnedValue::$e(val)
                    },
                )*
                _ => OwnedValue::NA,
            }
       }
       pub fn get_shared_val (id:u32, mem_ptr: usize) -> SharedValue {
            match id {
                $(
                    $id => SharedValue::$e($io::read(mem_ptr)),
                )*
                _ => SharedValue::NA,
            }
        }
        pub fn get_owned_prim_array_val(id:u32, size: usize, mem_ptr: &mut usize) -> Option<OwnedPrimArray> {
             match id {
                 $(
                     $id => {
                        let mut vals: Vec<$t> = Vec::with_capacity(size);
                        for _ in 0..size {
                            let read_res = $io::read(*mem_ptr).to_owned();
                            vals.push(read_res.into());
                            *mem_ptr += get_size(id, *mem_ptr);
                        }
                        Some(OwnedPrimArray::$e(vals))
                     },
                 )*
                 _ => None,
             }
        }
        pub fn get_shared_prim_array_val(id:u32, len: usize, mem_ptr: &mut usize) -> Option<SharedPrimArray> {
            match id {
                $(
                    $id => {
                       let (slice, size) = $io::read_slice(*mem_ptr, len);
                       *mem_ptr += size;
                       Some(SharedPrimArray::$e(slice))
                    },
                )*
                _ => None,
            }
       }
        pub fn set_val (id:u32, val: &OwnedValue, mut mem_ptr: usize) {
             match id {
                 $(
                     $id => {
                        if let &OwnedValue::PrimArray(OwnedPrimArray::$e(vec)) = &val {
                            for v in vec.iter() {
                                $io::write(v , mem_ptr);
                                mem_ptr += $io::val_size(v);
                            }
                        } else {
                            if let Some(val) = get_from_val!($e, val) {
                                $io::write(val, mem_ptr);
                            } else {
                                panic!("value does not match type id {}, actual value {:?}", id, val);
                            }
                        }
                     },
                 )*
                 _ => panic!("Type id not illegal {}", id),
             }
        }
        pub fn get_vsize (id: u32, val: &OwnedValue) -> usize {
            match id {
                $(
                    $id => {
                        if let Some(val) = get_from_val!($e, val) {
                            $io::val_size(val)
                        } else {
                            panic!("value does not match type id {}, actual value {:?}", id, val);
                        }
                    },
                )*
                _ => {panic!("type id does not found");},
           }
        }
        pub fn get_rsize (id: u32, val: &SharedValue) -> usize {
            match id {
                $(
                    $id => {
                        if let Some(val) = ref_from_val!($e, val) {
                            $io::val_size(val)
                        } else {
                            panic!("value does not match type id {}, actual value {:?}", id, val);
                        }
                    },
                )*
                _ => {panic!("type id does not found");},
           }
        }
        #[derive(Debug, PartialEq)]
        pub enum SharedPrimArray {
              $(
                  $e($io::Slice),
              )*
        }

        impl SharedPrimArray {
            pub fn size(&self) -> usize {
                match self {
                    $(
                        SharedPrimArray::$e(vec) => {
                            return vec.iter().map(|v| $io::val_size(v)).sum()
                        }
                    ),*
                }
            }
            pub fn len(&self) -> usize {
                match self {
                    $(
                        SharedPrimArray::$e(vec) => {
                            return vec.len()
                        }
                    ),*
                }
            }
            pub fn features(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match self {
                    $(
                        SharedPrimArray::$e(vec) => {
                            for v in vec.iter() {
                                res.push($io::feature(v));
                            }
                        }
                    ),*
                }
                res
            }
            pub fn data_size(&self) -> u8 {
                match self {
                   $(
                        SharedPrimArray::$e(vec) => $io::val_size(&vec[0]) as u8
                   ),*
                }
            }
            pub fn hashes(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match self {
                    $(
                        SharedPrimArray::$e(vec) => {
                            for v in vec.iter() {
                                res.push($io::hash(v));
                            }
                        }
                    ),*
                }
                res
            }
            $(
                pub fn $fn(&self) -> Option<& $io::Slice> {
                    match self {
                        SharedPrimArray::$e(ref vec) => Some(vec),
                        _ => None
                    }
                }
            )*
        }

        #[derive(Debug, PartialEq)]
        pub enum SharedValue {
            $(
                $e($io::ReadRef),
            )*
            Map(SharedMap),
            Array(Vec<SharedValue>),
            PrimArray(SharedPrimArray),
            NA,
            Null
        }
        impl SharedValue {
            $(
                ref_from_val_fn!($e, $fn, $io::ReadRef);
            )*

            pub fn owned(&self) -> OwnedValue {
                match self {
                    $(
                        SharedValue::$e(ref v) => {
                            let v: $t = (*v).to_owned().into();
                            OwnedValue::$e(v)
                        }
                    ),*,
                    SharedValue::Array(ref array) => OwnedValue::Array(array.iter().map(|v| v.owned()).collect()),
                    $(
                        SharedValue::PrimArray(SharedPrimArray::$e(ref vec)) => OwnedValue::PrimArray(OwnedPrimArray::$e(vec
                            .iter()
                            .map(|v| {
                                (*v).to_owned().into()
                            })
                        .collect())),
                    )*
                    SharedValue::Map(ref map) => OwnedValue::Map(map.owned()),
                    SharedValue::NA => OwnedValue::NA,
                    SharedValue::Null => OwnedValue::Null,
                }
            }

            #[allow(non_snake_case)]
            pub fn Map(&self) -> Option<&SharedMap> {
                match self {
                    &SharedValue::Map(ref m) => Some(m),
                    _ => None
                }
            }
            pub fn base_size(&self) -> usize {
                get_rsize(self.base_type_id(), self)
            }
            pub fn len(&self) -> Option<usize> {
                match self {
                    SharedValue::Array(ref array) => Some(array.len()),
                    $(SharedValue::PrimArray(SharedPrimArray::$e(ref vec)) => Some(vec.len()),)*
                    _ => None
                }
            }
            pub fn feature(&self) -> [u8; 8] {
                match self {
                    $(
                        SharedValue::$e(ref v) => $io::feature(v)
                    ),*,
                    SharedValue::Map(_) | SharedValue::Array(_) | SharedValue::PrimArray(_) => unreachable!(),
                    _ => [0u8; 8]
                }
            }

            pub fn features(&self) -> Vec<[u8; 8]> {
                match self {
                    SharedValue::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.feature());
                        }
                        res
                    },
                    SharedValue::PrimArray(ref prim_arr) => {
                        prim_arr.features()
                    },
                    _ => unreachable!()
                }
            }

            pub fn hash(&self) -> [u8; 8] {
                match self {
                    $(
                        &SharedValue::$e(v) => $io::hash(v)
                    ),*,
                    SharedValue::Map(_) | SharedValue::Array(_) | SharedValue::PrimArray(_) => panic!(),
                    _ => [0u8; 8]
                }
            }

            pub fn hashes(&self) -> Vec<[u8; 8]> {
                match self {
                    SharedValue::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.hash());
                        }
                        res
                    },
                    SharedValue::PrimArray(ref prim_arr) => {
                        prim_arr.hashes()
                    },
                    _ => unreachable!()
                }
            }
            pub fn base_type_id(&self) -> u32 {
                match self {
                    $(
                        &SharedValue::$e(ref _v) => $id,
                    )*
                    &SharedValue::Array(ref v) => v[0].base_type_id(),
                    $(SharedValue::PrimArray(SharedPrimArray::$e(_)) => $id,)*
                    _ => 0
                }
            }
            pub fn prim_array(&self) -> Option<&SharedPrimArray> {
                match self {
                    &SharedValue::PrimArray(ref pa) => Some(pa),
                    _ => None
                }
            }
        }

        pub type GenericValue = dyn Value;

        pub trait Value {
            $(
                fn $fn(&self) -> Option<$t>;
            )*
            fn feature(&self) -> [u8; 8];
            fn features(&self) -> Vec<[u8; 8]>;
            fn hash(&self) -> [u8; 8];
            fn hashes(&self) -> Vec<[u8; 8]>;
            fn base_type_id(&self) -> u32;
            fn index_of(&self, index: usize) -> &dyn Value;
            fn base_size(&self) -> usize;
            fn prim_array_data_size(&self) -> Option<u8>;
            fn uni_array(&self) -> Option<Vec<&dyn Value>>;
        }

        impl Value for OwnedValue {
            $(
                fn $fn(&self) -> Option<$t> {
                    OwnedValue::$fn(self).map(|v| v.to_owned())
                }
            )*

            fn feature(&self) -> [u8; 8] {
                OwnedValue::feature(self)
            }
            fn features(&self) -> Vec<[u8; 8]> {
                OwnedValue::features(&self)
            }
            fn hash(&self) -> [u8; 8] {
                OwnedValue::hash(self)
            }
            fn hashes(&self) -> Vec<[u8; 8]> {
                OwnedValue::hashes(self)
            }
            fn base_type_id(&self) -> u32 {
                OwnedValue::base_type_id(&self)
            }
            fn index_of(&self, index: usize) -> &dyn Value {
                &self[index]
            }
            fn base_size(&self) -> usize {
                OwnedValue::base_size(&self)
            }
            fn prim_array_data_size(&self) -> Option<u8> {
                match self {
                    OwnedValue::PrimArray(arr) => Some(arr.data_size()),
                    _ => None,
                }
            }
            fn uni_array(&self) -> Option<Vec<&dyn Value>> {
                match self {
                    OwnedValue::Array(arr) => Some(arr.iter().map(|v| v as &dyn Value).collect()),
                    _ => None
                }
            }
        }

        impl Index<usize> for OwnedValue {
            type Output = Self;
            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    &Self::Array(ref array) => array.get(index).unwrap_or(&NULL_OWNED_VALUE),
                    &Self::Map(ref map) => map.get_by_key_id(index as u64),
                    _ => &NULL_OWNED_VALUE,
                }
            }
        }

        impl Index<u64> for OwnedValue {
            type Output = Self;

            fn index(&self, index: u64) -> &Self::Output {
                match self {
                    &Self::Map(ref map) => map.get_by_key_id(index),
                    &Self::Array(ref array) => array.get(index as usize).unwrap_or(&NULL_OWNED_VALUE),
                    _ => &NULL_OWNED_VALUE,
                }
            }
        }

        impl Value for SharedValue {
            $(
                fn $fn(&self) -> Option<$t> {
                    SharedValue::$fn(self).map(|v| v.to_owned().into())
                }
            )*
            fn feature(&self) -> [u8; 8] {
                SharedValue::feature(self)
            }
            fn features(&self) -> Vec<[u8; 8]> {
                SharedValue::features(&self)
            }
            fn hash(&self) -> [u8; 8] {
                SharedValue::hash(self)
            }
            fn hashes(&self) -> Vec<[u8; 8]> {
                SharedValue::hashes(self)
            }
            fn base_type_id(&self) -> u32 {
                SharedValue::base_type_id(&self)
            }
            fn index_of(&self, index: usize) -> &dyn Value {
                &self[index]
            }
            fn base_size(&self) -> usize {
               SharedValue::base_size(self)
            }
            fn prim_array_data_size(&self) -> Option<u8> {
                match self {
                    SharedValue::PrimArray(arr) => Some(arr.data_size()),
                    _ => None,
                }
            }
            fn uni_array(&self) -> Option<Vec<&dyn Value>> {
                match self {
                    SharedValue::Array(arr) => Some(arr.iter().map(|v| v as &dyn Value).collect()),
                    _ => None
                }
            }
        }

        impl Index<usize> for SharedValue {
            type Output = Self;

            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    &Self::Array(ref array) => array.get(index).unwrap_or(&NULL_SHARED_VALUE),
                    &Self::Map(ref map) => map.get_by_key_id(index as u64),
                    _ => &NULL_SHARED_VALUE,
                }
            }
        }

        impl Index<u64> for SharedValue {
            type Output = Self;

            fn index(&self, index: u64) -> &Self::Output {
                match self {
                    &Self::Map(ref map) => map.get_by_key_id(index),
                    &Self::Array(ref array) => array.get(index as usize).unwrap_or(&NULL_SHARED_VALUE),
                    _ => &NULL_SHARED_VALUE,
                }
            }
        }

    );
}
