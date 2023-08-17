use bifrost_hasher::hash_str;
use crate::types::OwnedValue;

use crate::expr::Value;

use super::{SExpr, symbols::ParserExpr};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Expr {
    Symbol(u64, String),
    Value(OwnedValue),
    List(Vec<Expr>),
    Vec(Vec<Expr>),
}

impl Expr {
    pub fn from_sexpr<'a>(sexpr: SExpr<'a>) -> Self {
        match sexpr {
            SExpr::Symbol(s) => Self::Symbol(hash_str(&s), s),
            SExpr::ISymbol(i, s) => Self::Symbol(i, s),
            SExpr::Value(v) => Self::Value(match v {
                Value::Owned(o) => o,
                Value::Shared(s) => s.owned(),
            }),
            SExpr::List(l) => Self::List(l.into_iter().map(|e| Expr::from_sexpr(e)).collect()),
            SExpr::Vec(v) => Self::Vec(v.into_iter().map(|e| Expr::from_sexpr(e)).collect()),
            SExpr::LAMBDA(_, _) => unreachable!(),
        }
    }

    pub fn to_sexpr<'a>(self) -> SExpr<'a> {
        match self {
            Expr::Symbol(i, s) => SExpr::ISymbol(i, s),
            Expr::Value(v) => SExpr::Value(Value::Owned(v)),
            Expr::List(l) => SExpr::List(l.into_iter().map(|e| e.to_sexpr()).collect()),
            Expr::Vec(v) => SExpr::Vec(v.into_iter().map(|e| e.to_sexpr()).collect()),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            &Expr::List(ref l) => l.is_empty(),
            &Expr::Vec(ref v) => v.is_empty(),
            _ => false,
        }
    }

    pub fn nothing() -> Self {
      Self::List(vec![])
    }
}

impl ParserExpr for Expr {
    fn list(data: Vec<Self>) -> Self {
        Self::List(data)
    }

    fn vec(data: Vec<Self>) -> Self {
        Self::Vec(data)
    }

    fn symbol(name: String) -> Self {
        Self::Symbol(hash_str(&name), name)
    }

    fn owned_val(val: OwnedValue) -> Self {
        Self::Value(val)
    }
}