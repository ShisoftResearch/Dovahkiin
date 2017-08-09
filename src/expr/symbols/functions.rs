use super::*;
use super::lambda::eval_lambda;
use super::super::interpreter::ENV;

pub fn eval_function(func_expr: &SExpr, exprs: Vec<SExpr>) -> Result<SExpr, String> {
    match func_expr {
        &SExpr::ISymbol(symbol_id) => {
            ENV.with(|env| {

            });
            unimplemented!();
        },
        &SExpr::Symbol(ref symbol_name) =>
            return eval_function(&SExpr::ISymbol(hash_str(symbol_name)), exprs),
        _ => return Err(format!("{:?} is not a function", func_expr))
    }
}