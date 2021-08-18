use expr::interpreter::ENV;
use std::borrow::Borrow;
use std::rc::Rc;
use types::OwnedValue as Value;

#[macro_use]
pub mod symbols;
pub mod interpreter;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Value {
//     Owned(OwnedValue),
//     Shared(SharedValue)
// }

// impl Value {
//     pub const fn null() -> Self {
//         Self::Owned(OwnedValue::Null)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExpr {
    Symbol(String),
    ISymbol(u64, String),
    Value(Value),
    List(Vec<SExpr>),
    Vec(Vec<SExpr>),
    LAMBDA(Vec<SExpr>, Vec<SExpr>),
}

impl SExpr {
    pub fn eval(self) -> Result<SExpr, String> {
        match self {
            SExpr::List(exprs) => {
                if exprs.len() == 0 {
                    Ok(SExpr::Value(Value::Null))
                } else {
                    let mut iter = exprs.into_iter();
                    let func = iter.next().unwrap().eval()?;
                    Ok(symbols::functions::eval_function(&func, iter.collect())?)
                }
            }
            SExpr::ISymbol(symbol_id, _) => {
                let mut env_bind_ref: Option<Rc<SExpr>> = None;
                ENV.with(|env| {
                    let env_borrowed = env.borrow();
                    let bindings = env_borrowed.get_mut_bindings();
                    env_bind_ref = if let Some(binding_list) = bindings.get(&symbol_id) {
                        binding_list.front().cloned()
                    } else {
                        None
                    }
                });
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
}
