use std::collections::{HashMap, LinkedList};
use std::cell::RefCell;
use std::rc::Rc;
use expr::SExpr;

thread_local!(pub static ENV: RefCell<Envorinment> = RefCell::new(Envorinment::new()));

#[derive(Debug)]
pub struct Envorinment {
    pub bindings: HashMap<u64, LinkedList<Rc<SExpr>>>
}

impl Envorinment {
    pub fn new() -> Envorinment {
        Envorinment {
            bindings: HashMap::new()
        }
    }
}

pub fn eval_all(exprs: Vec<SExpr>) -> Result<Vec<SExpr>, String> {
    let mut result = Vec::with_capacity(exprs.len());
    for expr in exprs {
        result.push(expr.eval()?);
    }
    Ok(result)
}