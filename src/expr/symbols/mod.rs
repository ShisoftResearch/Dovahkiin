use expr::interpreter::Envorinment;
use expr::SExpr;
use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::Debug;
use bifrost_hasher::hash_str;

mod num_types;
mod arithmetic;



pub trait Symbol: Sync + Debug {
    fn eval(exprs: Vec<SExpr>) -> Result<SExpr, String> where Self: Sized;
}

macro_rules! defsymbols {
    ($($sym: expr => $name: ident, $eval: expr);*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Symbol for $name {
                fn eval(exprs: Vec<SExpr>) -> Result<SExpr, String> where Self: Sized {
                    $eval(exprs)
                }
            }
        )*
        lazy_static! {
            pub static ref ISYMBOL_MAP: HashMap<u64, Box<Symbol>> = {
                let mut symbol_map: HashMap<u64, Box<Symbol>> = HashMap::new();
                $(
                    symbol_map.insert(hash_str(stringify!($sym)), Box::new($name));
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
        Err(format!("Parameter number not match, Expected some but found empty"))
    } else {
        Ok(())
    }
}

defsymbols! {
    "+" => Add, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::add(exprs)
    };
    "-" => Subtract, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::subtract(exprs)
    };
    "*" => Multiply, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::multiply(exprs)
    };
    "/" => Divide, |exprs| {
        check_params_not_empty(&exprs)?;
        arithmetic::divide(exprs)
    };
    "u8" => U8, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u8(exprs.get(0).cloned().unwrap())
    };
    "u16" => U16, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u16(exprs.get(0).cloned().unwrap())
    };
    "u32" => U32, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u32(exprs.get(0).cloned().unwrap())
    };
    "u64" => U64, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::u64(exprs.get(0).cloned().unwrap())
    };
    "f32" => F32, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::f32(exprs.get(0).cloned().unwrap())
    };
    "f64" => F64, |exprs| {
        check_num_params(1, &exprs)?;
        num_types::f64(exprs.get(0).cloned().unwrap())
    }
}