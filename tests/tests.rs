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

#[test]
fn f16_parser_literals() {
    let mut interpreter = lisp::get_interpreter();
    let mut ev = |src: &str| lisp::eval_string(&mut interpreter, src).unwrap();

    assert_eq!(
        ev("1f16"),
        SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(0x3c00)))
    );
    assert_eq!(
        ev("2.5f16"),
        SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(0x4100)))
    );
    assert_eq!(
        ev("65504f16"),
        SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(0x7bff)))
    );
    assert_eq!(
        ev("0.0000000596f16"),
        SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(0x0001)))
    );
    assert!(
        lisp::eval_string(&mut interpreter, "65520f16").is_err(),
        "65520f16 must overflow"
    );
}

#[test]
fn f16_arithmetic_widens_to_f32() {
    let mut interpreter = lisp::get_interpreter();
    let mut ev = |src: &str| lisp::eval_string(&mut interpreter, src).unwrap();

    assert_eq!(
        ev("(+ 1f16 2f16)"),
        SExpr::from_owned_value(OwnedValue::F32(3.0))
    );
    assert_eq!(
        ev("(* 2f16 3f16)"),
        SExpr::from_owned_value(OwnedValue::F32(6.0))
    );
    assert_eq!(
        ev("(abs -1f16)"),
        SExpr::from_owned_value(OwnedValue::F32(1.0))
    );
    assert_eq!(
        ev("(min 3f16 2f16)"),
        SExpr::from_owned_value(OwnedValue::F32(2.0))
    );
}

#[test]
fn f16_comparisons_and_casts() {
    let mut interpreter = lisp::get_interpreter();
    let mut ev = |src: &str| lisp::eval_string(&mut interpreter, src).unwrap();

    assert_eq!(
        ev("(< 1f16 2f16)"),
        SExpr::from_owned_value(OwnedValue::Bool(true))
    );
    assert_eq!(
        ev("(= 1f16 1.0)"),
        SExpr::from_owned_value(OwnedValue::Bool(true))
    );
    assert_eq!(
        ev("(f16 1.5)"),
        SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(0x3e00)))
    );
    assert_eq!(
        ev("(f32 1f16)"),
        SExpr::from_owned_value(OwnedValue::F32(1.0))
    );
    assert_eq!(
        ev("(f64 1f16)"),
        SExpr::from_owned_value(OwnedValue::F64(1.0))
    );
}

#[test]
fn f16_clamp_rejects_invalid_bounds() {
    let mut interpreter = lisp::get_interpreter();
    interpreter.bind(
        "nan",
        SExpr::from_owned_value(OwnedValue::F16(half::f16::NAN)),
    );
    for source in [
        "(clamp 1f16 2f16 0f16)",
        "(clamp 1f16 nan 2f16)",
        "(clamp 1f16 0f16 nan)",
    ] {
        assert!(lisp::eval_string(&mut interpreter, source).is_err());
    }
    assert_eq!(
        lisp::eval_string(&mut interpreter, "(clamp 3f16 0f16 2f16)").unwrap(),
        SExpr::from_owned_value(OwnedValue::F32(2.0))
    );
}

#[test]
fn f16_literals_round_directly_and_reject_nonfinite() {
    use dovahkiin::lexer::lisp::Token;
    use dovahkiin::parser::lisp::SExprParser;
    let mut interpreter = lisp::get_interpreter();
    for (source, bits) in [
        ("1.00048828125f16", 0x3c00),
        ("1.00146484375f16", 0x3c02),
        ("1.000488282181322574615478515625f16", 0x3c01),
    ] {
        assert_eq!(
            lisp::eval_string(&mut interpreter, source).unwrap(),
            SExpr::from_owned_value(OwnedValue::F16(half::f16::from_bits(bits)))
        );
    }
    assert!(lisp::eval_string(&mut interpreter, "-65520f16").is_err());
    // Feed the numeric token directly because the lexer treats NaN as a symbol.
    assert!(
        SExprParser::parse_to_expr(vec![Token::FloatNumber("NaN".into(), "f16".into())]).is_err()
    );
}
