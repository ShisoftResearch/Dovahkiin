use bifrost_hasher::hash_str;
use crate::expr::SExpr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

use super::interpreter::Envorinment;
pub use super::*;

mod arithmetic;
pub mod bindings;
mod branching;
mod collections;
mod comparators;
pub mod functions;
mod lambda;
mod logic;
pub mod misc;
mod num_types;
mod stream;
pub mod utils;

pub trait Symbol: Sync + Debug {
    fn eval<'a>(
        &self,
        exprs: Vec<SExpr<'a>>,
        env: &mut Envorinment<'a>,
    ) -> Result<SExpr<'a>, String>;
    fn is_macro(&self) -> bool;
}

pub struct ISymbolMap {
    pub map: RefCell<HashMap<u64, Box<dyn Symbol>>>,
}

unsafe impl Sync for ISymbolMap {}
impl ISymbolMap {
    pub fn new(map: HashMap<u64, Box<dyn Symbol>>) -> ISymbolMap {
        ISymbolMap {
            map: RefCell::new(map),
        }
    }
    pub fn insert<'a, S>(&self, symbol_name: &'a str, symbol_impl: S) -> Result<(), ()>
    where
        S: Symbol + 'static,
    {
        match self.map.try_borrow_mut() {
            Ok(ref mut m) => {
                m.insert(hash_str(symbol_name), Box::new(symbol_impl));
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}

macro_rules! defsymbols {
    ($($sym: expr => $name: ident, $is_macro: expr, $eval: expr);*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Symbol for $name {
                fn eval<'a>(&self, exprs: Vec<SExpr<'a>>, env: &mut Envorinment<'a>) -> Result<SExpr<'a>, String> where Self: Sized {
                    $eval(exprs, env)
                }
                fn is_macro(&self) -> bool {
                    return $is_macro;
                }
            }
        )*
        lazy_static! {
            pub static ref ISYMBOL_MAP: ISymbolMap = {
                let mut symbol_map: HashMap<u64, Box<dyn Symbol>> = HashMap::new();
                $(
                    symbol_map.insert(hash_str($sym), Box::new($name));
                )*
                ISymbolMap::new(symbol_map)
            };
        }
    };
}

pub fn new_symbol<'a, S>(symbol_id: &'a str, symbol_impl: S) -> Result<(), ()>
where
    S: Symbol + 'static,
{
    ISYMBOL_MAP.insert(symbol_id, symbol_impl)
}

fn check_num_params(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if num != params.len() {
        Err(format!(
            "Parameter number not match. Except {} but found {}",
            num,
            params.len()
        ))
    } else {
        Ok(())
    }
}

fn check_params_not_empty(params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() == 0 {
        Err(format!(
            "Parameter number not match. Expected some but found empty"
        ))
    } else {
        Ok(())
    }
}

fn check_params_not_least_than(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() < num {
        Err(format!(
            "Parameter number not match, Expected at least {} but found {}",
            num,
            params.len()
        ))
    } else {
        Ok(())
    }
}

fn check_params_not_greater_than(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() > num {
        Err(format!(
            "Parameter number not match, Expected at most {} but found {}",
            num,
            params.len()
        ))
    } else {
        Ok(())
    }
}

fn split_pair(mut exprs: Vec<SExpr>) -> (SExpr, SExpr) {
    let e2 = exprs.pop().unwrap();
    let e1 = exprs.pop().unwrap();
    (e1, e2)
}

