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
        _ => return Err(format!("{:?} is not a function", func_expr))
    }
}