use super::utils::is_true;
use super::*;

pub fn if_<'a>(env: &mut Envorinment<'a>, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    let else_expr = iter.next();
    if is_true(&tester.eval(env)?) {
        return then_expr.eval(env);
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval(env);
    } else {
        return Ok(SExpr::Value(Value::null()));
    }
}

pub fn if_not<'a>(env: &mut Envorinment<'a>, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    let else_expr = iter.next();
    if !is_true(&tester.eval(env)?) {
        return then_expr.eval(env);
    } else if let Some(else_expr) = else_expr {
        return else_expr.eval(env);
    } else {
        return Ok(SExpr::Value(Value::null()));
    }
}

pub fn when<'a>(env: &mut Envorinment<'a>, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    if is_true(&tester.eval(env)?) {
        return then_expr.eval(env);
    } else {
        return Ok(SExpr::Value(Value::null()));
    }
}

pub fn when_not<'a>(env: &mut Envorinment<'a>, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let mut iter = exprs.into_iter();
    let tester = iter.next().unwrap();
    let then_expr = iter.next().unwrap();
    if !is_true(&tester.eval(env)?) {
        return then_expr.eval(env);
    } else {
        return Ok(SExpr::Value(Value::null()));
    }
}