defsymbols! {
    "if" => If, true, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        check_params_not_greater_than(3, &exprs)?;
        branching::if_(env, exprs)
    };
    "if-not" => IfNot, true, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        check_params_not_greater_than(3, &exprs)?;
        branching::if_not(env, exprs)
    };
    "when" => When, true, |exprs, env| {
        check_num_params(2, &exprs)?;
        branching::when(env,exprs)
    };
    "when-not" => WhenNot, true, |exprs, env| {
        check_num_params(2, &exprs)?;
        branching::when_not(env,exprs)
    };
    "=" => Equals, false, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::equals(exprs)
    };
    "!=" => NotEquals, false, |exprs, env| {
        check_num_params(2, &exprs)?;
        comparators::not_equals(exprs)
    };
    ">" => GreaterThan, false, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gt(exprs)
    };
    ">=" => GreaterThanEquals, false, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gte(exprs)
    };
    "<" => LessThan, false, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lt(exprs)
    };
    "<=" => LessThanEquals, false, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lte(exprs)
    };
    "+" => Add, false, |exprs, env| {
        check_params_not_empty(&exprs)?;
        arithmetic::add(exprs)
    };
    "-" => Subtract, false, |exprs, env| {
        check_params_not_empty(&exprs)?;
        arithmetic::subtract(exprs)
    };
    "*" => Multiply, false, |exprs, env| {
        check_params_not_empty(&exprs)?;
        arithmetic::multiply(exprs)
    };
    "/" => Divide, false, |exprs, env| {
        check_params_not_empty(&exprs)?;
        arithmetic::divide(exprs)
    };
    "let" => Let, true, |exprs, env| {
        bindings::let_binding(env, exprs)
    };
    "lambda" => Lambda, true, |exprs, env| {
        check_params_not_least_than(2, &exprs)?;
        lambda::lambda_placeholder(exprs)
    };
    "defunc" => DefineFunc, true, |exprs, env| {
        check_params_not_least_than(3, &exprs)?;
        functions::defn(env, exprs)
    };
    "def" => Define, true, |exprs, env| {
        check_num_params(2, &exprs)?;
        bindings::define(env, exprs)
    };
    "map" => Map, false, |exprs, env| {
        check_num_params(2, &exprs)?;
        let (func, data) = split_pair(exprs);
        stream::map(func, data, env)
    };
    "filter" => Filter, false, |exprs, env| {
        check_num_params(2, &exprs)?;
        let (func, data) = split_pair(exprs);
        stream::filter(func, data, env)
    };
    "do" => Do, false, |exprs, env| {
        misc::do_(exprs, env)
    };
    "to_vec" => ToVec, false, |mut exprs, env| {
        check_num_params(1, &exprs)?;
        stream::to_vec(exprs.pop().unwrap())
    };
    "to_array" => ToArray, false, |mut exprs, env| {
        check_num_params(1, &exprs)?;
        stream::to_array(exprs.pop().unwrap())
    };
    "inc" => Inc, false, |mut exprs, env| {
        check_num_params(1, &exprs)?;
        arithmetic::inc(exprs.pop().unwrap())
    };
    "concat" => Concat, false, |exprs, env| {
        collections::concat(exprs)
    };
    "size" => Size, false, |exprs, env| {
        collections::size(exprs)
    };
    "hash-map" => GenHashMap, false, |exprs, env| {
        collections::hashmap(exprs)
    };
    "merge" => MergeHashMap, false, |exprs, env| {
        collections::merge(exprs)
    };
    "conj" => Conjuction, false, |exprs, env| {
        collections::conj(exprs)
    };
    "or" => Or, true, |exprs, env| {
        logic::or(exprs, env)
    };
    "and" => And, true, |exprs, env| {
        logic::and(exprs, env)
    };
    "cond" => Conditional, true, |exprs, env| {
        logic::cond(exprs, env)
    };
    "u8" => U8, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::u8(exprs.get(0).cloned().unwrap())
    };
    "u16" => U16, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::u16(exprs.get(0).cloned().unwrap())
    };
    "u32" => U32, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::u32(exprs.get(0).cloned().unwrap())
    };
    "u64" => U64, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::u64(exprs.get(0).cloned().unwrap())
    };
    "i8" => I8, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::i8(exprs.get(0).cloned().unwrap())
    };
    "i16" => I16, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::i16(exprs.get(0).cloned().unwrap())
    };
    "i32" => I32, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::i32(exprs.get(0).cloned().unwrap())
    };
    "i64" => I64, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::i64(exprs.get(0).cloned().unwrap())
    };
    "f32" => F32, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::f32(exprs.get(0).cloned().unwrap())
    };
    "f64" => F64, false, |exprs, env| {
        check_num_params(1, &exprs)?;
        num_types::f64(exprs.get(0).cloned().unwrap())
    }
}
