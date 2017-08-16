use super::SExpr;
use super::utils::is_true;
use expr::symbols::Symbol;
use types::Value;

pub fn or(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    for expr in exprs {
        if is_true(expr.eval()?) {
            return Ok(SExpr::Value(Value::Bool(true)))
        }
    }
    return Ok(SExpr::Value(Value::Bool(false)))
}

pub fn and(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    for expr in exprs {
        if !is_true(expr.eval()?) {
            return Ok(SExpr::Value(Value::Bool(false)))
        }
    }
    return Ok(SExpr::Value(Value::Bool(true)))
}

pub fn cond(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() % 2 == 1 {
        return Err(format!("cond need even number of parameters, found {}", exprs.len()))
    }
    let mut exprs = exprs.into_iter();
    while let (Some(condition), Some(expr)) = (exprs.next(), exprs.next()) {
        if is_true(condition.eval()?) {
            return expr.eval();
        }
    }
    return Ok(SExpr::Value(Value::Null));
}