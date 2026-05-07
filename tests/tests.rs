use dovahkiin::expr::SExpr;
use dovahkiin::integrated::lisp;
use dovahkiin::types::{Map, OwnedValue, SharedValue};

extern crate dovahkiin;

#[test]
pub fn lisp_integrated_plus_function() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (+ -1i32 1i32 15i32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::I32(15))
    );
}

#[test]
pub fn lisp_integrated_binding() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32] (+ 1u32 x))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(2))
    );
}

#[test]
pub fn lisp_integrated_binding_2() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " (let [x 1u32 y 2u32] (+ x y))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(3))
    );
}

#[test]
pub fn lisp_integrated_binding_def() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(def x 1u32) (let [y 2u32] (+ x y))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(3))
    );
}

#[test]
pub fn lisp_integrated_lambda() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x] (+ 1u32 x)) 5u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(6))
    );
}

#[test]
pub fn lisp_integrated_lambda_2() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = " ((lambda [x y] (* x y)) 5u32 4u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(20))
    );
}

#[test]
pub fn lisp_integrated_functional() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(map (lambda [x] (+ x 1u32)) [1u32 2u32 3u32 4u32 5u32 6u32])";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::Vec(vec![
            SExpr::from_owned_value(OwnedValue::U32(2)),
            SExpr::from_owned_value(OwnedValue::U32(3)),
            SExpr::from_owned_value(OwnedValue::U32(4)),
            SExpr::from_owned_value(OwnedValue::U32(5)),
            SExpr::from_owned_value(OwnedValue::U32(6)),
            SExpr::from_owned_value(OwnedValue::U32(7))
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
            SExpr::from_owned_value(OwnedValue::U32(2)),
            SExpr::from_owned_value(OwnedValue::U32(3)),
            SExpr::from_owned_value(OwnedValue::U32(4)),
            SExpr::from_owned_value(OwnedValue::U32(5)),
            SExpr::from_owned_value(OwnedValue::U32(6)),
            SExpr::from_owned_value(OwnedValue::U32(7))
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
            SExpr::from_owned_value(OwnedValue::U32(0)),
            SExpr::from_owned_value(OwnedValue::U32(1)),
            SExpr::from_owned_value(OwnedValue::U32(2)),
            SExpr::from_owned_value(OwnedValue::U32(3)),
            SExpr::from_owned_value(OwnedValue::U32(4)),
            SExpr::from_owned_value(OwnedValue::U32(5))
        ])
    );
}

#[test]
pub fn lisp_lexer_test_1() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(+(+ 1u32 2u32) 3u32)";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::U32(6))
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
        lisp::eval_string(&mut interpreter, str_function)
            .unwrap()
            .owned_val()
            .unwrap(),
        OwnedValue::U32(2)
    );
}

#[test]
pub fn or() {
    let mut interpreter = lisp::get_interpreter();
    let str_function = "(let [x 2u64] (or (= x 1u64) (= x 2u64)))";
    assert_eq!(
        lisp::eval_string(&mut interpreter, str_function).unwrap(),
        SExpr::from_owned_value(OwnedValue::Bool(true))
    );
}

#[test]
pub fn keyword() {
    let mut interpreter = lisp::get_interpreter();
    let str_exp = "(into-map [:x 123u32, :y 456u64])";
    let map_expr = lisp::eval_string(&mut interpreter, str_exp).unwrap();
    let map_val = map_expr.shared_val().unwrap();
    let map = map_val.Map().unwrap();
    assert_eq!(map.get("x").u32().unwrap(), &123);
    assert_eq!(map.get("y").u64().unwrap(), &456);
}

#[test]
pub fn map() {
    let mut interpreter = lisp::get_interpreter();
    let str_exp = "{:x 123u32, :y 456u64}";
    let map_expr = lisp::eval_string(&mut interpreter, str_exp).unwrap();
    let map_val = map_expr.shared_val().unwrap();
    let map = map_val.Map().unwrap();
    assert_eq!(map.get("x").u32().unwrap(), &123);
    assert_eq!(map.get("y").u64().unwrap(), &456);
}

#[test]
pub fn map_vec() {
    let mut interpreter = lisp::get_interpreter();
    let str_exp = "{:x 123u32, :y [456u64, 789u64]}";
    let map_expr = lisp::eval_string(&mut interpreter, str_exp).unwrap();
    let map_val = map_expr.shared_val().unwrap();
    let map = map_val.Map().unwrap();
    assert_eq!(map.get("x").u32().unwrap(), &123);
    let y = map.get("y");
    match y {
        SharedValue::Array(arr) => {
            assert_eq!(arr[0].u64().unwrap(), &456);
            assert_eq!(arr[1].u64().unwrap(), &789);
        }
        _ => panic!(),
    }
}

#[test]
pub fn lisp_integrated_math_logarithms_and_exp() {
    let mut interpreter = lisp::get_interpreter();

    assert_eq!(
        lisp::eval_string(&mut interpreter, "(ln 1.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(0.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(log2 8.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(3.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(log10 1000.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(3.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(round (exp 1.0f64))").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(3.0))
    );
}

#[test]
pub fn lisp_integrated_math_helpers() {
    let mut interpreter = lisp::get_interpreter();

    assert_eq!(
        lisp::eval_string(&mut interpreter, "(sqrt 9.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(3.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(pow 3.0f64 2.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(9.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(abs -5i32)").unwrap(),
        SExpr::from_owned_value(OwnedValue::I32(5))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(min 3.0f64 2.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(2.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(max 3.0f64 2.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(3.0))
    );
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(clamp 9.0f64 1.0f64 8.0f64)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F64(8.0))
    );
}
