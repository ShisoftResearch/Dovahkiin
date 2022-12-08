use std::borrow::Borrow;
use std::rc::Rc;
use bifrost_hasher::hash_str;
use crate::types::{OwnedValue, SharedValue};
use crate::parser::lisp::ParserExpr;

use self::interpreter::Envorinment;

#[macro_use]
pub mod symbols;
pub mod interpreter;
pub mod serde;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Owned(OwnedValue),
    Shared(SharedValue<'a>),
}

impl<'a> Value<'a> {
    pub const fn null() -> Self {
        Self::Owned(OwnedValue::Null)
    }
    pub fn norm(&'a self) -> SharedValue<'a> {
        match self {
            Value::Owned(v) => v.shared(),
            Value::Shared(v) => v.clone(),
        }
    }
    pub fn owned(val: OwnedValue) -> Self {
        Value::Owned(val)
    }
    pub fn into_owned_val(self) -> OwnedValue {
        match self {
            Value::Owned(v) => v,
            Value::Shared(v) => v.owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr<'a> {
    Symbol(String),
    ISymbol(u64, String),
    Value(Value<'a>),
    List(Vec<SExpr<'a>>),
    Vec(Vec<SExpr<'a>>),
    LAMBDA(Vec<SExpr<'a>>, Vec<SExpr<'a>>),
}

impl<'a> SExpr<'a> {
    pub fn eval(self, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
        match self {
            SExpr::List(exprs) => {
                if exprs.len() == 0 {
                    Ok(SExpr::Value(Value::null()))
                } else {
                    let mut iter = exprs.into_iter();
                    let func = iter.next().unwrap().eval(env)?;
                    Ok(symbols::functions::eval_function(
                        &func,
                        iter.collect(),
                        env,
                    )?)
                }
            }
            SExpr::ISymbol(symbol_id, _) => {
                let env_bind_ref;
                let bindings = env.get_mut_bindings();
                env_bind_ref = if let Some(binding_list) = bindings.get(&symbol_id) {
                    binding_list.front().cloned()
                } else {
                    None
                };
                if let Some(binding) = env_bind_ref {
                    let bind_expr: &SExpr = binding.borrow();
                    Ok(bind_expr.clone())
                } else {
                    Ok(self)
                }
            }
            _ => Ok(self),
        }
    }
    pub fn owned_value(val: OwnedValue) -> Self {
        Self::Value(Value::Owned(val))
    }
    pub fn shared_value(val: SharedValue<'a>) -> Self {
        Self::Value(Value::Shared(val))
    }
    pub fn val(&'a self) -> Option<SharedValue<'a>> {
        if let SExpr::Value(v) = self {
            Some(v.norm())
        } else {
            None
        }
    }
    pub fn owned_val(self) -> Option<OwnedValue> {
        if let SExpr::Value(v) = self {
            match v {
                Value::Owned(v) => Some(v),
                Value::Shared(v) => Some(v.owned())
            }
        } else {
            None
        }
    }
    pub fn norm(&'a self) -> Self {
        if let SExpr::Value(Value::Owned(ref owned)) = self {
            SExpr::Value(Value::Shared(owned.shared()))
        } else {
            self.clone()
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            &SExpr::List(ref l) => l.is_empty(),
            &SExpr::Vec(ref v) => v.is_empty(),
            _ => false
        }
    }
}

impl ParserExpr for SExpr<'_> {
    fn list(data: Vec<Self>) -> Self {
        Self::List(data)
    }

    fn vec(data: Vec<Self>) -> Self {
        Self::Vec(data)
    }

    fn symbol(name: String) -> Self {
        Self::ISymbol(hash_str(&name), name)
    }

    fn owned_val(val: OwnedValue) -> Self {
        Self::owned_value(val)
    }
}
