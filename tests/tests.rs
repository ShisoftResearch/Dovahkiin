use dovahkiin::integrated::lisp;
use dovahkiin::lexer::{lisp as lisp_lexer};
use dovahkiin::parser::{lisp as lisp_parser};
use dovahkiin::types::Value;
use dovahkiin::expr::SExpr;

extern crate dovahkiin;

#[test]
pub fn lisp_integrated() {
    let interpreter = lisp::get_interpreter();
    let one_plus_one = " (+ -1i32 1i32 15i32)";
    let tokens = lisp_lexer::tokenize_str(one_plus_one).unwrap();
    println!("{:?}", tokens);
    println!("{:?}", lisp_parser::parse_to_sexpr(tokens).unwrap());
    assert_eq!(lisp::eval_string(&interpreter, one_plus_one).unwrap(),
               SExpr::Value(Value::I32(15)));
}
