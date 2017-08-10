use super::*;
use super::functions::eval_function;
use types::Value;

pub fn map(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let func = exprs.pop().unwrap();
    let data = exprs.pop().unwrap();
    let mut result;
    match data {
        SExpr::Value(Value::Array(arr)) => {
            result = Vec::with_capacity(arr.len());
            for val in arr {
                result.push(eval_function(
                    &func,
                    vec![SExpr::Value(val)])?);
            }
        },
        SExpr::Vec(expr_list) => {
            result = Vec::with_capacity(expr_list.len());
            for expr in expr_list {
                result.push(eval_function(
                    &func,
                    vec![expr.eval()?])?)
            }
        },
        _ => return Err(format!("Cannot map function on {:?}", data))
    }
    Ok(SExpr::Vec(result))
}