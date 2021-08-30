use crate::expr::interpreter::Envorinment;

use super::bindings::bind;
use super::lambda::{eval_lambda, lambda_placeholder};
use super::*;
use std::rc::Rc;

pub fn eval_function<'a>(
    func_expr: &SExpr<'a>,
    params: Vec<SExpr<'a>>,
    env: &mut Envorinment<'a>,
) -> Result<SExpr<'a>, String> {
    match func_expr {
        &SExpr::ISymbol(symbol_id, ref name) => {
            let env_bind;
            let bindings = env.get_mut_bindings();
            env_bind = if let Some(binding_list) = bindings.get(&symbol_id) {
                binding_list.front().cloned()
            } else {
                None
            };
            if let Some(env_bind) = env_bind {
                return eval_lambda(env_bind, params, env);
            } else {
                // internal functions
                let symbols = ISYMBOL_MAP.map.borrow();
                match symbols.get(&symbol_id) {
                    Some(symbol) => {
                        // if the symbol is not a macro, parameters will all be evaled here. Or passthrough those expressions.
                        let exprs = if symbol.is_macro() {
                            params
                        } else {
                            let mut evaled_params = Vec::with_capacity(params.len());
                            for param in params {
                                evaled_params.push(param.eval(env)?);
                            }
                            evaled_params
                        };
                        return symbol.eval(exprs, env);
                    }
                    _ => {
                        return Err(format!(
                            "Cannot find symbol \'{}\', id: {}",
                            name, symbol_id
                        ))
                    }
                }
            }
        }
        &SExpr::Symbol(ref symbol_name) => {
            let symbol_id = hash_str(symbol_name);
            let symbol_name = symbol_name.clone();
            return eval_function(&SExpr::ISymbol(symbol_id, symbol_name), params, env);
        }
        &SExpr::LAMBDA(_, _) => return eval_lambda(func_expr.clone(), params, env),
        &SExpr::Value(ref v) => return eval_value(v, params),
        _ => {}
    }
    return Err(format!("{:?} is not a function", func_expr));
}

