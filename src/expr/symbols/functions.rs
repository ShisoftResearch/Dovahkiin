use super::*;
use super::Symbol;
use super::lambda::eval_lambda;
use super::super::interpreter::ENV;
use types::Value;
use types::custom_types::map::*;
use std::rc::Rc;
use std::borrow::Borrow;

pub fn eval_function(func_expr: &SExpr, params: Vec<SExpr>) -> Result<SExpr, String> {
    match func_expr {
        &SExpr::ISymbol(symbol_id) => {
            let mut env_bind_ref: Option<Rc<SExpr>> = None;
            ENV.with(|env| {
                let mut env_borrowed = env.borrow();
                env_bind_ref = if let Some(binding_list) = env_borrowed.bindings.get(&symbol_id) {
                    binding_list.front().cloned()
                } else { None }
            });
            if let Some(env_bind) = env_bind_ref {
                return eval_lambda(env_bind.borrow(), params)
            } else {
                // internal functions
                match ISYMBOL_MAP.get(&symbol_id) {
                    Some(symbol) => {
                        return symbol.eval(params)
                    },
                    _ =>
                        return Err(format!("Cannot find symbol {}", symbol_id))
                }
            }
        },
        &SExpr::Symbol(ref symbol_name) =>
            return eval_function(&SExpr::ISymbol(hash_str(symbol_name)), params),
        &SExpr::Value(Value::String(ref str_key)) => {
            // same as clojure (:key map)
            if params.len() > 1 {
                return Err(format!("get from map can only take one parameter, found {}", params.len()))
            }
            if let Some(&SExpr::Value(Value::Map(ref m))) = params.get(0) {
                return Ok(SExpr::Value(m.get(str_key).clone()));
            } else {
                return Err(format!("When use string value as function, \
                only one map parameter is accepted, found {:?}", params))
            }
        },
        &SExpr::Value(Value::U64(index)) => {
            // get element by index from vec or by key_id form map
            if params.len() > 1 {
                return Err(format!("get by index/id can only take one parameter, found {}", params.len()))
            }
            match params.get(0) {
                Some(&SExpr::Value(Value::Map(ref m))) => {
                    return Ok(SExpr::Value(m.get_by_key_id(index).clone()));
                },
                Some(&SExpr::Value(Value::Array(ref arr))) => {
                    return Ok(SExpr::Value(arr.get(index as usize)
                        .cloned()
                        .unwrap_or(Value::Null)))
                },
                _ => {
                    return Err(format!("Data type not accepted for {:?}", params))
                }
            }
        },
        &SExpr::Value(Value::Map(ref m)) => {
            if params.len() > 1 {
                return Err(format!("get map can only take one parameter, found {}", params.len()))
            }
            match params.get(0) {
                Some(&SExpr::Value(Value::String(ref str_key))) => {
                    return Ok(SExpr::Value(m.get(str_key).clone()))
                },
                Some(&SExpr::Value(Value::U64(key_id))) => {
                    return Ok(SExpr::Value(m.get_by_key_id(key_id).clone()))
                },
                _ => {
                    return Err(format!("Key format not accepted, expect one string or u64\
                    Found {:?}", params));
                }
            }
        },
        &SExpr::Value(Value::Array(ref array)) => {
            if params.len() > 1 {
                return Err(format!("get map can only take one parameter, found {}", params.len()))
            }
            match params.get(0) {
                Some(&SExpr::Value(Value::U64(key_id))) => {
                    return Ok(SExpr::Value(array.get(key_id as usize)
                        .cloned()
                        .unwrap_or(Value::Null)))
                },
                _ => {
                    return Err(format!("Index format not accepted, expect u64\
                    Found {:?}", params));
                }
            }
        }
        _ => return Err(format!("{:?} is not a function", func_expr))
    }
}