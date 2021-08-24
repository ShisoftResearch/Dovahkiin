use super::functions::eval_function;
use super::utils::is_true;
use super::*;

pub fn to_array<'a>(expr: SExpr<'a>) -> Result<SExpr<'a>, String> {
    match expr {
        SExpr::Vec(vec) => {
            let mut array = Vec::new();
            for expr in vec {
                if let SExpr::Value(val) = expr {
                    array.push(val.norm())
                } else {
                    return Err(format!("Data {:?} cannot be value", expr));
                }
            }
            return Ok(SExpr::shared_value(SharedValue::Array(array)));
        }
        SExpr::Value(Value::Shared(SharedValue::Array(_)))
        | SExpr::Value(Value::Owned(OwnedValue::Array(_))) => return Ok(expr),
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
        SExpr::Value(Value::Owned(OwnedValue::Array(array))) => {
            return Ok(SExpr::Vec(
                array
                    .into_iter()
                    .map(|val| SExpr::Value(Value::Owned(val)))
                    .collect(),
            ));
        }
        SExpr::Value(Value::Shared(SharedValue::Array(array))) => {
            return Ok(SExpr::Vec(
                array
                    .into_iter()
                    .map(|val| SExpr::Value(Value::Shared(val)))
                    .collect(),
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

pub fn map<'a>(func: SExpr<'a>, data: SExpr<'a>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    match data {
        SExpr::Value(Value::Owned(OwnedValue::Array(_)))
        | SExpr::Value(Value::Shared(SharedValue::Array(_))) => return map(func, to_vec(data)?, env),
        SExpr::Vec(expr_list) => {
            let mut result = Vec::with_capacity(expr_list.len());
            for expr in expr_list {
                result.push(eval_function(&func, vec![expr.eval(env)?], env)?,)
            }
            return Ok(SExpr::Vec(result));
        }
        _ => return Err(format!("Cannot map function on {:?}", data)),
    }
}

pub fn filter<'a>(func: SExpr<'a>, data: SExpr<'a>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    match data {
        SExpr::Value(Value::Owned(OwnedValue::Array(_)))
        | SExpr::Value(Value::Shared(SharedValue::Array(_))) => return filter(func, to_vec(data)?, env),
        SExpr::Vec(expr_list) => {
            let mut result = Vec::with_capacity(expr_list.len());
            for expr in expr_list {
                let val = expr.eval(env)?;
                if is_true(eval_function(&func, vec![val.clone()], env)?) {
                    result.push(val)
                }
            }
            return Ok(SExpr::Vec(result));
        }
        _ => return Err(format!("Cannot map function on {:?}", data)),
    }
}
