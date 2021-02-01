use super::functions::eval_function;
use super::utils::is_true;
use super::*;

pub fn to_array(expr: SExpr) -> Result<SExpr, String> {
    match expr {
        SExpr::Vec(vec) => {
            let mut array = Vec::new();
            for expr in vec {
                if let SExpr::Value(val) = expr {
                    array.push(val)
                } else {
                    return Err(format!("Data {:?} cannot be value", expr));
                }
            }
            return Ok(SExpr::Value(Value::Array(array)));
        }
        SExpr::Value(Value::Array(_)) => Ok(expr),
        _ => {
            return Err(format!(
                "Only Vector can convert into array, found {:?}",
                expr
            ))
        }
    }
}

pub fn to_vec(expr: SExpr) -> Result<SExpr, String> {
    match expr {
        SExpr::Value(Value::Array(array)) => {
            return Ok(SExpr::Vec(
                array.into_iter().map(|val| SExpr::Value(val)).collect(),
            ));
        }
        SExpr::Vec(_) => Ok(expr),
        _ => {
            return Err(format!(
                "Only array value can convert into vector, found {:?}",
                expr
            ))
        }
    }
}

pub fn map(func: SExpr, data: SExpr) -> Result<SExpr, String> {
    match data {
        SExpr::Value(Value::Array(_)) => return map(func, to_vec(data)?),
        SExpr::Vec(expr_list) => {
            let mut result = Vec::with_capacity(expr_list.len());
            for expr in expr_list {
                result.push(eval_function(&func, vec![expr.eval()?])?)
            }
            return Ok(SExpr::Vec(result));
        }
        _ => return Err(format!("Cannot map function on {:?}", data)),
    }
}

pub fn filter(func: SExpr, data: SExpr) -> Result<SExpr, String> {
    match data {
        SExpr::Value(Value::Array(_)) => return filter(func, to_vec(data)?),
        SExpr::Vec(expr_list) => {
            let mut result = Vec::with_capacity(expr_list.len());
            for expr in expr_list {
                let val = expr.eval()?;
                if is_true(eval_function(&func, vec![val.clone()])?) {
                    result.push(val)
                }
            }
            return Ok(SExpr::Vec(result));
        }
        _ => return Err(format!("Cannot map function on {:?}", data)),
    }
}
