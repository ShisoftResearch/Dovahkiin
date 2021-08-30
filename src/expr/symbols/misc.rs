use super::*;

pub fn do_<'a>(exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    let mut result = SExpr::Value(Value::null());
    for expr in exprs {
        result = expr.eval(env)?;
    }
    return Ok(result);
}
