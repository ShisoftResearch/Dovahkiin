use expr::symbols::misc;
use expr::SExpr;
use std::collections::{HashMap, LinkedList};
use std::rc::Rc;

use super::symbols::bindings::bind_by_name;
#[derive(Debug)]
pub struct Envorinment<'a> {
    pub bindings: HashMap<u64, LinkedList<Rc<SExpr<'a>>>>,
}

impl<'a> Envorinment<'a> {
    pub fn new() -> Self {
        Envorinment {
            bindings: HashMap::new(),
        }
    }
    pub fn get_mut_bindings(&mut self) -> &mut HashMap<u64, LinkedList<Rc<SExpr<'a>>>> {
        &mut self.bindings
    }
}

pub fn eval_all<'a>(
    exprs: Vec<SExpr<'a>>,
    env: &mut Envorinment<'a>,
) -> Result<Vec<SExpr<'a>>, String> {
    let mut result = Vec::with_capacity(exprs.len());
    for expr in exprs {
        result.push(expr.eval(env)?);
    }
    Ok(result)
}

pub fn do_eval<'a>(exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> {
    misc::do_(exprs, env)
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    env: Envorinment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Interpreter {
            env: Envorinment::new(),
        }
    }
    pub fn eval(&mut self, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
        do_eval(exprs, &mut self.env)
    }
    pub fn bind<'b>(&mut self, name: &'b str, expr: SExpr<'a>) {
        bind_by_name(&mut self.env, name, expr)
    }
}
