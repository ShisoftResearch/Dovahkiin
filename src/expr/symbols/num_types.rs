use super::*;
use crate::types::f16_from_f64;

pub fn u64(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(num.to_f32() as u64))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*num as u64))),
        _ => Err("The value cannot be convert into u64".to_string()),
    }
}

pub fn u32(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(num.to_f32() as u32))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*num as u32))),
        _ => Err("The value cannot be convert into u32".to_string()),
    }
}

pub fn u16(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(num.to_f32() as u16))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*num as u16))),
        _ => Err("The value cannot be convert into u16".to_string()),
    }
}

pub fn u8(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(num.to_f32() as u8))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*num as u8))),
        _ => Err("The value cannot be convert into u8".to_string()),
    }
}

pub fn i64(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(num.to_f32() as i64))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I64(*num as i64))),
        _ => Err("The value cannot be convert into i64".to_string()),
    }
}

pub fn i32(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(num.to_f32() as i32))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I32(*num as i32))),
        _ => Err("The value cannot be convert into i32".to_string()),
    }
}

pub fn i16(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(num.to_f32() as i16))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I16(*num as i16))),
        _ => Err("The value cannot be convert into i16".to_string()),
    }
}

pub fn i8(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(num.to_f32() as i8))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::I8(*num as i8))),
        _ => Err("The value cannot be convert into i8".to_string()),
    }
}

pub fn f32(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(*num as f32))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F32(num.to_f32()))),
        _ => Err("The value cannot be convert into f32".to_string()),
    }
}

pub fn f64(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::U16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::U32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::U64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::I8(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::I16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::I32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::I64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::F32(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::F64(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(*num as f64))),
        Some(SharedValue::F16(num)) => Ok(SExpr::from_owned_value(OwnedValue::F64(num.to_f64()))),
        _ => Err("The value cannot be convert into f64".to_string()),
    }
}

/// Convert a numeric value to f16. The conversion rounds to nearest, ties to
/// even. NaN, infinity and overflow are errors.
pub fn f16(value: SExpr) -> Result<SExpr, String> {
    let num = match value.shared_val() {
        Some(SharedValue::U8(num)) => *num as f64,
        Some(SharedValue::U16(num)) => *num as f64,
        Some(SharedValue::U32(num)) => *num as f64,
        Some(SharedValue::U64(num)) => *num as f64,
        Some(SharedValue::I8(num)) => *num as f64,
        Some(SharedValue::I16(num)) => *num as f64,
        Some(SharedValue::I32(num)) => *num as f64,
        Some(SharedValue::I64(num)) => *num as f64,
        Some(SharedValue::F32(num)) => *num as f64,
        Some(SharedValue::F64(num)) => *num,
        Some(SharedValue::F16(num)) => num.to_f64(),
        _ => return Err("The value cannot be convert into f16".to_string()),
    };
    let converted = f16_from_f64(num)?;
    Ok(SExpr::from_owned_value(OwnedValue::F16(converted)))
}
