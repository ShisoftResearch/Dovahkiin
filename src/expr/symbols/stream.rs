use super::*;
use super::utils::is_true;
use super::functions::eval_function;
use std::panic;
use types::Value;

pub fn to_array(expr: SExpr) -> Result<SExpr, String> {
    match expr {
        SExpr::Vec(vec) => {
            let mut array = Vec::new();
            for expr in vec {
                if let SExpr::Value(val) = expr {
                    array.push(val)
                } else {
                    return Err(format!("Data {:?} cannot be value", expr))
                }
            }
            return Ok(SExpr::Value(Value::Array(array)));
        },
        _ => {
            return Err(format!("Only Vector can convert into array, found {:?}", expr))
        }
    }
}

pub fn to_vec(expr: SExpr) -> Result<SExpr, String> {
    match expr {
        SExpr::Value(Value::Array(array)) => {
            return Ok(SExpr::Vec(array.into_iter().map(|val| SExpr::Value(val)).collect()));
        },
        _ => {
            return Err(format!("Only array value can convert into vector, found {:?}", expr))
        }
    }
}

pub fn map(func: SExpr, data: SExpr) -> Result<SExpr, String> {
    let mut result;
    match data {
        SExpr::Value(Value::Array(_)) => {
            return map(func, to_vec(data)?)
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

pub fn filter(func: SExpr, data: SExpr) -> Result<SExpr, String> {
    match data {
        SExpr::Value(Value::Array(_)) => {
            return filter(func, to_vec(data)?)
        },
        SExpr::Vec(expr_list) => {
            panic::catch_unwind(|| {
                let exprs: Vec<SExpr> = expr_list
                    .into_iter()
                    .filter(|expr| {
                        // let it panic
                        let val = expr.clone().eval().unwrap();
                        is_true(eval_function(
                            &func,
                            vec![val]).unwrap())
                    })
                    .collect();
                SExpr::Vec(exprs)
            }).map_err(|e| format!("Cannot filter, for exception {:?}", e))
        },
        _ => return Err(format!("Cannot map function on {:?}", data))
    }
}