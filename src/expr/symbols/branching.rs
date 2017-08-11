use super::*;
use super::bindings::*;
use types::Value;
use super::utils::is_true;

pub fn if_(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let tester = exprs.pop().unwrap();
    let then_expr = exprs.pop().unwrap();
    let else_expr = exprs.pop();
    if is_true(tester.eval()?) {
        return then_expr.eval();
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Bool(false)))
    }
}

pub fn if_not(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let tester = exprs.pop().unwrap();
    let then_expr = exprs.pop().unwrap();
    let else_expr = exprs.pop();
    if !is_true(tester.eval()?) {
        return then_expr.eval();
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Bool(false)))
    }
}

pub fn when(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let tester = exprs.pop().unwrap();
    let then_expr = exprs.pop().unwrap();
    if is_true(tester.eval()?) {
        return then_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Bool(false)))
    }
}

pub fn when_not(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let tester = exprs.pop().unwrap();
    let then_expr = exprs.pop().unwrap();
    if !is_true(tester.eval()?) {
        return then_expr.eval();
    } else {
        return Ok(SExpr::Value(Value::Bool(false)))
    }
}