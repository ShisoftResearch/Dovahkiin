use types::Value;

#[macro_use]
pub mod symbols;
pub mod interpreter;

#[derive(Debug, Clone)]
pub enum SExpr {
    Symbol (String),
    ISymbol (u64),
    Value(Value),
    List(Vec<SExpr>),
    Vec(Vec<SExpr>),
    LAMBDA(Vec<SExpr>, Vec<SExpr>),
}

impl SExpr {
    pub fn eval(self) -> Result<SExpr, String> {
        // unimplemented!();
        Ok(self)
    }
}
