use super::*;
use super::lambda::eval_lambda;

pub fn map(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let func = exprs.remove(0);
    let mut result = Vec::with_capacity(exprs.len());
    for val in exprs {
        result.push(eval_lambda(&func, vec![val])?);
    }
    Ok(SExpr::Vec(result))
}