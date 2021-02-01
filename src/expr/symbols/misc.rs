use super::*;

pub fn do_(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut result = SExpr::Value(Value::Null);
    for expr in exprs {
        result = expr.eval()?;
    }
    return Ok(result);
}
