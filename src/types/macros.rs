
macro_rules! gen_primitive_types_io {
    (
        $($t:ty: $tmod:ident $feat_writer: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use std::mem;

                    pub type Slice<'a> = &'a [$t];
                    pub type ReadRef<'a> = &'a $t;

                    pub fn read<'a>(mem_ptr: usize) -> ReadRef<'a> {
                        debug_assert!(mem_ptr > 0);
                        unsafe {
                            &*(mem_ptr as *const $t)
                        }
                    }
                    pub fn read_slice<'a>(mem_ptr: usize, len: usize) -> (Slice<'a>, usize) {
                        unsafe {
                            let slice = std::slice::from_raw_parts(mem_ptr as *const _, len);
                            let size = type_size() * len;
                            (slice, size)
                        }
                    }
                    pub fn vec_to_read_ref(vec: &Vec<$t>) -> Slice {
                        vec.as_slice()
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        debug_assert!(mem_ptr > 0);
                        unsafe {
                            std::ptr::write(mem_ptr as *mut $t, *val)
                        }
                    }
                    pub const fn fixed_size() -> bool {
                        true
                    }
                    pub const fn type_size() -> usize {
                        mem::size_of::<$t>()
                    }
                    pub const fn type_align() -> usize {
                        mem::align_of::<$t>()
                    }
                    pub fn size_at(_: usize) -> usize {
                        type_size()
                    }
                    pub fn val_size(_: &$t) -> usize {
                        type_size()
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
                    use crate::types::*;

                    pub type Slice<'a> = &'a [$t];
                    pub type ReadRef<'a> = &'a $t;

                    pub fn read<'a>(mem_ptr: usize) -> ReadRef<'a> {
                        unsafe {
                            &*(mem_ptr as *const $t)
                        }
                    }
                    pub fn read_slice<'a>(mem_ptr: usize, len: usize) -> (Slice<'a>, usize) {
                        unsafe {
                            let slice = std::slice::from_raw_parts(mem_ptr as *const _, len);
                            let size = type_size() * len;
                            (slice, size)
                        }
                    }
                    pub fn vec_to_read_ref(vec: &Vec<$t>) -> Slice {
                        vec.as_slice()
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        unsafe {
                            std::ptr::write(mem_ptr as *mut $t, val.to_owned())
                        }
                    }
                    pub const fn type_size() -> usize {
                        std::mem::size_of::<$t>()
                    }
                    pub const fn type_align() -> usize {
                        std::mem::align_of::<$t>()
                    }
                    pub const fn fixed_size() -> bool {
                        true
                    }
                    pub fn size_at(_: usize) -> usize {
                        type_size()
                    }
                    pub fn val_size(_: &$t) -> usize {
                        type_size()
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
            $hash: expr,
            $as_ref: expr,
            $alignment: expr
        );*
    ) => (
            $(
                pub mod $tmod {
                    use crate::types::*;

                    pub type Slice<'a> = Vec<ReadRef<'a>>;
                    pub type ReadRef<'a> = &'a $rt;

                    pub fn read<'a>(mem_ptr: usize) -> ReadRef<'a> {
                        ($reader)(mem_ptr)
                    }
                    pub fn read_slice<'a>(mut mem_ptr: usize, len: usize) -> (Slice<'a>, usize) {
                        let origin_ptr = mem_ptr;
                        let res = (0..len).map(|_| {
                            let v = read(mem_ptr);
                            mem_ptr += val_size(v);
                            v
                        })
                        .collect::<Vec<_>>();
                        (res, mem_ptr - origin_ptr)
                    }
                    pub fn vec_to_read_ref<'a>(vec: &'a Vec<$t>) -> Slice<'a> {
                        vec.iter().map(|v| ($as_ref)(v)).collect()
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        ($writer)(val, mem_ptr)
                    }
                    pub fn type_size() -> usize {
                        panic!("variable type does not have type size")
                    }
                    pub const fn type_align() -> usize {
                        $alignment
                    }
                    pub const fn fixed_size() -> bool {
                        false
                    }
                    pub fn size_at(mem_ptr: usize) -> usize {
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
            [ $( $name:expr ),* ], $t:ty, $e:ident, $io:ident, $fn: ident
         );*
    ) => (

        #[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
        pub enum Type {
            Null,
            Map, // No matter which id we pick for 'Map' because w/r planners will ignore it when sub_fields is not 'None',
            $(
                $e,
            )*
            NA
        }

        impl Type {
            pub const fn id(self) -> u8  {
                self as u8
            }
            pub fn from_id(id: u8) -> Self {
                match id {
                    $(
                        type_id::$fn => Type::$e,
                    )*
                    _ => Type::NA
                }
            }
            pub fn size(&self) -> Option<usize> {
                match self {
                    $(
                        Type::$e => if $io::fixed_size() {
                            Some($io::type_size())
                        } else {
                            None
                        },
                    )*
                    _ => None
                }
            }
        }

        mod type_id {
            $(
                pub const $fn: u8 = super::Type::$e.id();
            )*
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

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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


        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
        pub enum OwnedValue {
            $(
                $e($t),
            )*
            Map(OwnedMap),
            Array(Vec<OwnedValue>),
            PrimArray(OwnedPrimArray),
            Null,
            NA
        }

        impl OwnedValue {
            $(
                get_from_val_fn!($e, $fn, $t);
            )*
            pub fn shared<'a>(&'a self) -> SharedValue<'a> {
                match self {
                    $(
                        OwnedValue::$e(ref v) => {
                            SharedValue::$e(v)
                        }
                    ),*,
                    OwnedValue::Array(ref array) => SharedValue::Array(array.iter().map(|v| v.shared()).collect()),
                    $(
                        OwnedValue::PrimArray(OwnedPrimArray::$e(ref vec)) => SharedValue::PrimArray(SharedPrimArray::$e($io::vec_to_read_ref(vec))),
                    )*
                    OwnedValue::Map(ref map) => SharedValue::Map(map.shared()),
                    OwnedValue::Null => SharedValue::Null,
                    OwnedValue::NA => SharedValue::NA,
                }
            }
            #[allow(non_snake_case)]
            pub fn Map(&self) -> Option<&OwnedMap> {
                match self {
                    &OwnedValue::Map(ref m) => Some(m),
                    _ => None
                }
            }
            pub fn base_size(&self) -> usize {
                get_vsize(self.base_type(), self)
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
                    OwnedValue::Map(ref map) => Some(map.len()),
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
            pub fn base_type(&self) -> Type {
                match self {
                    $(
                        &OwnedValue::$e(ref _v) => Type::$e,
                    )*
                    $(
                        OwnedValue::PrimArray(OwnedPrimArray::$e(_)) => Type::$e,
                    )*
                    &OwnedValue::Array(ref v) => v[0].base_type(),
                    &OwnedValue::Map(_) => Type::Map,
                    &OwnedValue::Null => Type::Null,
                    &OwnedValue::NA => Type::NA,
                }
            }
            pub fn prim_array(&self) -> Option<&OwnedPrimArray> {
                match self {
                    &OwnedValue::PrimArray(ref pa) => Some(pa),
                    _ => None
                }
            }
        }
        pub fn get_type_id (name: String) -> u8 {
           match name.as_ref() {
                $(
                    $($name => Type::$e.id(),)*
                )*
                _ => 0,
           }
        }
        pub fn get_type (t: Type) -> &'static str {
           match t {
                $(
                    Type::$e => [$($name),*][0],
                )*
                _ => "N/A",
           }
        }
        pub fn get_size (t: Type, mem_ptr: usize) -> usize {
           match t {
                $(
                    Type::$e => $io::size_at(mem_ptr),
                )*
                _ => 0
           }
        }
        pub fn get_owned_val (t: Type, mem_ptr: usize) -> OwnedValue {
            match t {
                $(
                    Type::$e => {
                        let val: $t = $io::read(mem_ptr).to_owned().into();
                        OwnedValue::$e(val)
                    },
                )*
                _ => OwnedValue::NA
            }
       }
       pub fn get_shared_val<'a> (t: Type, mem_ptr: usize) -> SharedValue<'a> {
            match t {
                $(
                    Type::$e => SharedValue::$e($io::read(mem_ptr)),
                )*
                _ => SharedValue::NA
            }
        }
        pub fn get_owned_prim_array_val(t: Type, size: usize, mem_ptr: &mut usize) -> Option<OwnedPrimArray> {
             match t {
                 $(
                    Type::$e => {
                        let mut vals: Vec<$t> = Vec::with_capacity(size);
                        for _ in 0..size {
                            let read_res = $io::read(*mem_ptr).to_owned();
                            vals.push(read_res.into());
                            *mem_ptr += get_size(t, *mem_ptr);
                        }
                        Some(OwnedPrimArray::$e(vals))
                     },
                 )*
                 _ => None
             }
        }
        pub fn get_shared_prim_array_val<'v>(t: Type, len: usize, mem_ptr: &mut usize) -> Option<SharedPrimArray<'v>> {
            match t {
                $(
                    Type::$e => {
                       let (slice, size) = $io::read_slice(*mem_ptr, len);
                       *mem_ptr += size;
                       Some(SharedPrimArray::$e(slice))
                    },
                )*
                _ => None,
            }
       }
        pub fn set_val (t: Type, val: &OwnedValue, mut mem_ptr: usize) {
             match t {
                 $(
                    Type::$e => {
                        if let &OwnedValue::PrimArray(OwnedPrimArray::$e(vec)) = &val {
                            for v in vec.iter() {
                                $io::write(v , mem_ptr);
                                mem_ptr += $io::val_size(v);
                            }
                        } else {
                            if let Some(val) = get_from_val!($e, val) {
                                $io::write(val, mem_ptr);
                            } else {
                                panic!("value does not match type id {:?}, actual value {:?}", t, val);
                            }
                        }
                     },
                 )*
                 _ => panic!("type {:?} does not supported for set_value", t)
             }
        }
        pub fn size_of_type(t: Type) -> usize {
            match t {
                $(
                    Type::$e => {
                        $io::type_size()
                    },
                )*
                _ => panic!("type {:?} does not supported for size_of", t)
           }
        }
        pub fn align_of_type(t: Type) -> usize {
            match t {
                $(
                    Type::$e => {
                        $io::type_align()
                    },
                )*
                _ => panic!("type {:?} does not supported for size_of", t)
           }
        }
        pub fn fixed_size(t: Type) -> bool {
            match t {
                $(
                    Type::$e => {
                        $io::fixed_size()
                    },
                )*
                _ => false
           }
        }
        pub fn get_vsize (t: Type, val: &OwnedValue) -> usize {
            match t {
                $(
                    Type::$e => {
                        if let Some(val) = get_from_val!($e, val) {
                            $io::val_size(val)
                        } else {
                            panic!("value does not match type id {:?}, actual value {:?}", t, val);
                        }
                    },
                )*
                _ => panic!("type {:?} does not supported for get_vsize", t)
           }
        }
        pub fn get_rsize (t: Type, val: &SharedValue) -> usize {
            match t {
                $(
                    Type::$e => {
                        if let Some(val) = ref_from_val!($e, val) {
                            $io::val_size(val)
                        } else {
                            panic!("value does not match type id {:?}, actual value {:?}", t, val);
                        }
                    },
                )*
                _ => panic!("type {:?} does not supported for get_rsize", t),
           }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub enum SharedPrimArray<'a> {
              $(
                  $e($io::Slice<'a>),
              )*
        }

        impl <'a> SharedPrimArray <'a> {
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

        #[derive(Debug, PartialEq, Clone)]
        pub enum SharedValue<'a> {
            $(
                $e($io::ReadRef<'a>),
            )*
            Map(SharedMap<'a>),
            Array(Vec<SharedValue<'a>>),
            PrimArray(SharedPrimArray<'a>),
            Null,
            NA
        }
        impl <'a> SharedValue <'a> {
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
                    SharedValue::Null => OwnedValue::Null,
                    SharedValue::NA => OwnedValue::NA,
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
                get_rsize(self.base_type(), self)
            }
            pub fn len(&self) -> Option<usize> {
                match self {
                    SharedValue::Array(ref array) => Some(array.len()),
                    SharedValue::Map(ref map) => Some(map.len()),
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
            pub fn base_type(&self) -> Type {
                match self {
                    $(
                        &SharedValue::$e(ref _v) => Type::$e,
                    )*
                    $(
                        SharedValue::PrimArray(SharedPrimArray::$e(_)) => Type::$e,
                    )*
                    &SharedValue::Array(ref v) => v[0].base_type(),
                    &SharedValue::Map(_) => Type::Map,
                    &SharedValue::Null => Type::Null,
                    &SharedValue::NA => Type::NA
                }
            }
            pub fn prim_array(&self) -> Option<&SharedPrimArray> {
                match self {
                    &SharedValue::PrimArray(ref pa) => Some(pa),
                    _ => None
                }
            }
        }

        impl <'a> Eq for SharedValue<'a> {
            // TODO: elaborate it
        }

        pub trait Value {
            type Map: Map;
            $(
                fn $fn(&self) -> Option<$t>;
            )*
            fn get_in_by_ids(&self, ids: &Vec<u64>) -> &Self;
            fn feature(&self) -> [u8; 8];
            fn features(&self) -> Vec<[u8; 8]>;
            fn hash(&self) -> [u8; 8];
            fn hashes(&self) -> Vec<[u8; 8]>;
            fn base_type(&self) -> Type;
            fn index_of(&self, index: usize) -> &Self;
            fn base_size(&self) -> usize;
            fn prim_array_data_size(&self) -> Option<u8>;
            fn uni_array(&self) -> Option<Vec<&Self>>;
            fn map(&self) -> Option<&Self::Map>;
        }

        impl Value for OwnedValue {

            type Map = OwnedMap;

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
            fn base_type(&self) -> Type {
                OwnedValue::base_type(self)
            }
            fn index_of(&self, index: usize) -> &Self {
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
            fn uni_array(&self) -> Option<Vec<&Self>> {
                match self {
                    OwnedValue::Array(arr) => Some(arr.iter().map(|v| v as &Self).collect()),
                    _ => None
                }
            }
            fn get_in_by_ids(&self, ids: &Vec<u64>) -> &Self {
                if let OwnedValue::Map(map) = &self {
                    map.get_in_by_ids(ids.iter())
                } else {
                    &OwnedValue::Null
                }
            }
            fn map(&self) -> Option<&OwnedMap> {
                match self {
                    OwnedValue::Map(map) => Some(map),
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

        impl <'a> Value for SharedValue<'a> {

            type Map = SharedMap<'a>;

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
            fn base_type(&self) -> Type {
                SharedValue::base_type(&self)
            }
            fn index_of(&self, index: usize) -> &Self {
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
            fn uni_array(&self) -> Option<Vec<&Self>> {
                match self {
                    SharedValue::Array(arr) => Some(arr.iter().map(|v| v as &Self).collect()),
                    _ => None
                }
            }
            fn get_in_by_ids(&self, ids: &Vec<u64>) -> &Self {
                if let SharedValue::Map(map) = &self {
                    map.get_in_by_ids(ids.iter())
                } else {
                    &SharedValue::Null
                }
            }
            fn map(&self) -> Option<&SharedMap<'a>> {
                match self {
                    SharedValue::Map(map) => Some(map),
                    _ => None
                }
            }
        }

        impl <'a> Index<usize> for SharedValue<'a> {
            type Output = Self;

            fn index(&self, index: usize) -> &Self::Output {
                match self {
                    &Self::Array(ref array) => array.get(index).unwrap_or(&NULL_SHARED_VALUE),
                    &Self::Map(ref map) => map.get_by_key_id(index as u64),
                    _ => &NULL_SHARED_VALUE,
                }
            }
        }

        impl <'a> Index<u64> for SharedValue<'a> {
            type Output = Self;

            fn index(&self, index: u64) -> &Self::Output {
                match self {
                    &Self::Map(ref map) => map.get_by_key_id(index),
                    &Self::Array(ref array) => array.get(index as usize).unwrap_or(&NULL_SHARED_VALUE),
                    _ => &NULL_SHARED_VALUE,
                }
            }
        }

        impl <'a> Default for SharedValue<'a> {
            fn default() -> Self {
                Self::NA
            }
        }

    );
}
