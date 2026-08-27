use log::trace;

use super::*;

pub fn equals(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let last_expr = exprs.pop().unwrap();
    let last = last_expr.shared_val();
    let last_num = last.as_ref().and_then(super::numeric::Num::from_shared);
    for expr in exprs {
        let expr = expr.shared_val();
        trace!("Comparing {:?} with {:?}", expr, last);
        // Numbers compare by value across widths: a u16 column equals the
        // i64 literal `1`. Everything else keeps exact equality.
        let equal = match (expr.as_ref().and_then(super::numeric::Num::from_shared), last_num) {
            (Some(a), Some(b)) => a.equals(b),
            _ => expr == last,
        };
        if !equal {
            return Ok(SExpr::from_owned_value(OwnedValue::Bool(false)));
        }
    }
    return Ok(SExpr::from_owned_value(OwnedValue::Bool(true)));
}

/// Ordered comparison chain over promoted numbers; `None` if any operand is
/// not numeric so the exact-type path can take it.
fn promoted_chain<'a>(
    values: &[SExpr<'a>],
    holds: impl Fn(std::cmp::Ordering) -> bool,
) -> Option<Result<SExpr<'a>, String>> {
    let nums = values
        .iter()
        .map(|v| v.shared_val().as_ref().and_then(super::numeric::Num::from_shared))
        .collect::<Option<Vec<_>>>()?;
    for pair in nums.windows(2) {
        match pair[0].partial_cmp(pair[1]) {
            Some(ordering) if holds(ordering) => {}
            _ => return Some(Ok(SExpr::from_owned_value(OwnedValue::Bool(false)))),
        }
    }
    Some(Ok(SExpr::from_owned_value(OwnedValue::Bool(true))))
}

pub fn not_equals(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    return Ok(SExpr::from_owned_value(OwnedValue::Bool({
        let l = exprs.pop();
        let r = exprs.pop();
        l.as_ref().map(|e| e.shared_val()) == r.as_ref().map(|e| e.shared_val())
    })));
}

