use types::Value;

#[macro_use]
pub mod symbols;
pub mod interpreter;

#[derive(Debug)]
pub enum SExpr {
    Symbol (String),
    ISymbol (Box<symbols::Symbol>),
    Value(Value),
    List(Vec<SExpr>)
}

