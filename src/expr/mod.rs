use crate::parser::lisp::ParserExpr;
use crate::types::referred::OwnedValueRef;
use crate::types::{OwnedValue, SharedValue};
use bifrost_hasher::hash_str;
use std::borrow::Borrow;
use std::rc::Rc;

use self::interpreter::Environment;

#[macro_use]
pub mod symbols;
pub mod interpreter;
pub mod serde;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Owned(OwnedValue),
    Shared(SharedValue<'a>),
    Ref(OwnedValueRef),
}

impl<'a> Value<'a> {
    pub const fn null() -> Self {
        Self::Owned(OwnedValue::Null)
    }
    pub fn shared(&'a self) -> SharedValue<'a> {
        match self {
            Value::Owned(v) => v.shared(),
            Value::Shared(v) => v.clone(),
            Value::Ref(v) => v.shared(),
        }
    }
    pub fn owned(val: OwnedValue) -> Self {
        Value::Owned(val)
    }
    pub fn into_owned(self) -> OwnedValue {
        match self {
            Value::Owned(v) => v,
            Value::Shared(v) => v.owned(),
            Value::Ref(v) => (&*v).clone(),
        }
    }
    pub fn into_ref(self) -> OwnedValueRef {
        match self {
            Value::Owned(v) => OwnedValueRef::new(v),
            Value::Shared(v) => OwnedValueRef::new(v.owned()),
            Value::Ref(v) => v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr<'a> {
    Symbol(String),
    ISymbol(u64, String),
    Keyword(u64, String),
    Value(Value<'a>),
    List(Vec<SExpr<'a>>),
    Vec(Vec<SExpr<'a>>),
    META(Box<Self>),
    LAMBDA(Vec<SExpr<'a>>, Vec<SExpr<'a>>),
}

impl<'a> SExpr<'a> {
    pub fn eval(self, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
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
                    let global_val =  crate::types::Value::get_by_id(&env.global_val, symbol_id);
                    if global_val != &SharedValue::NA {
                        Ok(SExpr::from_shared_value(global_val.clone()))
                    } else {
                        Ok(self)
                    }
                }
            }
            _ => Ok(self),
        }
    }
    pub fn from_owned_value(val: OwnedValue) -> Self {
        Self::Value(Value::Owned(val))
    }
    pub fn from_shared_value(val: SharedValue<'a>) -> Self {
        Self::Value(Value::Shared(val))
    }
    pub fn shared_val(&'a self) -> Option<SharedValue<'a>> {
        if let SExpr::Value(v) = self {
            Some(v.shared())
        } else {
            None
        }
    }
    pub fn owned_val(self) -> Option<OwnedValue> {
        if let SExpr::Value(v) = self {
            match v {
                Value::Owned(v) => Some(v),
                Value::Shared(v) => Some(v.owned()),
                Value::Ref(v) => Some((&*v).clone()),
            }
        } else {
            None
        }
    }
    pub fn shared(&'a self) -> Self {
        match self {
            SExpr::Value(Value::Owned(ref owned)) => SExpr::Value(Value::Shared(owned.shared())),
            SExpr::Value(Value::Ref(ref owned)) => SExpr::Value(Value::Shared(owned.shared())),
            _ => self.clone(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            &SExpr::List(ref l) => l.is_empty(),
            &SExpr::Vec(ref v) => v.is_empty(),
            _ => false,
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
        Self::from_owned_value(val)
    }

    fn keyword(name: String) -> Self {
        Self::Keyword(hash_str(&name), name)
    }

    fn into_val(self) -> Result<OwnedValue, String> {
        match self {
            SExpr::Symbol(s) => Ok(OwnedValue::String(s)),
            SExpr::ISymbol(_, s) => Ok(OwnedValue::String(s)),
            SExpr::Value(v) => Ok(v.into_owned()),
            SExpr::List(l) => array_from_exprs(l),
            SExpr::Vec(l) => array_from_exprs(l),
            SExpr::Keyword(_, s) => Ok(OwnedValue::String(s)),
            SExpr::META(m) => Err(format!("Cannot have meta as value {:?}", m)),
            SExpr::LAMBDA(i, o) => Err(format!("Cannot have lambda as value {:?} -> {:?}", i, o)),
        }
    }
}

fn array_from_exprs(l: Vec<SExpr>) -> Result<OwnedValue, String> {
    let mut res = vec![];
    for ele in l.into_iter().map(SExpr::into_val) {
        let val = ele?;
        res.push(val);
    }
    return Ok(OwnedValue::Array(res));
}
