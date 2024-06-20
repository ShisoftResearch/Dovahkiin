use bifrost_hasher::hash_str;

use crate::expr::symbols::misc;
use crate::expr::SExpr;
use crate::types::SharedValue;
use std::collections::{HashMap, LinkedList};
use std::mem;
use std::rc::Rc;

use super::symbols::bindings::bind;
use super::symbols::bindings::bind_by_name;

#[derive(Debug)]
pub struct Environment<'a> {
    pub bindings: HashMap<u64, LinkedList<Rc<SExpr<'a>>>>,
    pub global_val: SharedValue<'a>
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            global_val: SharedValue::NA
        }
    }
    pub fn from_global_val(global_val: SharedValue<'a>) -> Self {
        Environment {
            bindings: HashMap::new(),
            global_val
        }
    }
    pub fn get_mut_bindings(&mut self) -> &mut HashMap<u64, LinkedList<Rc<SExpr<'a>>>> {
        &mut self.bindings
    }
}

pub fn eval_all<'a>(
    exprs: Vec<SExpr<'a>>,
    env: &mut Environment<'a>,
) -> Result<Vec<SExpr<'a>>, String> {
    let mut result = Vec::with_capacity(exprs.len());
    for expr in exprs {
        result.push(expr.eval(env)?);
    }
    Ok(result)
}

pub fn do_eval<'a>(exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> {
    misc::do_(exprs, env)
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    env: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }
    pub fn from_global_val(val: SharedValue<'a>) -> Self {
        Interpreter { env: Environment::from_global_val(val) }
    }
    pub fn eval(&mut self, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
        do_eval(exprs, &mut self.env)
    }
    pub fn bind<'b>(&mut self, name: &'b str, expr: SExpr<'a>) {
        bind_by_name(&mut self.env, name, expr)
    }
    pub fn bind_by_id(&mut self, id: u64, expr: SExpr<'a>) {
        bind(&mut self.env, id, expr)
    }
    pub fn get_env(&mut self) -> &mut Environment<'a> {
        &mut self.env
    }
    pub fn clear(&mut self) {
        self.env.bindings.clear();
    }
    pub fn unbind<'b>(&mut self, name: &'b str) -> Option<SExpr<'a>> {
        self.env
            .bindings
            .remove(&hash_str(name))
            .and_then(|mut list| {
                list.pop_front()
                    .map(|rc| Rc::<SExpr<'_>>::into_inner(rc).unwrap())
            })
    }
    pub fn set_global_val(&mut self, val: SharedValue<'a>) {
        self.env.global_val = val;
    }
    pub fn unset_global_val(&mut self) -> SharedValue<'a> {
        mem::replace(&mut self.env.global_val, SharedValue::NA)
    }
}
