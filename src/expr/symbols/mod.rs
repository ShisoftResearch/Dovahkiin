use types::Value;
use expr::interpreter::Envorinment;
use std::collections::HashMap;
use std::fmt::Debug;

mod num_types;
mod arithmetic;

lazy_static! {
    pub static ref ISYMBOL_MAP: HashMap<u64, Box<Symbol>> = HashMap::new();
}

pub trait Symbol: Sync + Debug {
    fn eval(values: Vec<Value>) -> Result<Value, String> where Self: Sized;
}

macro_rules! defsymbols {
    ($($sym: expr => $name: ident, $eval: expr);*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Symbol for $name {
                fn eval(values: Vec<Value>) -> Result<Value, String> where Self: Sized {
                    $eval(values)
                }
            }
        )*
    };
}

fn check_num_params(num: usize, params: &Vec<Value>) -> Result<(), String> {
    if num != params.len() {
        Err(format!("Parameter number not match. Except {} but found {}", num, params.len()))
    } else {
        Ok(())
    }
}

fn check_params_not_empty(params: &Vec<Value>) -> Result<(), String> {
    if params.len() == 0 {
        Err(format!("Parameter number not match, Expected some but found empty"))
    } else {
        Ok(())
    }
}

defsymbols! {
    "+" => Add, |vals| {
        check_params_not_empty(&vals)?;
        arithmetic::add(vals)
    };
    "-" => Subtract, |vals| {
        check_params_not_empty(&vals)?;
        arithmetic::subtract(vals)
    };
    "*" => Multiply, |vals| {
        check_params_not_empty(&vals)?;
        arithmetic::multiply(vals)
    };
    "/" => Divide, |vals| {
        check_params_not_empty(&vals)?;
        arithmetic::divide(vals)
    };
    "u8" => U8, |vals| {
        check_num_params(1, &vals)?;
        num_types::u8(vals.get(0).cloned().unwrap())
    };
    "u16" => U16, |vals| {
        check_num_params(1, &vals)?;
        num_types::u16(vals.get(0).cloned().unwrap())
    };
    "u32" => U32, |vals| {
        check_num_params(1, &vals)?;
        num_types::u32(vals.get(0).cloned().unwrap())
    };
    "u64" => U64, |vals| {
        check_num_params(1, &vals)?;
        num_types::u64(vals.get(0).cloned().unwrap())
    };
    "f32" => F32, |vals| {
        check_num_params(1, &vals)?;
        num_types::f32(vals.get(0).cloned().unwrap())
    };
    "f64" => F64, |vals| {
        check_num_params(1, &vals)?;
        num_types::f64(vals.get(0).cloned().unwrap())
    }
}