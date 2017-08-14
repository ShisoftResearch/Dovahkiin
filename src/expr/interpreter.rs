use std::collections::{HashMap, LinkedList};
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::RefMut;
use expr::SExpr;
use expr::symbols::misc;

thread_local!(pub static ENV: RefCell<Rc<Envorinment>> = RefCell::new(Rc::new(Envorinment::new())));

#[derive(Debug)]
pub struct Envorinment {
    pub bindings: RefCell<HashMap<u64, LinkedList<Rc<SExpr>>>>
}

impl Envorinment {
    pub fn new() -> Envorinment {
        Envorinment {
            bindings: RefCell::new(HashMap::new())
        }
    }
    pub fn get_mut_bindings(&self) -> RefMut<HashMap<u64, LinkedList<Rc<SExpr>>>> {
        self.bindings.borrow_mut()
    }
}

pub fn eval_all(exprs: Vec<SExpr>) -> Result<Vec<SExpr>, String> {
    let mut result = Vec::with_capacity(exprs.len());
    for expr in exprs {
        result.push(expr.eval()?);
    }
    Ok(result)
}

pub fn do_eval(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    misc::do_(exprs)
}

#[derive(Debug)]
pub struct Interpreter {
    env: Rc<Envorinment>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Rc::new(Envorinment::new())
        }
    }
    pub fn eval(&self, exprs: Vec<SExpr>) -> Result<SExpr, String> {
        ENV.with(|env| {
            let mut env_borrowed = env.borrow_mut();
            *env_borrowed = self.env.clone();
        });
        do_eval(exprs)
    }
}