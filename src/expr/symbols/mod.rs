use expr::SExpr;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use bifrost_hasher::hash_str;

pub mod functions;
pub mod misc;
mod num_types;
mod arithmetic;
mod bindings;
mod lambda;
mod stream;
mod utils;
mod branching;
mod comparators;

pub trait Symbol: Sync + Debug {
    fn eval(&self, exprs: Vec<SExpr>) -> Result<SExpr, String>;
    fn is_macro(&self) -> bool;
}

macro_rules! defsymbols {
    ($($sym: expr => $name: ident, $is_macro: expr, $eval: expr);*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Symbol for $name {
                fn eval(&self, exprs: Vec<SExpr>) -> Result<SExpr, String> where Self: Sized {
                    $eval(exprs)
                }
                fn is_macro(&self) -> bool {
                    return $is_macro;
                }
            }
        )*
        lazy_static! {
            pub static ref ISYMBOL_MAP: HashMap<u64, Box<Symbol>> = {
                let mut symbol_map: HashMap<u64, Box<Symbol>> = HashMap::new();
                $(
                    symbol_map.insert(hash_str($sym), Box::new($name));
                )*
                symbol_map
            };
        }
    };
}

fn check_num_params(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if num != params.len() {
        Err(format!("Parameter number not match. Except {} but found {}", num, params.len()))
    } else {
        Ok(())
    }
}

fn check_params_not_empty(params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() == 0 {
        Err(format!("Parameter number not match. Expected some but found empty"))
    } else {
        Ok(())
    }
}

fn check_params_not_least_than(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() < num {
        Err(format!("Parameter number not match, Expected at least {} but found {}", num, params.len()))
    } else {
        Ok(())
    }
}

fn check_params_not_greater_than(num: usize, params: &Vec<SExpr>) -> Result<(), String> {
    if params.len() > num {
        Err(format!("Parameter number not match, Expected at most {} but found {}", num, params.len()))
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
    "if" => If, true, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        check_params_not_greater_than(3, &exprs)?;
        branching::if_(exprs)
    };
    "if-not" => IfNot, true, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        check_params_not_greater_than(3, &exprs)?;
        branching::if_not(exprs)
    };
    "when" => When, true, |exprs| {
        check_num_params(2, &exprs)?;
        branching::when(exprs)
    };
    "when-not" => WhenNot, true, |exprs| {
        check_num_params(2, &exprs)?;
        branching::when_not(exprs)
    };
    "=" => Equals, false, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        comparators::equals(exprs)
    };
    "!=" => NotEquals, false, |exprs| {
        check_num_params(2, &exprs)?;
        comparators::not_equals(exprs)
    };
    ">" => GreaterThan, false, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gt(exprs)
    };
    ">=" => GreaterThanEquals, false, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gte(exprs)
    };
    "<" => LessThan, false, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lt(exprs)
    };
    "<=" => LessThanEquals, false, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lte(exprs)
    };
    "+" => Add, false, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::add(exprs)
    };
    "-" => Subtract, false, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::subtract(exprs)
    };
    "*" => Multiply, false, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::multiply(exprs)
    };
    "/" => Divide, false, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::divide(exprs)
    };
    "let" => Let, true, |exprs| {
        bindings::let_binding(exprs)
    };
    "lambda" => Lambda, true, |exprs| {
        check_params_not_least_than(2, &exprs)?;
        lambda::lambda_placeholder(exprs)
    };
    "defunc" => DefineFunc, true, |exprs| {
        check_params_not_least_than(3, &exprs)?;
        functions::defn(exprs)
    };
    "def" => Define, true, |exprs| {
        check_num_params(2, &exprs)?;
        bindings::define(exprs)
    };
    "map" => Map, false, |exprs| {
        check_num_params(2, &exprs)?;
        let (func, data) = split_pair(exprs);
        stream::map(func, data)
    };
    "filter" => Filter, false, |exprs| {
        check_num_params(2, &exprs)?;
        let (func, data) = split_pair(exprs);
        stream::filter(func, data)
    };
    "do" => Do, false, |exprs| {
        misc::do_(exprs)
    };
    "to_vec" => ToVec, false, |mut exprs| {
        check_num_params(1, &exprs)?;
        stream::to_vec(exprs.pop().unwrap())
    };
    "to_array" => ToArray, false, |mut exprs| {
        check_num_params(1, &exprs)?;
        stream::to_array(exprs.pop().unwrap())
    };
    "inc" => Inc, false, |mut exprs| {
        check_num_params(1, &exprs)?;
        arithmetic::inc(exprs.pop().unwrap())
    };
    "u8" => U8, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u8(exprs.get(0).cloned().unwrap())
    };
    "u16" => U16, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u16(exprs.get(0).cloned().unwrap())
    };
    "u32" => U32, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u32(exprs.get(0).cloned().unwrap())
    };
    "u64" => U64, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u64(exprs.get(0).cloned().unwrap())
    };
    "i8" => I8, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::i8(exprs.get(0).cloned().unwrap())
    };
    "i16" => I16, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::i16(exprs.get(0).cloned().unwrap())
    };
    "i32" => I32, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::i32(exprs.get(0).cloned().unwrap())
    };
    "i64" => I64, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::i64(exprs.get(0).cloned().unwrap())
    };
    "f32" => F32, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::f32(exprs.get(0).cloned().unwrap())
    };
    "f64" => F64, false, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::f64(exprs.get(0).cloned().unwrap())
    }
}