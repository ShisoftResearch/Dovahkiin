use super::*;
use types::Value;

pub fn equals(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let last = exprs.pop().unwrap();
    for expr in exprs {
        if expr != last {
            return Ok(SExpr::Value(Value::Bool(false)));
        }
    }
    return Ok(SExpr::Value(Value::Bool(true)));
}

pub fn not_equals(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    return Ok(SExpr::Value(Value::Bool(exprs.pop() == exprs.pop())));
}

macro_rules! reduce {
    ($type: path, $values: ident, $exp: expr) => {{
        if let Some((first, elements)) = $values.split_first() {
            if let &SExpr::Value($type(first)) = first {
                let mut last = first;
                for val in elements {
                    if let &SExpr::Value($type(n)) = val {
                        if $exp(last, n) {
                            last = n;
                        } else {
                            return Ok(SExpr::Value(Value::Bool(false)));
                        }
                    } else {
                        return Err(format!(
                            "Type not match, expect {} found {:?}",
                            stringify!($type),
                            val
                        ));
                    }
                }
                Ok(SExpr::Value(Value::Bool(true)))
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

macro_rules! lt_ {
    ($type: ident, $values: ident) => {{
        reduce!(Value::$type, $values, |last, n| { last < n })
    }};
}

macro_rules! lte_ {
    ($type: ident, $values: ident) => {{
        reduce!(Value::$type, $values, |last, n| { last <= n })
    }};
}

macro_rules! gt_ {
    ($type: ident, $values: ident) => {{
        reduce!(Value::$type, $values, |last, n| { last > n })
    }};
}

macro_rules! gte_ {
    ($type: ident, $values: ident) => {{
        reduce!(Value::$type, $values, |last, n| { last >= n })
    }};
}

pub fn lt(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap() {
        &SExpr::Value(Value::U8(_)) => lt_!(U8, values),
        &SExpr::Value(Value::U16(_)) => lt_!(U16, values),
        &SExpr::Value(Value::U32(_)) => lt_!(U32, values),
        &SExpr::Value(Value::U64(_)) => lt_!(U64, values),
        &SExpr::Value(Value::I8(_)) => lt_!(I8, values),
        &SExpr::Value(Value::I16(_)) => lt_!(I16, values),
        &SExpr::Value(Value::I32(_)) => lt_!(I32, values),
        &SExpr::Value(Value::I64(_)) => lt_!(I64, values),
        &SExpr::Value(Value::F32(_)) => lt_!(F32, values),
        &SExpr::Value(Value::F64(_)) => lt_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn lte(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap() {
        &SExpr::Value(Value::U8(_)) => lte_!(U8, values),
        &SExpr::Value(Value::U16(_)) => lte_!(U16, values),
        &SExpr::Value(Value::U32(_)) => lte_!(U32, values),
        &SExpr::Value(Value::U64(_)) => lte_!(U64, values),
        &SExpr::Value(Value::I8(_)) => lte_!(I8, values),
        &SExpr::Value(Value::I16(_)) => lte_!(I16, values),
        &SExpr::Value(Value::I32(_)) => lte_!(I32, values),
        &SExpr::Value(Value::I64(_)) => lte_!(I64, values),
        &SExpr::Value(Value::F32(_)) => lte_!(F32, values),
        &SExpr::Value(Value::F64(_)) => lte_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn gt(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap() {
        &SExpr::Value(Value::U8(_)) => gt_!(U8, values),
        &SExpr::Value(Value::U16(_)) => gt_!(U16, values),
        &SExpr::Value(Value::U32(_)) => gt_!(U32, values),
        &SExpr::Value(Value::U64(_)) => gt_!(U64, values),
        &SExpr::Value(Value::I8(_)) => gt_!(I8, values),
        &SExpr::Value(Value::I16(_)) => gt_!(I16, values),
        &SExpr::Value(Value::I32(_)) => gt_!(I32, values),
        &SExpr::Value(Value::I64(_)) => gt_!(I64, values),
        &SExpr::Value(Value::F32(_)) => gt_!(F32, values),
        &SExpr::Value(Value::F64(_)) => gt_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn gte(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap() {
        &SExpr::Value(Value::U8(_)) => gte_!(U8, values),
        &SExpr::Value(Value::U16(_)) => gte_!(U16, values),
        &SExpr::Value(Value::U32(_)) => gte_!(U32, values),
        &SExpr::Value(Value::U64(_)) => gte_!(U64, values),
        &SExpr::Value(Value::I8(_)) => gte_!(I8, values),
        &SExpr::Value(Value::I16(_)) => gte_!(I16, values),
        &SExpr::Value(Value::I32(_)) => gte_!(I32, values),
        &SExpr::Value(Value::I64(_)) => gte_!(I64, values),
        &SExpr::Value(Value::F32(_)) => gte_!(F32, values),
        &SExpr::Value(Value::F64(_)) => gte_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}
