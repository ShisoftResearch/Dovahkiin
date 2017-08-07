use super::*;

pub fn u64(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::U64(num as u64)),
        Value::U16(num) => Ok(Value::U64(num as u64)),
        Value::U32(num) => Ok(Value::U64(num as u64)),
        Value::U64(num) => Ok(Value::U64(num as u64)),
        Value::I8(num) =>  Ok(Value::U64(num as u64)),
        Value::I16(num) => Ok(Value::U64(num as u64)),
        Value::I32(num) => Ok(Value::U64(num as u64)),
        Value::I64(num) => Ok(Value::U64(num as u64)),
        Value::F32(num) => Ok(Value::U64(num as u64)),
        Value::F64(num) => Ok(Value::U64(num as u64)),
        _ => {Err("The value cannot be convert into u64".to_string())}
    }
}

pub fn u32(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::U32(num as u32)),
        Value::U16(num) => Ok(Value::U32(num as u32)),
        Value::U32(num) => Ok(Value::U32(num as u32)),
        Value::U64(num) => Ok(Value::U32(num as u32)),
        Value::I8(num) =>  Ok(Value::U32(num as u32)),
        Value::I16(num) => Ok(Value::U32(num as u32)),
        Value::I32(num) => Ok(Value::U32(num as u32)),
        Value::I64(num) => Ok(Value::U32(num as u32)),
        Value::F32(num) => Ok(Value::U32(num as u32)),
        Value::F64(num) => Ok(Value::U32(num as u32)),
        _ => {Err("The value cannot be convert into u32".to_string())}
    }
}

pub fn u16(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::U16(num as u16)),
        Value::U16(num) => Ok(Value::U16(num as u16)),
        Value::U32(num) => Ok(Value::U16(num as u16)),
        Value::U64(num) => Ok(Value::U16(num as u16)),
        Value::I8(num) =>  Ok(Value::U16(num as u16)),
        Value::I16(num) => Ok(Value::U16(num as u16)),
        Value::I32(num) => Ok(Value::U16(num as u16)),
        Value::I64(num) => Ok(Value::U16(num as u16)),
        Value::F32(num) => Ok(Value::U16(num as u16)),
        Value::F64(num) => Ok(Value::U16(num as u16)),
        _ => {Err("The value cannot be convert into u16".to_string())}
    }
}

pub fn u8(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::U8(num as u8)),
        Value::U16(num) => Ok(Value::U8(num as u8)),
        Value::U32(num) => Ok(Value::U8(num as u8)),
        Value::U64(num) => Ok(Value::U8(num as u8)),
        Value::I8(num) =>  Ok(Value::U8(num as u8)),
        Value::I16(num) => Ok(Value::U8(num as u8)),
        Value::I32(num) => Ok(Value::U8(num as u8)),
        Value::I64(num) => Ok(Value::U8(num as u8)),
        Value::F32(num) => Ok(Value::U8(num as u8)),
        Value::F64(num) => Ok(Value::U8(num as u8)),
        _ => {Err("The value cannot be convert into u8".to_string())}
    }
}

pub fn f32(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::F32(num as f32)),
        Value::U16(num) => Ok(Value::F32(num as f32)),
        Value::U32(num) => Ok(Value::F32(num as f32)),
        Value::U64(num) => Ok(Value::F32(num as f32)),
        Value::I8(num) =>  Ok(Value::F32(num as f32)),
        Value::I16(num) => Ok(Value::F32(num as f32)),
        Value::I32(num) => Ok(Value::F32(num as f32)),
        Value::I64(num) => Ok(Value::F32(num as f32)),
        Value::F32(num) => Ok(Value::F32(num as f32)),
        Value::F64(num) => Ok(Value::F32(num as f32)),
        _ => {Err("The value cannot be convert into f32".to_string())}
    }
}

pub fn f64(value: Value) -> Result<Value, String> {
    match value {
        Value::U8(num) =>  Ok(Value::F64(num as f64)),
        Value::U16(num) => Ok(Value::F64(num as f64)),
        Value::U32(num) => Ok(Value::F64(num as f64)),
        Value::U64(num) => Ok(Value::F64(num as f64)),
        Value::I8(num) =>  Ok(Value::F64(num as f64)),
        Value::I16(num) => Ok(Value::F64(num as f64)),
        Value::I32(num) => Ok(Value::F64(num as f64)),
        Value::I64(num) => Ok(Value::F64(num as f64)),
        Value::F32(num) => Ok(Value::F64(num as f64)),
        Value::F64(num) => Ok(Value::F64(num as f64)),
        _ => {Err("The value cannot be convert into f64".to_string())}
    }
}