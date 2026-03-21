use super::utils::is_true;
use super::*;
use crate::types::SharedValue;

pub fn or<'a>(exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    for expr in exprs {
        if is_true(&expr.eval(env)?) {
            return Ok(SExpr::from_owned_value(OwnedValue::Bool(true)));
        }
    }
    return Ok(SExpr::from_owned_value(OwnedValue::Bool(false)));
}

pub fn and<'a>(exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    for expr in exprs {
        if !is_true(&expr.eval(env)?) {
            return Ok(SExpr::from_owned_value(OwnedValue::Bool(false)));
        }
    }
    return Ok(SExpr::from_owned_value(OwnedValue::Bool(true)));
}

pub fn not<'a>(mut exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    let value = exprs.pop().unwrap().eval(env)?;
    Ok(SExpr::from_owned_value(OwnedValue::Bool(!is_true(&value))))
}

pub fn is_null<'a>(mut exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    let expr = exprs.pop().unwrap();
    let is_null = match expr {
        SExpr::ISymbol(symbol_id, _) => matches!(
            crate::types::Value::get_by_id(env.global_val, symbol_id),
            SharedValue::Null
        ),
        other => matches!(other.eval(env)?.shared_val(), Some(SharedValue::Null)),
    };
    Ok(SExpr::from_owned_value(OwnedValue::Bool(is_null)))
}

pub fn is_not_null<'a>(
    mut exprs: Vec<SExpr<'a>>,
    env: &mut Environment<'a>,
) -> Result<SExpr<'a>, String> {
    let expr = exprs.pop().unwrap();
    let is_not_null = match expr {
        SExpr::ISymbol(symbol_id, _) => !matches!(
            crate::types::Value::get_by_id(env.global_val, symbol_id),
            SharedValue::Null
        ),
        other => !matches!(other.eval(env)?.shared_val(), Some(SharedValue::Null)),
    };
    Ok(SExpr::from_owned_value(OwnedValue::Bool(is_not_null)))
}

pub fn cond<'a>(exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    if exprs.len() % 2 == 1 {
        return Err(format!(
            "cond need even number of parameters, found {}",
            exprs.len()
        ));
    }
    let mut exprs = exprs.into_iter();
    while let (Some(condition), Some(expr)) = (exprs.next(), exprs.next()) {
        if is_true(&condition.eval(env)?) {
            return expr.eval(env);
        }
    }
    return Ok(SExpr::Value(Value::null()));
}
