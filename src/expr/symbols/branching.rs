use super::utils::is_true;
use super::*;

pub fn if_(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    let else_expr = iter.next();
    if is_true(tester.eval()?) {
        return then_expr.eval();
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Null));
    }
}

pub fn if_not(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    let else_expr = iter.next();
    if !is_true(tester.eval()?) {
        return then_expr.eval();
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Null));
    }
}

pub fn when(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    if is_true(tester.eval()?) {
        return then_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Null));
    }
}

pub fn when_not(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    if !is_true(tester.eval()?) {
        return then_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Null));
    }
}
