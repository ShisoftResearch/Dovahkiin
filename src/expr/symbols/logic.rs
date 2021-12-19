use super::utils::is_true;
use super::*;

pub fn or<'a>(exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    for expr in exprs {
        if is_true(&expr.eval(env)?) {
            return Ok(SExpr::owned_value(OwnedValue::Bool(true)));
        }
    }
    return Ok(SExpr::owned_value(OwnedValue::Bool(false)));
}

pub fn and<'a>(exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    for expr in exprs {
        if !is_true(&expr.eval(env)?) {
            return Ok(SExpr::owned_value(OwnedValue::Bool(false)));
        }
    }
    return Ok(SExpr::owned_value(OwnedValue::Bool(true)));
}

pub fn cond<'a>(exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
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
