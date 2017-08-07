use types::Value;

macro_rules! reduce {
    ($type: path, $values: ident, $exp: expr) => (
        {
            if let Some((first, elements)) = $values.split_first() {
                if let &$type(n) = first {
                    let mut result = n;
                    for val in elements {
                        if let &$type(n) = val {
                            result = $exp(result, n);
                        } else {
                            return Err(format!("Type not match, expect {} found {:?}", stringify!($type), val));
                        }
                    }
                    Ok($type(result))
                } else {
                    Err(format!("Type not match on the first value, expect {} found {:?}", stringify!($type), first))
                }
            } else {
                Err("Cannot do reduce on values".to_string())
            }
        }
    )
}

macro_rules! add {
    ($type: ident, $values: ident) => ({
        reduce!(Value::$type, $values, |result, n| {result + n})
    })
}

macro_rules! subtract {
    ($type: ident, $values: ident) => ({
        reduce!(Value::$type, $values, |result, n| {result - n})
    })
}

macro_rules! multiply {
    ($type: ident, $values: ident) => ({
        reduce!(Value::$type, $values, |result, n| {result * n})
    })
}

macro_rules! divide {
    ($type: ident, $values: ident) => ({
        reduce!(Value::$type, $values, |result, n| {result / n})
    })
}

pub fn add(values: Vec<Value>) -> Result<Value, String> {
    match values.get(0).unwrap() {
        &Value::U8(_) =>  add!(U8,  values),
        &Value::U16(_) => add!(U16, values),
        &Value::U32(_) => add!(U32, values),
        &Value::U64(_) => add!(U64, values),
        &Value::F32(_) => add!(F32, values),
        &Value::F64(_) => add!(F64, values),
        _ => {
            Err("Type cannot be added".to_string())
        }
    }
}

pub fn subtract(values: Vec<Value>) -> Result<Value, String> {
    match values.get(0).unwrap() {
        &Value::U8(_) =>  subtract!(U8,  values),
        &Value::U16(_) => subtract!(U16, values),
        &Value::U32(_) => subtract!(U32, values),
        &Value::U64(_) => subtract!(U64, values),
        &Value::F32(_) => subtract!(F32, values),
        &Value::F64(_) => subtract!(F64, values),
        _ => {
            Err("Type cannot be subtracted".to_string())
        }
    }
}

pub fn multiply(values: Vec<Value>) -> Result<Value, String> {
    match values.get(0).unwrap() {
        &Value::U8(_) =>  multiply!(U8,  values),
        &Value::U16(_) => multiply!(U16, values),
        &Value::U32(_) => multiply!(U32, values),
        &Value::U64(_) => multiply!(U64, values),
        &Value::F32(_) => multiply!(F32, values),
        &Value::F64(_) => multiply!(F64, values),
        _ => {
            Err("Type cannot be multiplied".to_string())
        }
    }
}

pub fn divide(values: Vec<Value>) -> Result<Value, String> {
    match values.get(0).unwrap() {
        &Value::U8(_) =>  divide!(U8,  values),
        &Value::U16(_) => divide!(U16, values),
        &Value::U32(_) => divide!(U32, values),
        &Value::U64(_) => divide!(U64, values),
        &Value::F32(_) => divide!(F32, values),
        &Value::F64(_) => divide!(F64, values),
        _ => {
            Err("Type cannot be divided".to_string())
        }
    }
}