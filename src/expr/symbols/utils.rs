use super::*;
use types::Value;

pub fn is_true (expr: SExpr) -> bool {
    match expr {
        SExpr::Value(Value::Bool(false)) | SExpr::Value(Value::Null) => false,
        _ => true // anything else than false and null value will be considered as yes
    }
}