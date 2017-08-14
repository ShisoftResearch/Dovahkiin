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
    assert_eq!(lisp::eval_string(&interpreter, plus_function).unwrap(),
               SExpr::Value(Value::I32(15)));
}

#[test]
pub fn lisp_integrated_binding() {
    let interpreter = lisp::get_interpreter();
    let inc_function = " (let [x 1u32] (+ 1u32 x))";
    let tokens = lisp_lexer::tokenize_str(inc_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, inc_function).unwrap(),
               SExpr::Value(Value::U32(2)));
}

#[test]
pub fn lisp_integrated_binding_2() {
    let interpreter = lisp::get_interpreter();
    let inc_function = " (let [x 1u32 y 2u32] (+ x y))";
    let tokens = lisp_lexer::tokenize_str(inc_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, inc_function).unwrap(),
               SExpr::Value(Value::U32(3)));
}

#[test]
pub fn lisp_integrated_lambda() {
    let interpreter = lisp::get_interpreter();
    let inc_function = " ((lambda [x] (+ 1u32 x)) 5u32)";
    let tokens = lisp_lexer::tokenize_str(inc_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, inc_function).unwrap(),
               SExpr::Value(Value::U32(6)));
}