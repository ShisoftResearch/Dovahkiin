use types::Value;

#[macro_use]
pub mod symbols;
pub mod interpreter;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        match self {
            SExpr::List(exprs) => {
                if exprs.len() == 0 {
                    Ok(SExpr::Value(Value::Null))
                } else {
                    let mut iter = exprs.into_iter();
                    let func = iter.next().unwrap().eval()?;
                    Ok(symbols::functions::eval_function(&func, iter.collect())?)
                }
            },
            _ => Ok(self)
        }
    }
}
