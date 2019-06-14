#[macro_use]
mod macros;
pub mod custom_types;
pub mod value;

use serde::Deserialize;
use std::vec::IntoIter;
pub use types::custom_types::any::*;
pub use types::custom_types::bytes::*;
pub use types::custom_types::id::*;
pub use types::custom_types::map::*;
pub use types::custom_types::pos::*;
pub use types::value::*;

gen_primitive_types_io!(
    bool:   bool_io       big_end_cast!();
    char:   char_io       big_end_cast!();
    i8:     i8_io         big_end_cast!();
    i16:    i16_io        big_end!(write_i16);
    i32:    i32_io        big_end!(write_i32);
    i64:    i64_io        big_end!(write_i64);
    u8:     u8_io         big_end_cast!();
    u16:    u16_io        big_end!(write_u16);
    u32:    u32_io        big_end!(write_u32);
    u64:    u64_io        big_end!(write_u64);
//    isize:  isize_io      ;
//    usize:  usize_io      ;
    f32:    f32_io        big_end!(write_f32);
    f64:    f64_io        big_end!(write_f64)
);

gen_compound_types_io! (
    Pos2d32, pos2d32_io, {
        |mem_ptr| {
            let x = f32_io::read(mem_ptr);
            let y = f32_io::read(mem_ptr + f32_io::size(0));
            Pos2d32 {x: x, y: y}
        }
    }, {
        |val: &Pos2d32, mem_ptr| {
            f32_io::write(&val.x, mem_ptr);
            f32_io::write(&val.y, mem_ptr + f32_io::size(0));
        }
    }, {
        f32_io::size(0) * 2
    }, {
        |_| [0u8; 8]
    }, {
        |val: &Pos2d32| {
            use std::hash::Hasher;
            let mut hasher = twox_hash::XxHash::default();
            hasher.write(&f32_io::feature(&val.x));
            hasher.write(&f32_io::feature(&val.y));
            u64_io::feature(&hasher.finish())
        }
    };

    Pos2d64, pos2d64_io, {
        |mem_ptr| {
            let x = f64_io::read(mem_ptr);
            let y = f64_io::read(mem_ptr + f64_io::size(0));
            Pos2d64 {x: x, y: y}
        }
    }, {
        |val: &Pos2d64, mem_ptr| {
            f64_io::write(&val.x, mem_ptr);
            f64_io::write(&val.y, mem_ptr + f64_io::size(0));
        }
    }, {
        f64_io::size(0) * 2
    }, {
        |_| [0u8; 8]
    }, {
        |val: &Pos2d64| {
            use std::hash::Hasher;
            let mut hasher = twox_hash::XxHash::default();
            hasher.write(&f64_io::feature(&val.x));
            hasher.write(&f64_io::feature(&val.y));
            u64_io::feature(&hasher.finish())
        }
    };

    //////////////////////////////////////////////////////////////

    Pos3d32, pos3d32_io, {
        |mem_ptr| {
            let x = f32_io::read(mem_ptr);
            let y = f32_io::read(mem_ptr + f32_io::size(0));
            let z = f32_io::read(mem_ptr + f32_io::size(0) * 2);
            Pos3d32 {x: x, y: y, z: z}
        }
    }, {
        |val: &Pos3d32, mem_ptr| {
            f32_io::write(&val.x, mem_ptr);
            f32_io::write(&val.y, mem_ptr + f32_io::size(0));
            f32_io::write(&val.z, mem_ptr + f32_io::size(0) * 2);
        }
    }, {
        f32_io::size(0) * 3
    }, {
        |_| [0u8; 8]
    }, {
        |val: &Pos3d32| {
            use std::hash::Hasher;
            let mut hasher = twox_hash::XxHash::default();
            hasher.write(&f32_io::feature(&val.x));
            hasher.write(&f32_io::feature(&val.y));
            hasher.write(&f32_io::feature(&val.z));
            u64_io::feature(&hasher.finish())
        }
    };

    Pos3d64, pos3d64_io, {
        |mem_ptr| {
            let x = f64_io::read(mem_ptr);
            let y = f64_io::read(mem_ptr + f64_io::size(0));
            let z = f64_io::read(mem_ptr + f64_io::size(0) * 2);
            Pos3d64 {x: x, y: y, z: z}
        }
    }, {
        |val: &Pos3d64, mem_ptr| {
            f64_io::write(&val.x, mem_ptr);
            f64_io::write(&val.y, mem_ptr + f64_io::size(0));
            f64_io::write(&val.z, mem_ptr + f64_io::size(0) * 2);
        }
    }, {
        f64_io::size(0) * 3
    }, {
        |_| [0u8; 8]
    }, {
        |val: &Pos3d64| {
            use std::hash::Hasher;
            let mut hasher = twox_hash::XxHash::default();
            hasher.write(&f64_io::feature(&val.x));
            hasher.write(&f64_io::feature(&val.y));
            hasher.write(&f64_io::feature(&val.z));
            u64_io::feature(&hasher.finish())
        }
    };

    //////////////////////////////////////////////////////////

    Id, id_io, {
        |mem_ptr| {
            let higher = u64_io::read(mem_ptr);
            let lower =  u64_io::read(mem_ptr + u64_io::size(0));
            Id {higher: higher, lower: lower}
        }
    }, {
        |val: &Id, mem_ptr| {
            u64_io::write(&val.higher, mem_ptr);
            u64_io::write(&val.lower,  mem_ptr + u64_io::size(0));
        }
    }, {
        u64_io::size(0) * 2
    }, {
        |id: &Id| {
            let big_end = big_end!(write_u64);
            big_end(id.higher ^ id.lower)
        }
    }, {
        |val: &Id| {
            use std::hash::Hasher;
            let mut hasher = twox_hash::XxHash::default();
            hasher.write(&u64_io::feature(&val.higher));
            hasher.write(&u64_io::feature(&val.lower));
            u64_io::feature(&hasher.finish())
        }
    }
);

