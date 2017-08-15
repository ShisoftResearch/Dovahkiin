use dovahkiin::integrated::lisp;
use dovahkiin::lexer::{lisp as lisp_lexer};
use dovahkiin::parser::{lisp as lisp_parser};
use dovahkiin::types::Value;
use dovahkiin::expr::SExpr;

extern crate dovahkiin;

#[test]
pub fn lisp_integrated_plus_function() {
    let interpreter = lisp::get_interpreter();
    let str_function = " (+ -1i32 1i32 15i32)";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Value(Value::I32(15)));
}

#[test]
pub fn lisp_integrated_binding() {
    let interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32] (+ 1u32 x))";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Value(Value::U32(2)));
}

#[test]
pub fn lisp_integrated_binding_2() {
    let interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32 y 2u32] (+ x y))";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Value(Value::U32(3)));
}

#[test]
pub fn lisp_integrated_lambda() {
    let interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x] (+ 1u32 x)) 5u32)";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Value(Value::U32(6)));
}

#[test]
pub fn lisp_integrated_lambda_2() {
    let interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x y] (* x y)) 5u32 4u32)";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Value(Value::U32(20)));
}

#[test]
pub fn lisp_integrated_functional() {
    let interpreter = lisp::get_interpreter();
    let str_function = "(map (lambda [x] (+ x 1u32)) [1u32 2u32 3u32 4u32 5u32 6u32])";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Vec(vec![SExpr::Value(Value::U32(2)), SExpr::Value(Value::U32(3)),
                               SExpr::Value(Value::U32(4)), SExpr::Value(Value::U32(5)),
                               SExpr::Value(Value::U32(6)), SExpr::Value(Value::U32(7))]));
}

#[test]
pub fn lisp_integrated_functional_symbolic() {
    let interpreter = lisp::get_interpreter();
    let str_function = " (map inc [1u32 2u32 3u32 4u32 5u32 6u32])";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Vec(vec![SExpr::Value(Value::U32(2)), SExpr::Value(Value::U32(3)),
                               SExpr::Value(Value::U32(4)), SExpr::Value(Value::U32(5)),
                               SExpr::Value(Value::U32(6)), SExpr::Value(Value::U32(7))]));
}

#[test]
pub fn lisp_integrated_functional_defunc() {
    let interpreter = lisp::get_interpreter();
    let str_function = "(defunc dec [x] (- x 1u32))\
                             (map dec [1u32 2u32 3u32 4u32 5u32 6u32])";
    let tokens = lisp_lexer::tokenize_str(str_function).unwrap();
    assert_eq!(lisp::eval_string(&interpreter, str_function).unwrap(),
               SExpr::Vec(vec![SExpr::Value(Value::U32(0)), SExpr::Value(Value::U32(1)),
                               SExpr::Value(Value::U32(2)), SExpr::Value(Value::U32(3)),
                               SExpr::Value(Value::U32(4)), SExpr::Value(Value::U32(5))]));
}