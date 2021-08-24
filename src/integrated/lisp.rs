use expr::interpreter::Interpreter;
use expr::SExpr;
use lexer::lisp as lisp_lexer;
use parser::lisp as lisp_parser;

pub fn parse_to_expr<'a, 'b>(code: &'b str) -> Result<Vec<SExpr<'a>>, String> {
    let tokens = lisp_lexer::tokenize_str(code)?;
    lisp_parser::parse_to_sexpr(tokens)
}

pub fn get_interpreter<'a>() -> Interpreter<'a> {
    Interpreter::new()
}

pub fn eval<'a>(interpreter: &mut Interpreter<'a>, exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    interpreter.eval(exprs)
}

pub fn eval_string<'a, 'b>(interpreter: &mut Interpreter<'a>, code: &'b str) -> Result<SExpr<'a>, String> {
    eval(interpreter, parse_to_expr(code)?)
}
