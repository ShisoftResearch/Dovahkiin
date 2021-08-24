use super::*;

macro_rules! reduce {
    ($type: ident, $values: ident, $exp: expr) => {{
        if let Some((Some(first), elements)) = $values.split_first().map(|(f, es)| (f.val(), es)) {
            if let SharedValue::$type(n) = first {
                let mut result = *n;
                for val in elements {
                    let val = val.val();
                    if let Some(SharedValue::$type(n)) = val {
                        result = $exp(result, n);
                    } else {
                        return Err(format!(
                            "Type not match, expect {} found {:?}",
                            stringify!($type),
                            val
                        ));
                    }
                }
                Ok(SExpr::owned_value(OwnedValue::$type(result)))
            } else {
                Err(format!(
                    "Type not match on the first value, expect {} found {:?}",
                    stringify!($type),
                    first
                ))
            }
        } else {
            Err("Cannot do reduce on values".to_string())
        }
    }};
}

macro_rules! add_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result + n })
    }};
}

macro_rules! subtract_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result - n })
    }};
}

macro_rules! multiply_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result * n })
    }};
}

macro_rules! divide_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result / n })
    }};
}

pub fn add(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap().val() {
        Some(SharedValue::U8(_)) => add_!(U8, values),
        Some(SharedValue::U16(_)) => add_!(U16, values),
        Some(SharedValue::U32(_)) => add_!(U32, values),
        Some(SharedValue::U64(_)) => add_!(U64, values),
        Some(SharedValue::I8(_)) => add_!(I8, values),
        Some(SharedValue::I16(_)) => add_!(I16, values),
        Some(SharedValue::I32(_)) => add_!(I32, values),
        Some(SharedValue::I64(_)) => add_!(I64, values),
        Some(SharedValue::F32(_)) => add_!(F32, values),
        Some(SharedValue::F64(_)) => add_!(F64, values),
        _ => Err(format!("Type cannot be added {:?}", values)),
    }
}

pub fn subtract(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap().val() {
        Some(SharedValue::U8(_)) => subtract_!(U8, values),
        Some(SharedValue::U16(_)) => subtract_!(U16, values),
        Some(SharedValue::U32(_)) => subtract_!(U32, values),
        Some(SharedValue::U64(_)) => subtract_!(U64, values),
        Some(SharedValue::I8(_)) => subtract_!(I8, values),
        Some(SharedValue::I16(_)) => subtract_!(I16, values),
        Some(SharedValue::I32(_)) => subtract_!(I32, values),
        Some(SharedValue::I64(_)) => subtract_!(I64, values),
        Some(SharedValue::F32(_)) => subtract_!(F32, values),
        Some(SharedValue::F64(_)) => subtract_!(F64, values),
        _ => Err(format!("Type cannot be subtracted: {:?}", values)),
    }
}

pub fn multiply(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap().val() {
        Some(SharedValue::U8(_)) => multiply_!(U8, values),
        Some(SharedValue::U16(_)) => multiply_!(U16, values),
        Some(SharedValue::U32(_)) => multiply_!(U32, values),
        Some(SharedValue::U64(_)) => multiply_!(U64, values),
        Some(SharedValue::I8(_)) => multiply_!(I8, values),
        Some(SharedValue::I16(_)) => multiply_!(I16, values),
        Some(SharedValue::I32(_)) => multiply_!(I32, values),
        Some(SharedValue::I64(_)) => multiply_!(I64, values),
        Some(SharedValue::F32(_)) => multiply_!(F32, values),
        Some(SharedValue::F64(_)) => multiply_!(F64, values),
        _ => Err(format!("Type cannot be multiplied: {:?}", values)),
    }
}

pub fn divide(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap().val() {
        Some(SharedValue::U8(_)) => divide_!(U8, values),
        Some(SharedValue::U16(_)) => divide_!(U16, values),
        Some(SharedValue::U32(_)) => divide_!(U32, values),
        Some(SharedValue::U64(_)) => divide_!(U64, values),
        Some(SharedValue::I8(_)) => divide_!(I8, values),
        Some(SharedValue::I16(_)) => divide_!(I16, values),
        Some(SharedValue::I32(_)) => divide_!(I32, values),
        Some(SharedValue::I64(_)) => divide_!(I64, values),
        Some(SharedValue::F32(_)) => divide_!(F32, values),
        Some(SharedValue::F64(_)) => divide_!(F64, values),
        _ => Err(format!("Type cannot be divided: {:?}", values)),
    }
}

pub fn inc(value: SExpr) -> Result<SExpr, String> {
    let value = match value.val() {
        Some(SharedValue::U8(v)) => SExpr::owned_value(OwnedValue::U8(v + 1)),
        Some(SharedValue::U16(v)) => SExpr::owned_value(OwnedValue::U16(v + 1)),
        Some(SharedValue::U32(v)) => SExpr::owned_value(OwnedValue::U32(v + 1)),
        Some(SharedValue::U64(v)) => SExpr::owned_value(OwnedValue::U64(v + 1)),
        Some(SharedValue::I8(v)) => SExpr::owned_value(OwnedValue::I8(v + 1)),
        Some(SharedValue::I16(v)) => SExpr::owned_value(OwnedValue::I16(v + 1)),
        Some(SharedValue::I32(v)) => SExpr::owned_value(OwnedValue::I32(v + 1)),
        Some(SharedValue::I64(v)) => SExpr::owned_value(OwnedValue::I64(v + 1)),
        _ => return Err(format!("Type cannot be increased: {:?}", value)),
    };
    Ok(value)
}