gen_variable_types_io!(
    String,
    string_io,
    {
        use std::ptr;
        |mem_ptr| {
            let len = u32_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + u32_io::size(0);
            let mut bytes = Vec::with_capacity(len);
            for i in 0..len {
                let ptr = smem_ptr + i;
                let b = unsafe { ptr::read(ptr as *mut u8) };
                bytes.push(b);
            }
            String::from_utf8(bytes).unwrap()
        }
    },
    {
        use std::ptr;
        |val: &String, mem_ptr| {
            let bytes = val.as_bytes();
            let len = bytes.len();
            u32_io::write(&(len as u32), mem_ptr);
            let mut smem_ptr = mem_ptr + u32_io::size(0);
            unsafe {
                for b in bytes {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    {
        |mem_ptr| {
            let str_len = u32_io::read(mem_ptr) as usize;
            str_len + u32_io::size(0)
        }
    },
    |val: &String| { val.as_bytes().len() + u32_io::size(0) },
    |val: &String| {
        let bytes = val.as_bytes();
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), bytes.len()) { r[i] = bytes[i] }
        r
    },
    |val: &String| {
        u64_io::feature(&::bifrost_hasher::hash_str(val))
    }
);

gen_variable_types_io!(
    Bytes,
    bytes_io,
    {
        use std::ptr;
        |mem_ptr| {
            let len = u32_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + u32_io::size(0);
            let mut bytes = Vec::with_capacity(len);
            for i in 0..len {
                let ptr = smem_ptr + i;
                let b = unsafe { ptr::read(ptr as *mut u8) };
                bytes.push(b);
            }
            Bytes::from_vec(bytes)
        }
    },
    {
        use std::ptr;
        |val: &Bytes, mem_ptr| {
            let bytes = &val.data;
            let len = bytes.len();
            u32_io::write(&(len as u32), mem_ptr);
            let mut smem_ptr = mem_ptr + u32_io::size(0);
            unsafe {
                for b in bytes {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    |mem_ptr| { u32_io::read(mem_ptr) as usize + u32_io::size(0) },
    |val: &Bytes| { val.len() + u32_io::size(0) },
    |val: &Bytes| {
        let bytes = &val.data;
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), bytes.len()) { r[i] = bytes[i] }
        r
    },
    |val: &Bytes| {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash::default();
        hasher.write(&val.data);
        u64_io::feature(&hasher.finish())
    }
);

gen_variable_types_io!(
    SmallBytes,
    small_bytes_io,
    {
        use std::ptr;
        |mem_ptr| {
            let len = u8_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + 1;
            let mut bytes = Vec::with_capacity(len);
            for i in 0..len {
                let ptr = smem_ptr + i;
                let b = unsafe { ptr::read(ptr as *mut u8) };
                bytes.push(b);
            }
            SmallBytes::from_vec(bytes)
        }
    },
    {
        use std::ptr;
        |val: &SmallBytes, mem_ptr| {
            let bytes = &val.data;
            let len = bytes.len();
            u8_io::write(&(len as u8), mem_ptr);
            let mut smem_ptr = mem_ptr + 1;
            unsafe {
                for b in bytes {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    |mem_ptr| { u8_io::read(mem_ptr) as usize + 1 },
    |val: &SmallBytes| { val.len() + 1 },
    |val: &SmallBytes| {
        let bytes = &val.data;
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), bytes.len()) { r[i] = bytes[i] }
        r
    },
    |val: &SmallBytes| {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash::default();
        hasher.write(&val.data);
        u64_io::feature(&hasher.finish())
    }
);

gen_variable_types_io!(
    Any,
    any_io,
    {
        use std::ptr;
        |mem_ptr| {
            let len = u32_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + u32_io::size(0);
            let mut bytes = Vec::with_capacity(len);
            for i in 0..len {
                let ptr = smem_ptr + i;
                let b = unsafe { ptr::read(ptr as *mut u8) };
                bytes.push(b);
            }
            Any { data: bytes }
        }
    },
    {
        use std::ptr;
        |val: &Any, mem_ptr| {
            let bytes = &val.data;
            let len = bytes.len();
            u32_io::write(&(len as u32), mem_ptr);
            let mut smem_ptr = mem_ptr + u32_io::size(0);
            unsafe {
                for b in bytes {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    {
        |mem_ptr| {
            let str_len = u32_io::read(mem_ptr) as usize;
            str_len + u32_io::size(0)
        }
    },
    |val: &Any| { val.data.len() + u32_io::size(0) },
    |val: &Any| {
        let bytes = &val.data;
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), bytes.len()) { r[i] = bytes[i] }
        r
    },
    |val: &Any| {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash::default();
        hasher.write(&val.data);
        u64_io::feature(&hasher.finish())
    }
);

define_types!(
    ["bool", "bit"], 1, bool                           ,Bool     ,false ,  bool_io       ;
    ["char"], 2, char                                  ,Char     ,false ,  char_io       ;
    ["i8"], 3, i8                                      ,I8       ,false ,  i8_io         ;
    ["i16", "int"], 4, i16                             ,I16      ,false ,  i16_io        ;
    ["i32", "long"], 5, i32                            ,I32      ,false ,  i32_io        ;
    ["i64", "longlong"], 6, i64                        ,I64      ,false ,  i64_io        ;
    ["u8", "byte"], 7, u8                              ,U8       ,false ,  u8_io         ;
    ["u16"], 8, u16                                    ,U16      ,false ,  u16_io        ;
    ["u32"], 9, u32                                    ,U32      ,false ,  u32_io        ;
    ["u64"], 10, u64                                   ,U64      ,false ,  u64_io        ;
//    ["isize"], 11, isize                               ,Isize    ,false ,  isize_io      ;
//    ["usize"], 12, usize                               ,Usize    ,false ,  usize_io      ;
    ["f32", "float"], 13, f32                          ,F32      ,false ,  f32_io        ;
    ["f64", "double"], 14, f64                         ,F64      ,false ,  f64_io        ;
    ["pos2d32", "pos2d", "pos", "pos32"], 15, Pos2d32  ,Pos2d32  ,true  ,  pos2d32_io    ;
    ["pos2d64", "pos64"], 16, Pos2d64                  ,Pos2d64  ,true  ,  pos2d64_io    ;
    ["pos3d32", "pos3d"], 17, Pos3d32                  ,Pos3d32  ,true  ,  pos3d32_io    ;
    ["pos3d64"], 18, Pos3d64                           ,Pos3d64  ,true  ,  pos3d64_io    ;
    ["id"], 19, Id                                     ,Id       ,true  ,  id_io         ;
    ["string", "str"], 20, String                      ,String   ,true  ,  string_io     ;
    ["any", "dynamic"], 21, Any                        ,Any      ,true  ,  any_io        ;
    ["bytes"], 22, Bytes                               ,Bytes         ,true  ,  bytes_io            ;
    ["small_bytes"], 23, SmallBytes                    ,SmallBytes    ,true  ,  small_bytes_io
);

#[macro_export]
macro_rules! data_map {
    ($($k:ident: $v:expr),*) => {{
            let mut map = Map::new();
            $(map.insert_value(stringify!($k), $v);)*
            map
     }};
}

#[macro_export]
macro_rules! data_map_value {
    ($($k:ident: $v:expr),*) => {{
            Value::Map(data_map!($($k: $v),*))
     }};
}

pub fn type_id_of(t: Type) -> u32 {
    return t as u32;
}

pub static NULL_VALUE: Value = Value::Null;
pub const ARRAY_LEN_TYPE_ID: u32 = 9; //u32
pub const NULL_TYPE_ID: u32 = 7; //u8

pub trait Prim {}
