use types::Value;

#[macro_use]
pub mod symbols;
pub mod interpreter;

#[derive(Debug)]
pub enum SExpr {
    Symbol (String),
    Value(Value),
    List(Vec<SExpr>),
    Vec(Vec<SExpr>)
}

