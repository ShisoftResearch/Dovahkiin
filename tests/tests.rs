use dovahkiin::expr::{SExpr, Value};
use dovahkiin::integrated::lisp;
use dovahkiin::types::OwnedValue;

extern crate dovahkiin;

#[test]
pub fn lisp_integrated_plus_function() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (+ -1i32 1i32 15i32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::I32(15))
    );
}

#[test]
pub fn lisp_integrated_binding() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32] (+ 1u32 x))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(2))
    );
}

#[test]
pub fn lisp_integrated_binding_2() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32 y 2u32] (+ x y))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(3))
    );
}

#[test]
pub fn lisp_integrated_binding_def() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(def x 1u32) (let [y 2u32] (+ x y))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(3))
    );
}

#[test]
pub fn lisp_integrated_lambda() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x] (+ 1u32 x)) 5u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(6))
    );
}

#[test]
pub fn lisp_integrated_lambda_2() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x y] (* x y)) 5u32 4u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(20))
    );
}

#[test]
pub fn lisp_integrated_functional() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(map (lambda [x] (+ x 1u32)) [1u32 2u32 3u32 4u32 5u32 6u32])";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::Vec(vec![
            SExpr::owned_value(OwnedValue::U32(2)),
            SExpr::owned_value(OwnedValue::U32(3)),
            SExpr::owned_value(OwnedValue::U32(4)),
            SExpr::owned_value(OwnedValue::U32(5)),
            SExpr::owned_value(OwnedValue::U32(6)),
            SExpr::owned_value(OwnedValue::U32(7))
        ])
    );
}

#[test]
pub fn lisp_integrated_functional_symbolic() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (map inc [1u32 2u32 3u32 4u32 5u32 6u32])";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::Vec(vec![
            SExpr::owned_value(OwnedValue::U32(2)),
            SExpr::owned_value(OwnedValue::U32(3)),
            SExpr::owned_value(OwnedValue::U32(4)),
            SExpr::owned_value(OwnedValue::U32(5)),
            SExpr::owned_value(OwnedValue::U32(6)),
            SExpr::owned_value(OwnedValue::U32(7))
        ])
    );
}

#[test]
pub fn lisp_integrated_functional_defunc() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(defunc dec [x] (- x 1u32))\
                        (map dec [1u32 2u32 3u32 4u32 5u32 6u32])";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::Vec(vec![
            SExpr::owned_value(OwnedValue::U32(0)),
            SExpr::owned_value(OwnedValue::U32(1)),
            SExpr::owned_value(OwnedValue::U32(2)),
            SExpr::owned_value(OwnedValue::U32(3)),
            SExpr::owned_value(OwnedValue::U32(4)),
            SExpr::owned_value(OwnedValue::U32(5))
        ])
    );
}

#[test]
pub fn lisp_lexer_test_1() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(+(+ 1u32 2u32) 3u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(6))
    );
}

#[test]
pub fn scoping() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(def x 1u32)\n
                        (defunc y [] x)\n
                        (let [x 2u32] (y))";
    // 2 for dynamic scoping, 1 for lexical scoping. Dovahkiin is dynamic scoping
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::owned_value(OwnedValue::U32(2))
    );
}