fn eval_value<'a>(v: &Value<'a>, params: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    match &v {
        &Value::Shared(sv) => {
            match sv {
                SharedValue::String(str_key) => {
                    // same as clojure (:key map)
                    if params.len() > 1 {
                        return Err(format!(
                            "get from map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    if let Some(Some(SharedValue::Map(ref m))) =
                        params.get(0).map(|expr| expr.val())
                    {
                        return Ok(SExpr::owned_value(m.get(str_key).owned()));
                    } else {
                        return Err(format!(
                            "When use string value as function, \
                        only one map parameter is accepted, found {:?}",
                            params
                        ));
                    }
                }
                SharedValue::U64(index) => {
                    // get element by index from vec or by key_id form map
                    if params.len() > 1 {
                        return Err(format!(
                            "get by index/id can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|expr| expr.val()) {
                        Some(Some(SharedValue::Map(ref m))) => {
                            let val = m.get_by_key_id(**index).clone();
                            return Ok(SExpr::owned_value(val.owned()));
                        }
                        Some(Some(SharedValue::Array(ref arr))) => {
                            return Ok(SExpr::owned_value(
                                arr.get(**index as usize)
                                    .cloned()
                                    .unwrap_or(SharedValue::Null)
                                    .owned(),
                            ))
                        }
                        _ => return Err(format!("Data type not accepted for {:?}", params)),
                    }
                }
                SharedValue::Map(ref m) => {
                    if params.len() > 1 {
                        return Err(format!(
                            "get map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|v| v.val()) {
                        Some(Some(SharedValue::String(str_key))) => {
                            return Ok(SExpr::shared_value(m.get(str_key).clone()))
                        }
                        Some(Some(SharedValue::U64(key_id))) => {
                            return Ok(SExpr::shared_value(m.get_by_key_id(*key_id).clone()))
                        }
                        _ => {
                            return Err(format!(
                                "Key format not accepted, expect one string or u64\
                                 Found {:?}",
                                params
                            ));
                        }
                    }
                }
                SharedValue::Array(ref array) => {
                    if params.len() > 1 {
                        return Err(format!(
                            "get map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|v| v.val()) {
                        Some(Some(SharedValue::U64(key_id))) => {
                            return Ok(SExpr::shared_value(
                                array
                                    .get(*key_id as usize)
                                    .cloned()
                                    .unwrap_or(SharedValue::Null),
                            ))
                        }
                        _ => {
                            return Err(format!(
                                "Index format not accepted, expect u64\
                                 Found {:?}",
                                params
                            ));
                        }
                    }
                }
                _ => {
                    return Err(format!("value {:?} cannot be used as a function", v));
                }
            }
        }
        &Value::Owned(ov) => {
            match ov {
                OwnedValue::String(str_key) => {
                    // same as clojure (:key map)
                    if params.len() > 1 {
                        return Err(format!(
                            "get from map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    if let Some(Some(SharedValue::Map(ref m))) =
                        params.get(0).map(|expr| expr.val())
                    {
                        return Ok(SExpr::owned_value(m.get(str_key).owned()));
                    } else {
                        return Err(format!(
                            "When use string value as function, \
                        only one map parameter is accepted, found {:?}",
                            params
                        ));
                    }
                }
                OwnedValue::U64(index) => {
                    // get element by index from vec or by key_id form map
                    if params.len() > 1 {
                        return Err(format!(
                            "get by index/id can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|expr| expr.val()) {
                        Some(Some(SharedValue::Map(ref m))) => {
                            let val = m.get_by_key_id(*index).clone();
                            return Ok(SExpr::owned_value(val.owned()));
                        }
                        Some(Some(SharedValue::Array(ref arr))) => {
                            return Ok(SExpr::owned_value(
                                arr.get(*index as usize)
                                    .cloned()
                                    .unwrap_or(SharedValue::Null)
                                    .owned(),
                            ))
                        }
                        _ => return Err(format!("Data type not accepted for {:?}", params)),
                    }
                }
                OwnedValue::Map(ref m) => {
                    if params.len() > 1 {
                        return Err(format!(
                            "get map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|v| v.val()) {
                        Some(Some(SharedValue::String(str_key))) => {
                            return Ok(SExpr::owned_value(m.get(str_key).clone()))
                        }
                        Some(Some(SharedValue::U64(key_id))) => {
                            return Ok(SExpr::owned_value(m.get_by_key_id(*key_id).clone()))
                        }
                        _ => {
                            return Err(format!(
                                "Key format not accepted, expect one string or u64\
                                 Found {:?}",
                                params
                            ));
                        }
                    }
                }
                OwnedValue::Array(ref array) => {
                    if params.len() > 1 {
                        return Err(format!(
                            "get map can only take one parameter, found {}",
                            params.len()
                        ));
                    }
                    match params.get(0).map(|v| v.val()) {
                        Some(Some(SharedValue::U64(key_id))) => {
                            return Ok(SExpr::owned_value(
                                array
                                    .get(*key_id as usize)
                                    .cloned()
                                    .unwrap_or(OwnedValue::Null),
                            ))
                        }
                        _ => {
                            return Err(format!(
                                "Index format not accepted, expect u64\
                                 Found {:?}",
                                params
                            ));
                        }
                    }
                }
                _ => {
                    return Err(format!("value {:?} cannot be used as a function", v));
                }
            }
        }
    }
}

pub fn defn<'a>(env: &mut Envorinment<'a>, mut exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let name = exprs.remove(0);
    let lambda = lambda_placeholder(exprs)?;
    if let SExpr::Symbol(name) = name {
        bind(env, hash_str(&name), lambda);
    } else if let SExpr::ISymbol(id, _) = name {
        bind(env, id, lambda);
    } else {
        return Err(format!(
            "Function name should be a symbol, found {:?}",
            name
        ));
    }
    return Ok(SExpr::Value(Value::null()));
}
