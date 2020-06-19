macro_rules! gen_primitive_types_io {
    (
        $($t:ty: $tmod:ident $feat_writer: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use std::ptr;
                    use std::mem;
                    pub fn read(mem_ptr: usize) -> $t {
                         unsafe {
                            ptr::read(mem_ptr as *mut $t)
                        }
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        unsafe {
                            ptr::write(mem_ptr as *mut $t, *val)
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
        $($t:ident, $tmod:ident, $reader:expr, $writer: expr, $size:expr, $feat: expr, $hash: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;
                    pub fn read(mem_ptr: usize) -> $t {
                        let read = $reader;
                        read(mem_ptr)
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        let write = $writer;
                        write(val, mem_ptr)
                    }
                    pub fn size(_: usize) -> usize {
                        $size
                    }
                    pub fn val_size(_: &$t) -> usize {
                        size(0)
                    }
                    pub fn feature(val: &$t) -> [u8; 8] {
                        let feature = $feat;
                        feature(val)
                    }
                    pub fn hash(val: &$t) -> [u8; 8] {
                        let hash = $hash;
                        hash(val)
                    }
                }
            )*
    );
}

macro_rules! gen_variable_types_io {
    (
        $($t:ident, $tmod:ident, $reader:expr, $writer: expr, $size:expr, $val_size:expr, $feat: expr, $hash: expr);*
    ) => (
            $(
                pub mod $tmod {
                    use types::*;
                    pub fn read(mem_ptr: usize) -> $t {
                        let read = $reader;
                        read(mem_ptr)
                    }
                    pub fn write(val: &$t, mem_ptr: usize) {
                        let write = $writer;
                        write(val, mem_ptr)
                    }
                    pub fn size(mem_ptr: usize) -> usize {
                        let size = $size;
                        size(mem_ptr)
                    }
                    pub fn val_size(val: &$t) -> usize {
                        let size = $val_size;
                        size(val)
                    }
                    pub fn feature(val: &$t) -> [u8; 8] {
                        let feature = $feat;
                        feature(val)
                    }
                    pub fn hash(val: &$t) -> [u8; 8] {
                        let hash = $hash;
                        hash(val)
                    }
                }
            )*
    );
}

macro_rules! get_from_val {
    (true, $e:ident, $d:ident) => {
        match $d {
            &Value::$e(ref v) => Some(v),
            _ => None,
        }
    };
    (false, $e:ident, $d:ident) => {
        match $d {
            &Value::$e(ref v) => Some(v),
            _ => None,
        }
    };
}

macro_rules! get_from_val_fn {
    (true, $e:ident, $t:ty) => (
        #[allow(non_snake_case)] 
        pub fn $e(&self) -> Option<&$t> {
            get_from_val!(true, $e, self)
        }
    );
    (false, $e:ident, $t:ty) => (
        #[allow(non_snake_case)] 
        pub fn $e(&self) -> Option<&$t> {
            get_from_val!(false, $e, self)
        }
    )
}

macro_rules! define_types {
    (
        $(
            [ $( $name:expr ),* ], $id:expr, $t:ty, $e:ident, $r:ident, $io:ident
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
                fn value(self) -> Value {
                    Value::$e(self)
                }
            }
            impl ToValue for Vec<$t> {
                fn value(self) -> Value {
                    Value::PrimArray(PrimitiveArray::$e(self))
                }
            }
        )*

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub enum Value {
            $(
                $e($t),
            )*
            Map(Map),
            Array(Vec<Value>),
            PrimArray(PrimitiveArray),
            NA,
            Null
        }

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub enum PrimitiveArray {
              $(
                  $e(Vec<$t>),
              )*
        }

        impl PrimitiveArray {
            pub fn size(&self) -> usize {
                match self {
                    $(
                        PrimitiveArray::$e(vec) => {
                            return vec.iter().map(|v| $io::val_size(v)).sum()
                        }
                    ),*
                }
            }
            pub fn len(&self) -> usize {
                match self {
                    $(
                        PrimitiveArray::$e(vec) => {
                            return vec.len()
                        }
                    ),*
                }
            }
            pub fn features(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match self {
                    $(
                        PrimitiveArray::$e(vec) => {
                            for v in vec {
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
                        PrimitiveArray::$e(vec) => $io::val_size(&vec[0]) as u8
                   ),*
                }
            }
            pub fn hashes(&self) -> Vec<[u8; 8]> {
                let mut res = vec![];
                match self {
                    $(
                        PrimitiveArray::$e(vec) => {
                            for v in vec {
                                res.push($io::hash(v));
                            }
                        }
                    ),*
                }
                res
            }
        }

        impl Value {
            $(
                get_from_val_fn!($r, $e, $t);
            )*
            #[allow(non_snake_case)] 
            pub fn Map(&self) -> Option<&Map> {
                match self {
                    &Value::Map(ref m) => Some(m),
                    _ => None
                }
            }
            pub fn base_size(&self) -> usize {
                get_vsize(self.base_type_id(), self)
            }
            pub fn cloned_iter_value(&self) -> Option<IntoIter<Value>> {
                match self {
                    Value::Array(ref array) => Some(array.clone().into_iter()),
                    $(Value::PrimArray(PrimitiveArray::$e(ref vec)) => {
                        let packed_iter = vec.iter().map(|v| v.clone().value());
                        let packed_vec: Vec<_> = packed_iter.collect();
                        Some(packed_vec.into_iter())
                    },)*
                    _ => None
                }
            }
            pub fn iter_value(&self) -> Option<ValueIter> {
                if let Value::Array(ref array) = self {
                    Some(ValueIter::new(array))
                } else { None }
            }
            pub fn len(&self) -> Option<usize> {
                match self {
                    Value::Array(ref array) => Some(array.len()),
                    $(Value::PrimArray(PrimitiveArray::$e(ref vec)) => Some(vec.len()),)*
                    _ => None
                }
            }
            pub fn feature(&self) -> [u8; 8] {
                match self {
                    $(
                        Value::$e(ref v) => $io::feature(v)
                    ),*,
                    Value::Map(_) | Value::Array(_) | Value::PrimArray(_) => unreachable!(),
                    _ => [0u8; 8]
                }
            }

            pub fn features(&self) -> Vec<[u8; 8]> {
                match self {
                    Value::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.feature());
                        }
                        res
                    },
                    Value::PrimArray(ref prim_arr) => {
                        prim_arr.features()
                    },
                    _ => unreachable!()
                }
            }

            pub fn hash(&self) -> [u8; 8] {
                match self {
                    $(
                        Value::$e(ref v) => $io::hash(v)
                    ),*,
                    Value::Map(_) | Value::Array(_) | Value::PrimArray(_) => panic!(),
                    _ => [0u8; 8]
                }
            }

            pub fn hashes(&self) -> Vec<[u8; 8]> {
                match self {
                    Value::Array(ref vec) => {
                        let mut res = vec![];
                        for v in vec {
                            res.push(v.hash());
                        }
                        res
                    },
                    Value::PrimArray(ref prim_arr) => {
                        prim_arr.hashes()
                    },
                    _ => unreachable!()
                }
            }
            pub fn base_type_id(&self) -> u32 {
                match self {
                    $(
                        &Value::$e(ref _v) => $id,
                    )*
                    &Value::Array(ref v) => v[0].base_type_id(),
                    $(Value::PrimArray(PrimitiveArray::$e(_)) => $id,)*
                    _ => 0
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
        pub fn get_val (id:u32, mem_ptr: usize) -> Value {
             match id {
                 $(
                     $id => Value::$e($io::read(mem_ptr)),
                 )*
                 _ => Value::NA,
             }
        }
        pub fn get_prim_array_val(id:u32, size: usize, mem_ptr: &mut usize) -> Option<PrimitiveArray> {
             match id {
                 $(
                     $id => {
                        let mut vals = Vec::with_capacity(size);
                        for _ in 0..size {
                            vals.push($io::read(*mem_ptr));
                            *mem_ptr += get_size(id, *mem_ptr);
                        }
                        Some(PrimitiveArray::$e(vals))
                     },
                 )*
                 _ => None,
             }
        }
        pub fn set_val (id:u32, val: &Value, mut mem_ptr: usize) {
             match id {
                 $(
                     $id => {
                        if let &Value::PrimArray(PrimitiveArray::$e(ref vec)) = val {
                            for v in vec {
                                $io::write(v , mem_ptr);
                                mem_ptr += $io::val_size(v);
                            }
                        } else {
                            $io::write(get_from_val!($r, $e, val).unwrap() , mem_ptr);
                        }
                     },
                 )*
                 _ => panic!("Type id not illegal {}", id),
             }
        }
        pub fn get_vsize (id: u32, val: &Value) -> usize {
            match id {
                $(
                    $id => {
                        let val_opt = get_from_val!($r, $e, val);
                        if val_opt.is_none() {
                            panic!("value does not match type id {}, actual value {:?}", id, val);
                        } else {
                            $io::val_size(val_opt.unwrap())
                        }
                    },
                )*
                _ => {panic!("type id does not found");},
           }
        }
    );
}
