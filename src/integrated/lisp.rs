use expr::interpreter::Interpreter;
use expr::SExpr;
use lexer::lisp as lisp_lexer;
use parser::lisp as lisp_parser;

pub fn parse_to_expr<'a>(code: &'a str) -> Result<Vec<SExpr>, String> {
    let tokens = lisp_lexer::tokenize_str(code)?;
    lisp_parser::parse_to_sexpr(tokens)
}

pub fn get_interpreter() -> Interpreter {
    Interpreter::new()
}

pub fn eval(interpreter: &Interpreter, exprs: Vec<SExpr>) -> Result<SExpr, String> {
    interpreter.eval(exprs)
}

pub fn eval_string<'a>(interpreter: &Interpreter, code: &'a str) -> Result<SExpr, String> {
    eval(interpreter, parse_to_expr(code)?)
}
