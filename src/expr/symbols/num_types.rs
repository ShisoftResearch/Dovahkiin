use super::*;

pub fn u64(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::U64(num as u64))),
        _ => Err("The value cannot be convert into u64".to_string()),
    }
}

pub fn u32(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::U32(num as u32))),
        _ => Err("The value cannot be convert into u32".to_string()),
    }
}

pub fn u16(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::U16(num as u16))),
        _ => Err("The value cannot be convert into u16".to_string()),
    }
}

pub fn u8(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::U8(num as u8))),
        _ => Err("The value cannot be convert into u8".to_string()),
    }
}

pub fn i64(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::I64(num as i64))),
        _ => Err("The value cannot be convert into i64".to_string()),
    }
}

pub fn i32(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::I32(num as i32))),
        _ => Err("The value cannot be convert into i32".to_string()),
    }
}

pub fn i16(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::I16(num as i16))),
        _ => Err("The value cannot be convert into i16".to_string()),
    }
}

pub fn i8(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::I8(num as i8))),
        _ => Err("The value cannot be convert into i8".to_string()),
    }
}

pub fn f32(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::F32(num as f32))),
        _ => Err("The value cannot be convert into f32".to_string()),
    }
}

pub fn f64(value: SExpr) -> Result<SExpr, String> {
    match value {
        SExpr::Value(Value::U8(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::U16(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::U32(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::U64(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::I8(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::I16(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::I32(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::I64(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::F32(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        SExpr::Value(Value::F64(num)) => Ok(SExpr::Value(Value::F64(num as f64))),
        _ => Err("The value cannot be convert into f64".to_string()),
    }
}
