use crate::expr::interpreter::Envorinment;

use super::super::Value;
use super::*;
use std::collections::LinkedList;
use std::rc::Rc;

pub fn bind_rc<'a>(env: &mut Envorinment<'a>, id: u64, val_rc: Rc<SExpr<'a>>) {
    let binding_map = &mut env.bindings;
    binding_map
        .entry(id)
        .or_insert_with(|| LinkedList::new())
        .push_front(val_rc);
}

pub fn bind<'a>(env: &mut Envorinment<'a>, id: u64, val: SExpr<'a>) {
    bind_rc(env, id, Rc::new(val))
}

pub fn unbind<'a>(env: &mut Envorinment<'a>,id: u64) {
    let binding_map = &mut env.bindings;
    binding_map
        .entry(id)
        .or_insert_with(|| LinkedList::new())
        .pop_front();
}

pub fn let_binding<'a>(env: &mut Envorinment<'a>, mut exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
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
                let val = expr.eval(env)?;
                bind(env, symbol_id, val);
                binded_ids.push(symbol_id);
            } else {
                return Err(format!("cannot bind to {:?}, no value", symbol));
            }
        }
    }
    let mut body_result = SExpr::Value(Value::null());
    for body_line in exprs {
        body_result = body_line.eval(env)?;
    }
    for binded_id in binded_ids {
        unbind(env, binded_id);
    }
    return Ok(body_result);
}

pub fn define<'a>(env: &mut Envorinment<'a>, mut exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let name = exprs.remove(0);
    let val = exprs.remove(0).eval(env)?;
    if let SExpr::Symbol(name) = name {
        bind(env, hash_str(&name), val);
    } else if let SExpr::ISymbol(id, _) = name {
        bind(env, id, val)
    } else {
        return Err(format!("Cannot bind to {:?}", name));
    }
    return Ok(SExpr::Value(Value::null()));
}
