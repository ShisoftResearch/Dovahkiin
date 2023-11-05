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
    Keyword(u64, String),
    META(Vec<Expr>),
    LAMBDA(Vec<Expr>, Vec<Expr>),
}

fn sexpr_list_to_expr_list(list: Vec<SExpr>) -> Vec<Expr> {
    list.into_iter().map(|e| Expr::from_sexpr(e)).collect()
}

fn expr_list_to_sexpr_list<'a>(list: Vec<Expr>) -> Vec<SExpr<'a>> {
    list.into_iter().map(|e| e.to_sexpr()).collect()
}

impl Expr {
    pub fn from_sexpr<'a>(sexpr: SExpr<'a>) -> Self {
        match sexpr {
            SExpr::Symbol(s) => Self::Symbol(hash_str(&s), s),
            SExpr::ISymbol(i, s) => Self::Symbol(i, s),
            SExpr::Keyword(id, name) => Self::Keyword(id, name),
            SExpr::Value(v) => Self::Value(match v {
                Value::Owned(o) => o,
                Value::Shared(s) => s.owned(),
            }),
            SExpr::List(l) => Self::List(sexpr_list_to_expr_list(l)),
            SExpr::Vec(v) => Self::Vec(sexpr_list_to_expr_list(v)),
            SExpr::META(v) => Self::META(sexpr_list_to_expr_list(v)),
            SExpr::LAMBDA(p, b) => {
                Self::LAMBDA(
                    sexpr_list_to_expr_list(p),
                    sexpr_list_to_expr_list(b)
                )
            },
        }
    }

    pub fn to_sexpr<'a>(self) -> SExpr<'a> {
        match self {
            Expr::Symbol(i, s) => SExpr::ISymbol(i, s),
            Expr::Keyword(id, name) => SExpr::Keyword(id, name),
            Expr::Value(v) => SExpr::Value(Value::Owned(v)),
            Expr::List(l) => SExpr::List(expr_list_to_sexpr_list(l)),
            Expr::Vec(v) => SExpr::Vec(expr_list_to_sexpr_list(v)),
            Expr::META(v) => SExpr::META(expr_list_to_sexpr_list(v)),
            Expr::LAMBDA(p, b) => {
                SExpr::LAMBDA(
                    expr_list_to_sexpr_list(p),
                    expr_list_to_sexpr_list(b)
                )
            },
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

    fn keyword(name: String) -> Self {
        Self::Keyword(hash_str(&name), name)
    }

    fn owned_val(val: OwnedValue) -> Self {
        Self::Value(val)
    }
}