macro_rules! reduce {
    ($type: ident, $values: ident, $exp: expr) => {{
        if let Some((first, elements)) = $values.split_first() {
            if let Some(SharedValue::$type(first)) = first.shared_val() {
                let mut last = first;
                for val in elements {
                    if let Some(SharedValue::$type(n)) = val.shared_val() {
                        if $exp(last, n) {
                            last = n;
                        } else {
                            return Ok(SExpr::from_owned_value(OwnedValue::Bool(false)));
                        }
                    } else {
                        return Err(format!(
                            "Type not match, expect {} found {:?}",
                            stringify!($type),
                            val
                        ));
                    }
                }
                Ok(SExpr::from_owned_value(OwnedValue::Bool(true)))
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
        reduce!($type, $values, |last, n| { last < n })
    }};
}

macro_rules! lte_ {
    ($type: ident, $values: ident) => {{
        reduce!($type, $values, |last, n| { last <= n })
    }};
}

macro_rules! gt_ {
    ($type: ident, $values: ident) => {{
        reduce!($type, $values, |last, n| { last > n })
    }};
}

macro_rules! gte_ {
    ($type: ident, $values: ident) => {{
        reduce!($type, $values, |last, n| { last >= n })
    }};
}

pub fn lt(values: Vec<SExpr>) -> Result<SExpr, String> {
    {
        let shared = values.iter().map(|v| v.shared_val()).collect::<Vec<_>>();
        if super::numeric::needs_promotion(&shared) {
            if let Some(result) = promoted_chain(&values, |o| o == std::cmp::Ordering::Less) {
                return result;
            }
        }
    }
    match values.get(0).unwrap().shared_val() {
        Some(SharedValue::U8(_)) => lt_!(U8, values),
        Some(SharedValue::U16(_)) => lt_!(U16, values),
        Some(SharedValue::U32(_)) => lt_!(U32, values),
        Some(SharedValue::U64(_)) => lt_!(U64, values),
        Some(SharedValue::I8(_)) => lt_!(I8, values),
        Some(SharedValue::I16(_)) => lt_!(I16, values),
        Some(SharedValue::I32(_)) => lt_!(I32, values),
        Some(SharedValue::I64(_)) => lt_!(I64, values),
        Some(SharedValue::F32(_)) => lt_!(F32, values),
        Some(SharedValue::F64(_)) => lt_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn lte(values: Vec<SExpr>) -> Result<SExpr, String> {
    {
        let shared = values.iter().map(|v| v.shared_val()).collect::<Vec<_>>();
        if super::numeric::needs_promotion(&shared) {
            if let Some(result) = promoted_chain(&values, |o| o != std::cmp::Ordering::Greater) {
                return result;
            }
        }
    }
    match values.get(0).unwrap().shared_val() {
        Some(SharedValue::U8(_)) => lte_!(U8, values),
        Some(SharedValue::U16(_)) => lte_!(U16, values),
        Some(SharedValue::U32(_)) => lte_!(U32, values),
        Some(SharedValue::U64(_)) => lte_!(U64, values),
        Some(SharedValue::I8(_)) => lte_!(I8, values),
        Some(SharedValue::I16(_)) => lte_!(I16, values),
        Some(SharedValue::I32(_)) => lte_!(I32, values),
        Some(SharedValue::I64(_)) => lte_!(I64, values),
        Some(SharedValue::F32(_)) => lte_!(F32, values),
        Some(SharedValue::F64(_)) => lte_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn gt(values: Vec<SExpr>) -> Result<SExpr, String> {
    {
        let shared = values.iter().map(|v| v.shared_val()).collect::<Vec<_>>();
        if super::numeric::needs_promotion(&shared) {
            if let Some(result) = promoted_chain(&values, |o| o == std::cmp::Ordering::Greater) {
                return result;
            }
        }
    }
    match values.get(0).unwrap().shared_val() {
        Some(SharedValue::U8(_)) => gt_!(U8, values),
        Some(SharedValue::U16(_)) => gt_!(U16, values),
        Some(SharedValue::U32(_)) => gt_!(U32, values),
        Some(SharedValue::U64(_)) => gt_!(U64, values),
        Some(SharedValue::I8(_)) => gt_!(I8, values),
        Some(SharedValue::I16(_)) => gt_!(I16, values),
        Some(SharedValue::I32(_)) => gt_!(I32, values),
        Some(SharedValue::I64(_)) => gt_!(I64, values),
        Some(SharedValue::F32(_)) => gt_!(F32, values),
        Some(SharedValue::F64(_)) => gt_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn gte(values: Vec<SExpr>) -> Result<SExpr, String> {
    {
        let shared = values.iter().map(|v| v.shared_val()).collect::<Vec<_>>();
        if super::numeric::needs_promotion(&shared) {
            if let Some(result) = promoted_chain(&values, |o| o != std::cmp::Ordering::Less) {
                return result;
            }
        }
    }
    match values.get(0).unwrap().shared_val() {
        Some(SharedValue::U8(_)) => gte_!(U8, values),
        Some(SharedValue::U16(_)) => gte_!(U16, values),
        Some(SharedValue::U32(_)) => gte_!(U32, values),
        Some(SharedValue::U64(_)) => gte_!(U64, values),
        Some(SharedValue::I8(_)) => gte_!(I8, values),
        Some(SharedValue::I16(_)) => gte_!(I16, values),
        Some(SharedValue::I32(_)) => gte_!(I32, values),
        Some(SharedValue::I64(_)) => gte_!(I64, values),
        Some(SharedValue::F32(_)) => gte_!(F32, values),
        Some(SharedValue::F64(_)) => gte_!(F64, values),
        _ => Err(format!("Type cannot be compared: {:?}", values)),
    }
}

pub fn in_(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let needle = exprs.remove(0);
    let Some(needle) = needle.shared_val() else {
        return Ok(SExpr::from_owned_value(OwnedValue::Bool(false)));
    };

    let matched = exprs
        .into_iter()
        .any(|expr| expr.shared_val().as_ref() == Some(&needle));
    Ok(SExpr::from_owned_value(OwnedValue::Bool(matched)))
}

macro_rules! between_ {
    ($type: ident, $value: ident, $lower: ident, $upper: ident) => {{
        if let (
            Some(SharedValue::$type(value)),
            Some(SharedValue::$type(lower)),
            Some(SharedValue::$type(upper)),
        ) = (
            $value.shared_val(),
            $lower.shared_val(),
            $upper.shared_val(),
        ) {
            Ok(SExpr::from_owned_value(OwnedValue::Bool(
                lower <= value && value <= upper,
            )))
        } else {
            Err(format!(
                "Type not match, expect {} found {:?}, {:?}, {:?}",
                stringify!($type),
                $value,
                $lower,
                $upper
            ))
        }
    }};
}

pub fn between(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let value = exprs.remove(0);
    let lower = exprs.remove(0);
    let upper = exprs.remove(0);

    match value.shared_val() {
        Some(SharedValue::U8(_)) => between_!(U8, value, lower, upper),
        Some(SharedValue::U16(_)) => between_!(U16, value, lower, upper),
        Some(SharedValue::U32(_)) => between_!(U32, value, lower, upper),
        Some(SharedValue::U64(_)) => between_!(U64, value, lower, upper),
        Some(SharedValue::I8(_)) => between_!(I8, value, lower, upper),
        Some(SharedValue::I16(_)) => between_!(I16, value, lower, upper),
        Some(SharedValue::I32(_)) => between_!(I32, value, lower, upper),
        Some(SharedValue::I64(_)) => between_!(I64, value, lower, upper),
        Some(SharedValue::F32(_)) => between_!(F32, value, lower, upper),
        Some(SharedValue::F64(_)) => between_!(F64, value, lower, upper),
        _ => Err(format!(
            "Type cannot be compared: {:?}",
            vec![value, lower, upper]
        )),
    }
}
