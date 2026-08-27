pub mod numeric;
use bifrost_hasher::hash_str;
use bifrost_plugins::hash_ident;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::Debug;

use super::interpreter::Environment;
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
        env: &mut Environment<'a>,
    ) -> Result<SExpr<'a>, String>;
    fn is_macro(&self) -> bool;
}

pub struct ISymbolMap {
    map: UnsafeCell<HashMap<u64, Box<dyn Symbol>>>,
}

// SAFETY: Reads are lock-free and assume the symbol table is immutable after initialization.
// Callers may mutate the table only through the documented unsafe APIs, which require exclusive
// initialization-time access with no concurrent readers or writers.
unsafe impl Sync for ISymbolMap {}

impl ISymbolMap {
    pub fn new(map: HashMap<u64, Box<dyn Symbol>>) -> ISymbolMap {
        ISymbolMap {
            map: UnsafeCell::new(map),
        }
    }

    pub fn get(&self, symbol_id: u64) -> Option<&dyn Symbol> {
        // SAFETY: Readers only take a shared reference to the map. This is sound if and only if
        // callers uphold the registry invariant that no unsafe mutation happens concurrently with
        // reads, and that the registry is not mutated after initialization.
        unsafe {
            (&*self.map.get())
                .get(&symbol_id)
                .map(|symbol| symbol.as_ref())
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee there are no concurrent readers or writers of the symbol map.
    /// This is intended for single-threaded initialization before the interpreter is used.
    pub unsafe fn insert<'a, S>(&self, symbol_name: &'a str, symbol_impl: S)
    where
        S: Symbol + 'static,
    {
        let map = unsafe { &mut *self.map.get() };
        map.insert(hash_str(symbol_name), Box::new(symbol_impl));
    }
}

macro_rules! defsymbols {
    ($($sym: expr => $name: ident, $is_macro: expr, $eval: expr);*) => {
        $(
            #[derive(Debug)]
            pub struct $name;
            impl Symbol for $name {
                fn eval<'a>(&self, exprs: Vec<SExpr<'a>>, env: &mut Environment<'a>) -> Result<SExpr<'a>, String> where Self: Sized {
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
                    symbol_map.insert(hash_ident!($sym), Box::new($name));
                )*
                ISymbolMap::new(symbol_map)
            };
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum SysSymbol {
            $(
                $name = hash_ident!($sym),
            )*
        }
        impl SysSymbol {
            pub fn from_id(id: u64) -> Option<Self> {
                match id {
                    $(
                        hash_ident!($sym) => Some(Self::$name),
                    )*
                    _ => None
                }
            }
        }
    };
}

/// # Safety
///
/// The caller must guarantee there are no concurrent readers or writers of the global symbol
/// registry. This should only be used during interpreter initialization.
pub unsafe fn new_symbol<'a, S>(symbol_name: &'a str, symbol_impl: S)
where
    S: Symbol + 'static,
{
    unsafe { ISYMBOL_MAP.insert(symbol_name, symbol_impl) }
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
    "=" => Equals, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::equals(exprs)
    };
    "!=" => NotEquals, false, |exprs, _env| {
        check_num_params(2, &exprs)?;
        comparators::not_equals(exprs)
    };
    "in" => In, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::in_(exprs)
    };
    "between" => Between, false, |exprs, _env| {
        check_num_params(3, &exprs)?;
        comparators::between(exprs)
    };
    "is-null" => IsNull, true, |exprs, env| {
        check_num_params(1, &exprs)?;
        logic::is_null(exprs, env)
    };
    "is-not-null" => IsNotNull, true, |exprs, env| {
        check_num_params(1, &exprs)?;
        logic::is_not_null(exprs, env)
    };
    ">" => GreaterThan, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gt(exprs)
    };
    ">=" => GreaterThanEquals, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::gte(exprs)
    };
    "<" => LessThan, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lt(exprs)
    };
    "<=" => LessThanEquals, false, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        comparators::lte(exprs)
    };
    "+" => Add, false, |exprs, _env| {
        check_params_not_empty(&exprs)?;
        arithmetic::add(exprs)
    };
    "-" => Subtract, false, |exprs, _env| {
        check_params_not_empty(&exprs)?;
        arithmetic::subtract(exprs)
    };
    "*" => Multiply, false, |exprs, _env| {
        check_params_not_empty(&exprs)?;
        arithmetic::multiply(exprs)
    };
    "/" => Divide, false, |exprs, _env| {
        check_params_not_empty(&exprs)?;
        arithmetic::divide(exprs)
    };
    "abs" => Abs, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::abs(exprs.pop().unwrap())
    };
    "sqrt" => Sqrt, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::sqrt(exprs.pop().unwrap())
    };
    "ln" => Ln, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::ln(exprs.pop().unwrap())
    };
    "log2" => Log2, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::log2(exprs.pop().unwrap())
    };
    "log10" => Log10, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::log10(exprs.pop().unwrap())
    };
    "exp" => Exp, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::exp(exprs.pop().unwrap())
    };
    "floor" => Floor, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::floor(exprs.pop().unwrap())
    };
    "ceil" => Ceil, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::ceil(exprs.pop().unwrap())
    };
    "round" => Round, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::round(exprs.pop().unwrap())
    };
    "pow" => Pow, false, |exprs, _env| {
        check_num_params(2, &exprs)?;
        let (base, exp) = split_pair(exprs);
        arithmetic::pow(base, exp)
    };
    "min" => Min, false, |exprs, _env| {
        check_num_params(2, &exprs)?;
        let (lhs, rhs) = split_pair(exprs);
        arithmetic::min(lhs, rhs)
    };
    "max" => Max, false, |exprs, _env| {
        check_num_params(2, &exprs)?;
        let (lhs, rhs) = split_pair(exprs);
        arithmetic::max(lhs, rhs)
    };
    "clamp" => Clamp, false, |mut exprs, _env| {
        check_num_params(3, &exprs)?;
        let max = exprs.pop().unwrap();
        let min = exprs.pop().unwrap();
        let value = exprs.pop().unwrap();
        arithmetic::clamp(value, min, max)
    };
    "let" => Let, true, |exprs, env| {
        bindings::let_binding(env, exprs)
    };
    "lambda" => Lambda, true, |exprs, _env| {
        check_params_not_least_than(2, &exprs)?;
        lambda::lambda_placeholder(exprs.into_iter())
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
    "to_vec" => ToVec, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        stream::to_vec(exprs.pop().unwrap())
    };
    "to_array" => ToArray, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        stream::to_array(exprs.pop().unwrap())
    };
    "inc" => Inc, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        arithmetic::inc(exprs.pop().unwrap())
    };
    "concat" => Concat, false, |exprs, _env| {
        collections::concat(exprs)
    };
    "size" => Size, false, |exprs, _env| {
        collections::size(exprs)
    };
    "into-map" => IntoMap, false, |exprs, _env| {
        collections::map(exprs)
    };
    "merge" => MergeHashMap, false, |exprs, _env| {
        collections::merge(exprs)
    };
    "conj" => Conjuction, false, |exprs, _env| {
        collections::conj(exprs)
    };
    "or" => Or, true, |exprs, env| {
        logic::or(exprs, env)
    };
    "and" => And, true, |exprs, env| {
        logic::and(exprs, env)
    };
    "not" => Not, true, |exprs, env| {
        check_num_params(1, &exprs)?;
        logic::not(exprs, env)
    };
    "cond" => Conditional, true, |exprs, env| {
        logic::cond(exprs, env)
    };
    "u8" => U8, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::u8(exprs.pop().unwrap())
    };
    "u16" => U16, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::u16(exprs.pop().unwrap())
    };
    "u32" => U32, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::u32(exprs.pop().unwrap())
    };
    "u64" => U64, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::u64(exprs.pop().unwrap())
    };
    "i8" => I8, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::i8(exprs.pop().unwrap())
    };
    "i16" => I16, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::i16(exprs.pop().unwrap())
    };
    "i32" => I32, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::i32(exprs.pop().unwrap())
    };
    "i64" => I64, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::i64(exprs.pop().unwrap())
    };
    "f32" => F32, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::f32(exprs.pop().unwrap())
    };
    "f64" => F64, false, |mut exprs, _env| {
        check_num_params(1, &exprs)?;
        num_types::f64(exprs.pop().unwrap())
    }
}
