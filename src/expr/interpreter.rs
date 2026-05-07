use bifrost_hasher::hash_str;

use crate::expr::symbols::misc;
use crate::expr::SExpr;
use crate::types::SharedValue;
use std::collections::{BTreeMap, LinkedList};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use super::symbols::bindings::bind;
use super::symbols::bindings::bind_by_name;

pub const DEFAULT_GLOBAL_VAL: SharedValue = SharedValue::NA;

#[derive(Debug)]
pub struct Environment<'a> {
    pub bindings: BTreeMap<u64, LinkedList<Rc<SExpr<'a>>>>,
    pub global_val: &'a SharedValue<'a>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Environment {
            bindings: BTreeMap::new(),
            global_val: &DEFAULT_GLOBAL_VAL,
        }
    }
    pub fn get_mut_bindings(&mut self) -> &mut BTreeMap<u64, LinkedList<Rc<SExpr<'a>>>> {
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
    pub fn set_global_val(&mut self, val: &'a SharedValue<'a>) {
        self.env.global_val = val;
    }
    pub fn guarded_set_global_val<'b, 'c>(
        &'a mut self,
        val: &'b SharedValue<'c>,
    ) -> GlobalValGuard<'a> {
        unsafe {
            self.unsafe_set_global_val(val);
        }
        GlobalValGuard { inter: self }
    }
    pub unsafe fn unsafe_set_global_val<'b, 'c>(&mut self, val: &'b SharedValue<'c>) {
        self.env.global_val = mem::transmute(val);
    }
    pub fn unset_global_val(&mut self) -> &'a SharedValue<'a> {
        mem::replace(&mut self.env.global_val, &DEFAULT_GLOBAL_VAL)
    }
}

pub struct GlobalValGuard<'a> {
    inter: &'a mut Interpreter<'a>,
}

impl<'a> Drop for GlobalValGuard<'a> {
    fn drop(&mut self) {
        self.inter.unset_global_val();
    }
}

impl<'a> Deref for GlobalValGuard<'a> {
    type Target = Interpreter<'a>;

    fn deref(&self) -> &Self::Target {
        self.inter
    }
}

impl<'a> DerefMut for GlobalValGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inter
    }
}
