use dovahkiin::integrated::lisp;
use dovahkiin::lexer::{lisp as lisp_lexer};
use dovahkiin::parser::{lisp as lisp_parser};
use dovahkiin::types::Value;
use dovahkiin::expr::SExpr;

extern crate dovahkiin;

#[test]
pub fn lisp_integrated_plus_function() {
    let interpreter = lisp::get_interpreter();
    let plus_function = " (+ -1i32 1i32 15i32)";
    let tokens = lisp_lexer::tokenize_str(plus_function).unwrap();
    println!("{:?}", tokens);
    println!("{:?}", lisp_parser::parse_to_sexpr(tokens).unwrap());
    assert_eq!(lisp::eval_string(&interpreter, plus_function).unwrap(),
               SExpr::Value(Value::I32(15)));
}

#[test]
pub fn lisp_integrated_binding() {
    let interpreter = lisp::get_interpreter();
    let inc_function = " (let [x 1u32] (+ 1u32 x))";
    let tokens = lisp_lexer::tokenize_str(inc_function).unwrap();
    println!("{:?}", tokens);
    println!("{:?}", lisp_parser::parse_to_sexpr(tokens).unwrap());
    assert_eq!(lisp::eval_string(&interpreter, inc_function).unwrap(),
               SExpr::Value(Value::U32(2)));
}