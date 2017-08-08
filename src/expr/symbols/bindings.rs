use super::*;
use std::collections::LinkedList;
use expr::interpreter::ENV;

pub fn bind(id: u64, val: SExpr) {
    ENV.with(|env| {
        let mut env_borrowed = env.borrow_mut();
        let mut binding_map = &mut env_borrowed.bindings;
        binding_map.entry(id).or_insert_with(|| LinkedList::new()).push_front(val);
    });
}

pub fn let_binding (mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() < 2 {
        return Err(format!("Too few parameters for let. Required at least 2 but found {}", exprs.len()));
    }
    let form_expr = exprs.remove(0);
    let form = if let SExpr::Vec(vec) = form_expr { vec } else {
        return Err(format!("Let need a vector as form, found {:?}", form_expr))
    };
    if form.len() % 2 == 1 {
        return Err(format!("Let form require even number of parameters, found {}", form.len()));
    }
    let body_result = {

    };
    unimplemented!();
}