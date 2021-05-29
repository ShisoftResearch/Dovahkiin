#[macro_use]
mod macros;
pub mod custom_types;
pub mod owned_value;

use log::{debug, info, trace, warn};
use serde::Deserialize;
use std::{ops::Index, vec::IntoIter};
pub use types::custom_types::any::*;
pub use types::custom_types::bytes::*;
pub use types::custom_types::id::*;
pub use types::custom_types::owned_map::*;
pub use types::custom_types::pos::*;
pub use types::custom_types::shared_map::*;
pub use types::owned_value::*;

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
    f32:    f32_io        big_end!(write_f32);
    f64:    f64_io        big_end!(write_f64)
);

gen_compound_types_io! (
    Pos2d32, pos2d32_io,
    {
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

    Pos2d64, pos2d64_io,
    {
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

    Pos3d32, pos3d32_io,
    {
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

    Pos3d64, pos3d64_io,
    {
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

    Id, id_io,
    {
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
    str,
    string_io,
    {
        |mem_ptr| {
            let len = *u32_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + u32_io::type_size();
            let slice = unsafe { std::slice::from_raw_parts(smem_ptr as *const u8, len) };
            std::str::from_utf8(slice).unwrap()
        }
    },
    {
        use std::ptr;
        |val: &str, mem_ptr| {
            let bytes = val.as_bytes();
            let len = bytes.len();
            u32_io::write(&(len as u32), mem_ptr);
            let mut smem_ptr = mem_ptr + u32_io::type_size();
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
            let str_len = *u32_io::read(mem_ptr) as usize;
            str_len + u32_io::type_size()
        }
    },
    |val: &str| { val.as_bytes().len() + u32_io::type_size() },
    |val: &str| {
        let bytes = val.as_bytes();
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), bytes.len()) {
            r[i] = bytes[i]
        }
        r
    },
    |val: &str| { u64_io::feature(&::bifrost_hasher::hash_str(val)) }
);

gen_variable_types_io!(
    Bytes,
    [u8],
    bytes_io,
    {
        |mem_ptr| {
            let len = *u32_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + u32_io::type_size();
            unsafe { std::slice::from_raw_parts(smem_ptr as *const u8, len) }
        }
    },
    {
        use std::ptr;
        |val: &[u8], mem_ptr| {
            let len = val.len();
            u32_io::write(&(len as u32), mem_ptr);
            let mut smem_ptr = mem_ptr + u32_io::type_size();
            unsafe {
                for b in val {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    |mem_ptr| { *u32_io::read(mem_ptr) as usize + u32_io::type_size() },
    |val: &[u8]| { val.len() + u32_io::type_size() },
    |val: &[u8]| {
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), val.len()) {
            r[i] = val[i]
        }
        r
    },
    |val: &[u8]| {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash::default();
        hasher.write(val);
        u64_io::feature(&hasher.finish())
    }
);

gen_variable_types_io!(
    SmallBytes,
    [u8],
    small_bytes_io,
    {
        |mem_ptr| {
            let len = *u8_io::read(mem_ptr) as usize;
            let smem_ptr = mem_ptr + 1;
            unsafe { std::slice::from_raw_parts(smem_ptr as *const u8, len) }
        }
    },
    {
        use std::ptr;
        |val: &[u8], mem_ptr| {
            let len = val.len() as u8;
            u8_io::write(&len, mem_ptr);
            let mut smem_ptr = mem_ptr + 1;
            unsafe {
                for b in val {
                    ptr::write(smem_ptr as *mut u8, *b);
                    smem_ptr += 1;
                }
            }
        }
    },
    |mem_ptr| { *u8_io::read(mem_ptr) as usize + 1 },
    |val: &[u8]| { val.len() + 1 },
    |val: &[u8]| {
        let mut r = [0u8; 8];
        for i in 0..::std::cmp::min(r.len(), val.len()) {
            r[i] = val[i]
        }
        r
    },
    |val: &[u8]| {
        use std::hash::Hasher;
        let mut hasher = twox_hash::XxHash::default();
        hasher.write(&val);
        u64_io::feature(&hasher.finish())
    }
);

define_types!(
    ["bool", "bit"], bool                          ,Bool        ,  bool_io, bool       ;
    ["char"], char                                 ,Char        ,  char_io, char       ;
    ["i8"], i8                                     ,I8          ,  i8_io, i8          ;
    ["i16", "int"], i16                            ,I16         ,  i16_io, i16        ;
    ["i32", "long"], i32                           ,I32         ,  i32_io, i32        ;
    ["i64", "longlong"], i64                       ,I64         ,  i64_io, i64        ;
    ["u8", "byte"], u8                             ,U8          ,  u8_io, u8         ;
    ["u16"], u16                                   ,U16         ,  u16_io, u16        ;
    ["u32"], u32                                   ,U32         ,  u32_io, u32        ;
    ["u64"], u64                                   ,U64         ,  u64_io, u64        ;
    ["f32", "float"], f32                          ,F32         ,  f32_io, f32        ;
    ["f64", "double"], f64                         ,F64         ,  f64_io, f64        ;
    ["pos2d32", "pos2d", "pos", "pos32"], Pos2d32  ,Pos2d32     ,  pos2d32_io, pos2d32    ;
    ["pos2d64", "pos64"], Pos2d64                  ,Pos2d64     ,  pos2d64_io, pos2d64    ;
    ["pos3d32", "pos3d"], Pos3d32                  ,Pos3d32     ,  pos3d32_io, pos3d32    ;
    ["pos3d64"], Pos3d64                           ,Pos3d64     ,  pos3d64_io, pos3d64    ;
    ["id"], Id                                     ,Id          ,  id_io, id         ;
    ["string", "str"], String                      ,String      ,  string_io, string     ;
    ["bytes"], Bytes                               ,Bytes       ,  bytes_io, bytes      ;
    ["small_bytes"], SmallBytes                    ,SmallBytes  ,  small_bytes_io, small_bytes
);

#[macro_export]
macro_rules! data_map {
    ($($k:ident: $v:expr),*) => {{
            let mut map = $crate::types::OwnedMap::new();
            $(map.insert_value(stringify!($k), $v);)*
            map
     }};
}

#[macro_export]
macro_rules! data_map_value {
    ($($k:ident: $v:expr),*) => {{
        $crate::types::OwnedValue::Map(data_map!($($k: $v),*))
     }};
}

pub fn type_id_of(t: Type) -> u32 {
    return t as u32;
}

pub static NULL_OWNED_VALUE: OwnedValue = OwnedValue::Null;
pub static NULL_SHARED_VALUE: SharedValue = SharedValue::Null;
pub const ARRAY_LEN_TYPE: Type = Type::U32; //u32
pub const TYPE_CODE_TYPE: Type = Type::U8; //u32

impl<'a> Index<&'a str> for SharedValue {
    type Output = Self;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            &Self::Map(ref map) => map.get(index),
            _ => &NULL_SHARED_VALUE,
        }
    }
}
