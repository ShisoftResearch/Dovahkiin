use crate::expr::interpreter::Interpreter;
use crate::expr::SExpr;
use crate::lexer::lisp as lisp_lexer;
use crate::parser::lisp as lisp_parser;

use crate::expr::serde::Expr;

pub fn parse_to_sexpr<'a, 'b>(code: &'b str) -> Result<Vec<SExpr<'a>>, String> {
    let tokens = lisp_lexer::tokenize_str(code)?;
    lisp_parser::SExprParser::parse_to_expr(tokens)
}

pub fn parse_to_serde_expr<'b>(code: &'b str) -> Result<Vec<Expr>, String> {
    let tokens = lisp_lexer::tokenize_str(code)?;
    lisp_parser::SerdeExprParser::parse_to_expr(tokens)
}

pub fn get_interpreter<'a>() -> Interpreter<'a> {
    Interpreter::new()
}

pub fn eval<'a>(
    interpreter: &mut Interpreter<'a>,
    exprs: Vec<SExpr<'a>>,
) -> Result<SExpr<'a>, String> {
    interpreter.eval(exprs)
}

pub fn eval_string<'a, 'b>(
    interpreter: &mut Interpreter<'a>,
    code: &'b str,
) -> Result<SExpr<'a>, String> {
    eval(interpreter, parse_to_sexpr(code)?)
}
