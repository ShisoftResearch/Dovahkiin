use super::super::Value;
use super::*;
use expr::interpreter::ENV;
use std::collections::LinkedList;
use std::rc::Rc;

pub fn bind_rc(id: u64, val_rc: Rc<SExpr>) {
    ENV.with(|env| {
        let env_borrow = env.borrow_mut();
        let binding_map = &mut env_borrow.bindings.borrow_mut();
        binding_map
            .entry(id)
            .or_insert_with(|| LinkedList::new())
            .push_front(val_rc);
    });
}

pub fn bind(id: u64, val: SExpr) {
    bind_rc(id, Rc::new(val))
}

pub fn unbind(id: u64) {
    ENV.with(|env| {
        let env_borrow = env.borrow_mut();
        let binding_map = &mut env_borrow.bindings.borrow_mut();
        binding_map
            .entry(id)
            .or_insert_with(|| LinkedList::new())
            .pop_front();
    });
}

pub fn let_binding(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() < 2 {
        return Err(format!(
            "Too few parameters for let. Required at least 2 but found {}",
            exprs.len()
        ));
    }
    let mut binded_ids = Vec::new();
    {
        let form_expr = exprs.remove(0);
        let form = if let SExpr::Vec(vec) = form_expr {
            vec
        } else {
            return Err(format!("Let need a vector as form, found {:?}", form_expr));
        };
        if form.len() % 2 == 1 {
            return Err(format!(
                "Let form require even number of parameters, but found {}",
                form.len()
            ));
        }
        let mut form_iter = form.into_iter();
        while let Some(symbol) = form_iter.next() {
            let symbol_id = match symbol {
                SExpr::Symbol(ref sym_str) => hash_str(sym_str),
                SExpr::ISymbol(id, _) => id,
                _ => return Err(format!("Cannot bind to {:?}, need symbol", symbol)),
            };
            if let Some(expr) = form_iter.next() {
                bind(symbol_id, expr.eval()?);
                binded_ids.push(symbol_id);
            } else {
                return Err(format!("cannot bind to {:?}, no value", symbol));
            }
        }
    }
    let mut body_result = SExpr::Value(Value::Null);
    for body_line in exprs {
        body_result = body_line.eval()?;
    }
    for binded_id in binded_ids {
        unbind(binded_id);
    }
    return Ok(body_result);
}

pub fn define(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let name = exprs.remove(0);
    let val = exprs.remove(0).eval()?;
    if let SExpr::Symbol(name) = name {
        bind(hash_str(&name), val);
    } else if let SExpr::ISymbol(id, _) = name {
        bind(id, val)
    } else {
        return Err(format!("Cannot bind to {:?}", name));
    }
    return Ok(SExpr::Value(Value::Null));
}
