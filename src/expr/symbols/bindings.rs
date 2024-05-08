use crate::expr::interpreter::Envorinment;

use super::super::Value;
use super::*;
use std::collections::LinkedList;

pub fn bind_by_name<'a, 'b>(env: &mut Envorinment<'a>, name: &'b str, val: SExpr<'a>) {
    bind(env, hash_str(name), val)
}

pub fn bind<'a>(env: &mut Envorinment<'a>, id: u64, val: SExpr<'a>) {
    let binding_map = &mut env.bindings;
    let bind_val = if let SExpr::Value(v) = val {
        SExpr::Value(Value::Ref(v.into_ref())) // Get to ref so cloning won't cost much
    } else {
        val
    };
    binding_map
        .entry(id)
        .or_insert_with(|| LinkedList::new())
        .push_front(Rc::new(bind_val));
}

pub fn unbind<'a>(env: &mut Envorinment<'a>, id: u64) {
    let binding_map = &mut env.bindings;
    binding_map
        .entry(id)
        .or_insert_with(|| LinkedList::new())
        .pop_front();
}

pub fn let_binding<'a>(
    env: &mut Envorinment<'a>,
    exprs: Vec<SExpr<'a>>,
) -> Result<SExpr<'a>, String> {
    if exprs.len() < 2 {
        return Err(format!(
            "Too few parameters for let. Required at least 2 but found {}",
            exprs.len()
        ));
    }
    let mut exprs = exprs.into_iter();
    let mut binded_ids = Vec::new();
    {
        let form_expr = exprs.next().unwrap();
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

pub fn define<'a>(
    env: &mut Envorinment<'a>,
    exprs: Vec<SExpr<'a>>,
) -> Result<SExpr<'a>, String> {
    let mut exprs = exprs.into_iter();
    let name = exprs.next().unwrap();
    let val = exprs.next().unwrap().eval(env)?;
    if let SExpr::Symbol(name) = name {
        bind_by_name(env, &name, val);
    } else if let SExpr::ISymbol(id, _) = name {
        bind(env, id, val)
    } else {
        return Err(format!("Cannot bind to {:?}", name));
    }
    return Ok(SExpr::Value(Value::null()));
}
