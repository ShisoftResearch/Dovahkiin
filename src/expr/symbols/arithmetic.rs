use super::*;

macro_rules! reduce {
    ($type: ident, $values: ident, $exp: expr) => {{
        if let Some((Some(first), elements)) =
            $values.split_first().map(|(f, es)| (f.shared_val(), es))
        {
            if let SharedValue::$type(n) = first {
                let mut result = *n;
                for val in elements {
                    let val = val.shared_val();
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
                Ok(SExpr::from_owned_value(OwnedValue::$type(result)))
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
        return reduce!($type, $values, |result, n| { result + n });
    }};
}

macro_rules! subtract_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result - n });
    }};
}

macro_rules! multiply_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result * n });
    }};
}

macro_rules! divide_ {
    ($type: ident, $values: ident) => {{
        return reduce!($type, $values, |result, n| { result / n });
    }};
}

pub fn add(values: Vec<SExpr>) -> Result<SExpr, String> {
    match values.get(0).unwrap().shared_val() {
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
    match values.get(0).unwrap().shared_val() {
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
    match values.get(0).unwrap().shared_val() {
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
    match values.get(0).unwrap().shared_val() {
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
    let value = match value.shared_val() {
        Some(SharedValue::U8(v)) => SExpr::from_owned_value(OwnedValue::U8(v + 1)),
        Some(SharedValue::U16(v)) => SExpr::from_owned_value(OwnedValue::U16(v + 1)),
        Some(SharedValue::U32(v)) => SExpr::from_owned_value(OwnedValue::U32(v + 1)),
        Some(SharedValue::U64(v)) => SExpr::from_owned_value(OwnedValue::U64(v + 1)),
        Some(SharedValue::I8(v)) => SExpr::from_owned_value(OwnedValue::I8(v + 1)),
        Some(SharedValue::I16(v)) => SExpr::from_owned_value(OwnedValue::I16(v + 1)),
        Some(SharedValue::I32(v)) => SExpr::from_owned_value(OwnedValue::I32(v + 1)),
        Some(SharedValue::I64(v)) => SExpr::from_owned_value(OwnedValue::I64(v + 1)),
        _ => return Err(format!("Type cannot be increased: {:?}", value)),
    };
    Ok(value)
}

macro_rules! unary_float_fn {
    ($name: ident, $op32: expr, $op64: expr) => {
        pub fn $name(value: SExpr) -> Result<SExpr, String> {
            match value.shared_val() {
                Some(SharedValue::F32(v)) => {
                    Ok(SExpr::from_owned_value(OwnedValue::F32($op32(*v))))
                }
                Some(SharedValue::F64(v)) => {
                    Ok(SExpr::from_owned_value(OwnedValue::F64($op64(*v))))
                }
                _ => Err(format!(
                    "{} expects f32 or f64, found {:?}",
                    stringify!($name),
                    value
                )),
            }
        }
    };
}

pub fn abs(value: SExpr) -> Result<SExpr, String> {
    match value.shared_val() {
        Some(SharedValue::U8(v)) => Ok(SExpr::from_owned_value(OwnedValue::U8(*v))),
        Some(SharedValue::U16(v)) => Ok(SExpr::from_owned_value(OwnedValue::U16(*v))),
        Some(SharedValue::U32(v)) => Ok(SExpr::from_owned_value(OwnedValue::U32(*v))),
        Some(SharedValue::U64(v)) => Ok(SExpr::from_owned_value(OwnedValue::U64(*v))),
        Some(SharedValue::I8(v)) => Ok(SExpr::from_owned_value(OwnedValue::I8(v.abs()))),
        Some(SharedValue::I16(v)) => Ok(SExpr::from_owned_value(OwnedValue::I16(v.abs()))),
        Some(SharedValue::I32(v)) => Ok(SExpr::from_owned_value(OwnedValue::I32(v.abs()))),
        Some(SharedValue::I64(v)) => Ok(SExpr::from_owned_value(OwnedValue::I64(v.abs()))),
        Some(SharedValue::F32(v)) => Ok(SExpr::from_owned_value(OwnedValue::F32(v.abs()))),
        Some(SharedValue::F64(v)) => Ok(SExpr::from_owned_value(OwnedValue::F64(v.abs()))),
        _ => Err(format!("abs expects a numeric value, found {:?}", value)),
    }
}

unary_float_fn!(sqrt, |v: f32| v.sqrt(), |v: f64| v.sqrt());
unary_float_fn!(ln, |v: f32| v.ln(), |v: f64| v.ln());
unary_float_fn!(log2, |v: f32| v.log2(), |v: f64| v.log2());
unary_float_fn!(log10, |v: f32| v.log10(), |v: f64| v.log10());
unary_float_fn!(exp, |v: f32| v.exp(), |v: f64| v.exp());
unary_float_fn!(floor, |v: f32| v.floor(), |v: f64| v.floor());
unary_float_fn!(ceil, |v: f32| v.ceil(), |v: f64| v.ceil());
unary_float_fn!(round, |v: f32| v.round(), |v: f64| v.round());

pub fn pow<'a>(base: SExpr<'a>, exp: SExpr<'a>) -> Result<SExpr<'a>, String> {
    match (base.shared_val(), exp.shared_val()) {
        (Some(SharedValue::F32(base)), Some(SharedValue::F32(exp))) => {
            Ok(SExpr::from_owned_value(OwnedValue::F32(base.powf(*exp))))
        }
        (Some(SharedValue::F64(base)), Some(SharedValue::F64(exp))) => {
            Ok(SExpr::from_owned_value(OwnedValue::F64(base.powf(*exp))))
        }
        (Some(SharedValue::F32(base)), Some(SharedValue::U32(exp))) => Ok(SExpr::from_owned_value(
            OwnedValue::F32(base.powi(*exp as i32)),
        )),
        (Some(SharedValue::F64(base)), Some(SharedValue::U32(exp))) => Ok(SExpr::from_owned_value(
            OwnedValue::F64(base.powi(*exp as i32)),
        )),
        _ => Err(format!(
            "pow expects (f32|f64, f32|f64|u32), found ({:?}, {:?})",
            base, exp
        )),
    }
}

macro_rules! binary_ord_fn {
    ($name: ident, $cmp: tt) => {
        pub fn $name<'a>(lhs: SExpr<'a>, rhs: SExpr<'a>) -> Result<SExpr<'a>, String> {
            match (lhs.shared_val(), rhs.shared_val()) {
                (Some(SharedValue::U8(lhs)), Some(SharedValue::U8(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::U8(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::U16(lhs)), Some(SharedValue::U16(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::U16(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::U32(lhs)), Some(SharedValue::U32(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::U32(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::U64(lhs)), Some(SharedValue::U64(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::U64(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::I8(lhs)), Some(SharedValue::I8(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::I8(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::I16(lhs)), Some(SharedValue::I16(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::I16(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::I32(lhs)), Some(SharedValue::I32(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::I32(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::I64(lhs)), Some(SharedValue::I64(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::I64(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::F32(lhs)), Some(SharedValue::F32(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::F32(if lhs $cmp rhs { *lhs } else { *rhs }))),
                (Some(SharedValue::F64(lhs)), Some(SharedValue::F64(rhs))) => Ok(SExpr::from_owned_value(OwnedValue::F64(if lhs $cmp rhs { *lhs } else { *rhs }))),
                _ => Err(format!(
                    "{} expects two numeric values of the same type, found ({:?}, {:?})",
                    stringify!($name),
                    lhs,
                    rhs
                )),
            }
        }
    };
}

binary_ord_fn!(min, <=);
binary_ord_fn!(max, >=);

pub fn clamp<'a>(value: SExpr<'a>, min: SExpr<'a>, max: SExpr<'a>) -> Result<SExpr<'a>, String> {
    match (value.shared_val(), min.shared_val(), max.shared_val()) {
        (Some(SharedValue::U8(value)), Some(SharedValue::U8(min)), Some(SharedValue::U8(max))) => {
            Ok(SExpr::from_owned_value(OwnedValue::U8(
                (*value).clamp(*min, *max),
            )))
        }
        (
            Some(SharedValue::U16(value)),
            Some(SharedValue::U16(min)),
            Some(SharedValue::U16(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::U16(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::U32(value)),
            Some(SharedValue::U32(min)),
            Some(SharedValue::U32(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::U32(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::U64(value)),
            Some(SharedValue::U64(min)),
            Some(SharedValue::U64(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::U64(
            (*value).clamp(*min, *max),
        ))),
        (Some(SharedValue::I8(value)), Some(SharedValue::I8(min)), Some(SharedValue::I8(max))) => {
            Ok(SExpr::from_owned_value(OwnedValue::I8(
                (*value).clamp(*min, *max),
            )))
        }
        (
            Some(SharedValue::I16(value)),
            Some(SharedValue::I16(min)),
            Some(SharedValue::I16(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::I16(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::I32(value)),
            Some(SharedValue::I32(min)),
            Some(SharedValue::I32(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::I32(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::I64(value)),
            Some(SharedValue::I64(min)),
            Some(SharedValue::I64(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::I64(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::F32(value)),
            Some(SharedValue::F32(min)),
            Some(SharedValue::F32(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::F32(
            (*value).clamp(*min, *max),
        ))),
        (
            Some(SharedValue::F64(value)),
            Some(SharedValue::F64(min)),
            Some(SharedValue::F64(max)),
        ) => Ok(SExpr::from_owned_value(OwnedValue::F64(
            (*value).clamp(*min, *max),
        ))),
        _ => Err(format!(
            "clamp expects three numeric values of the same type, found ({:?}, {:?}, {:?})",
            value, min, max
        )),
    }
}
