use super::*;

pub fn is_true(expr: &SExpr) -> bool {
    match expr.val() {
        Some(SharedValue::Bool(false)) | Some(SharedValue::Null) => false,
        _ => true, // anything else than false and null value will be considered as yes
    }
}
