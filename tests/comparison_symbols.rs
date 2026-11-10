use dovahkiin::expr::SExpr;
use dovahkiin::integrated::lisp;
use dovahkiin::types::*;

fn eval_bool(source: &str, bindings: &[(&str, OwnedValue)]) -> bool {
    let mut interpreter = lisp::get_interpreter();
    for (name, value) in bindings {
        interpreter.bind(name, SExpr::from_owned_value(value.clone()));
    }
    match lisp::eval_string(&mut interpreter, source)
        .unwrap()
        .owned_val()
    {
        Some(OwnedValue::Bool(value)) => value,
        other => panic!("{source}: expected a bool, found {other:?}"),
    }
}

fn numeric_values(value: u8) -> Vec<OwnedValue> {
    vec![
        OwnedValue::U8(value),
        OwnedValue::U16(value.into()),
        OwnedValue::U32(value.into()),
        OwnedValue::U64(value.into()),
        OwnedValue::I8(value as i8),
        OwnedValue::I16(value.into()),
        OwnedValue::I32(value.into()),
        OwnedValue::I64(value.into()),
        OwnedValue::F16(half::f16::from_f32(value.into())),
        OwnedValue::F32(value.into()),
        OwnedValue::F64(value.into()),
    ]
}

fn value_pairs() -> Vec<(OwnedValue, OwnedValue)> {
    let mut pairs: Vec<_> = numeric_values(1)
        .into_iter()
        .zip(numeric_values(2))
        .collect();
    pairs.extend([
        (OwnedValue::Bool(false), OwnedValue::Bool(true)),
        (OwnedValue::Char('a'), OwnedValue::Char('b')),
        (
            OwnedValue::String("a".into()),
            OwnedValue::String("b".into()),
        ),
        (
            OwnedValue::Id(Id::from_bits(1)),
            OwnedValue::Id(Id::from_bits(2)),
        ),
        (OwnedValue::Null, OwnedValue::NA),
        (OwnedValue::NA, OwnedValue::Null),
        (
            OwnedValue::Bytes(Bytes::from_vec(vec![1])),
            OwnedValue::Bytes(Bytes::from_vec(vec![2])),
        ),
        (
            OwnedValue::SmallBytes(SmallBytes::from_vec(vec![1])),
            OwnedValue::SmallBytes(SmallBytes::from_vec(vec![2])),
        ),
        (
            OwnedValue::Array(vec![OwnedValue::I64(1)]),
            OwnedValue::Array(vec![OwnedValue::I64(2)]),
        ),
        (
            OwnedValue::PrimArray(OwnedPrimArray::U8(vec![1])),
            OwnedValue::PrimArray(OwnedPrimArray::U8(vec![2])),
        ),
        (
            OwnedValue::Map(OwnedMap::from_pairs(
                vec![("x".into(), OwnedValue::I64(1))].into_iter(),
            )),
            OwnedValue::Map(OwnedMap::from_pairs(
                vec![("x".into(), OwnedValue::I64(2))].into_iter(),
            )),
        ),
        (
            OwnedValue::Pos2d32(Pos2d32 { x: 1.0, y: 1.0 }),
            OwnedValue::Pos2d32(Pos2d32 { x: 2.0, y: 1.0 }),
        ),
        (
            OwnedValue::Pos2d64(Pos2d64 { x: 1.0, y: 1.0 }),
            OwnedValue::Pos2d64(Pos2d64 { x: 2.0, y: 1.0 }),
        ),
        (
            OwnedValue::Pos3d32(Pos3d32 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            OwnedValue::Pos3d32(Pos3d32 {
                x: 2.0,
                y: 1.0,
                z: 1.0,
            }),
        ),
        (
            OwnedValue::Pos3d64(Pos3d64 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            OwnedValue::Pos3d64(Pos3d64 {
                x: 2.0,
                y: 1.0,
                z: 1.0,
            }),
        ),
    ]);
    pairs
}

#[test]
fn equality_and_inequality_for_each_value_type() {
    let mut failures = Vec::new();
    for (a, different) in value_pairs() {
        for (b, equal) in [(a.clone(), true), (different, false)] {
            let bindings = [("a", a.clone()), ("b", b.clone())];
            for (source, expected) in [
                ("(= a b)", equal),
                ("(!= a b)", !equal),
                ("(not (= a b))", !equal),
            ] {
                if eval_bool(source, &bindings) != expected {
                    failures.push(format!("{source}, {a:?}, {b:?}: expected {expected}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn promoted_numeric_equality_and_inequality() {
    let mut failures = Vec::new();
    for a in numeric_values(1) {
        for (values, equal) in [(numeric_values(1), true), (numeric_values(2), false)] {
            for b in values {
                let bindings = [("a", a.clone()), ("b", b.clone())];
                for (source, expected) in [
                    ("(= a b)", equal),
                    ("(!= a b)", !equal),
                    ("(not (= a b))", !equal),
                ] {
                    if eval_bool(source, &bindings) != expected {
                        failures.push(format!("{source}, {a:?}, {b:?}: expected {expected}"));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn nary_inequality_negates_all_equal() {
    for (args, equal) in [
        ("1 1 1", true),
        ("1u8 1i32 1.0", true),
        ("1 2 1", false),
        ("1 1 2", false),
        ("2 1 1", false),
        ("1 2 3", false),
        ("1 1 1 2", false),
        ("a a a", true),
        ("a a b", false),
    ] {
        let bindings = [
            ("a", OwnedValue::String("a".into())),
            ("b", OwnedValue::String("b".into())),
        ];
        assert_eq!(
            eval_bool(&format!("(= {args})"), &bindings),
            equal,
            "{args}"
        );
        assert_eq!(
            eval_bool(&format!("(!= {args})"), &bindings),
            !equal,
            "{args}"
        );
        assert_eq!(
            eval_bool(&format!("(not (= {args}))"), &bindings),
            !equal,
            "{args}"
        );
    }
}

#[test]
fn null_and_absent_operands_follow_equality() {
    // Unbound symbols have no value. Null and NA are distinct stored values.
    for (args, equal) in [
        ("missing missing", true),
        ("missing other", true),
        ("missing a", false),
        ("a a", true),
        ("a b", false),
    ] {
        let bindings = [("a", OwnedValue::Null), ("b", OwnedValue::NA)];
        assert_eq!(
            eval_bool(&format!("(= {args})"), &bindings),
            equal,
            "{args}"
        );
        assert_eq!(
            eval_bool(&format!("(!= {args})"), &bindings),
            !equal,
            "{args}"
        );
        assert_eq!(
            eval_bool(&format!("(not (= {args}))"), &bindings),
            !equal,
            "{args}"
        );
    }
}

#[test]
fn numeric_edges_follow_equality() {
    for (a, b, equal) in [
        (OwnedValue::I64(-1), OwnedValue::U64(u64::MAX), false),
        (
            OwnedValue::U64(u64::MAX),
            OwnedValue::U64(u64::MAX - 1),
            false,
        ),
        (OwnedValue::F64(-0.0), OwnedValue::I8(0), true),
        (
            OwnedValue::F32(f32::INFINITY),
            OwnedValue::F64(f64::INFINITY),
            true,
        ),
        (
            OwnedValue::F16(half::f16::NAN),
            OwnedValue::F16(half::f16::NAN),
            false,
        ),
        (OwnedValue::F32(f32::NAN), OwnedValue::F32(f32::NAN), false),
        (OwnedValue::F64(f64::NAN), OwnedValue::I64(1), false),
        (OwnedValue::I64(1), OwnedValue::String("1".into()), false),
    ] {
        let bindings = [("a", a), ("b", b)];
        assert_eq!(eval_bool("(= a b)", &bindings), equal, "{bindings:?}");
        assert_eq!(eval_bool("(!= a b)", &bindings), !equal, "{bindings:?}");
        assert_eq!(
            eval_bool("(!= a b)", &bindings),
            eval_bool("(not (= a b))", &bindings)
        );
    }
}

#[test]
fn ordered_comparisons_for_all_numeric_widths() {
    for a in numeric_values(1) {
        for b in numeric_values(2) {
            for (op, ascending, equal, descending) in [
                ("<", true, false, false),
                ("<=", true, true, false),
                (">", false, false, true),
                (">=", false, true, true),
            ] {
                let bindings = [("a", a.clone()), ("b", b.clone())];
                for (args, expected) in [
                    ("a b", ascending),
                    ("a a", equal),
                    ("b a", descending),
                    ("a b a", false),
                ] {
                    assert_eq!(
                        eval_bool(&format!("({op} {args})"), &bindings),
                        expected,
                        "{op} {args}: {bindings:?}"
                    );
                }
            }
        }
    }
    for (source, expected) in [
        ("(< 1 2 3)", true),
        ("(<= 1 1u8 2.0)", true),
        ("(> 3 2 1)", true),
        ("(>= 3.0 2u8 2)", true),
    ] {
        assert_eq!(eval_bool(source, &[]), expected, "{source}");
    }
}

#[test]
fn unordered_float_comparisons_are_false() {
    for nan in [
        OwnedValue::F16(half::f16::NAN),
        OwnedValue::F32(f32::NAN),
        OwnedValue::F64(f64::NAN),
    ] {
        for b in [nan.clone(), OwnedValue::I64(1)] {
            let bindings = [("a", nan.clone()), ("b", b)];
            for op in ["<", "<=", ">", ">="] {
                assert!(
                    !eval_bool(&format!("({op} a b)"), &bindings),
                    "{op}: {bindings:?}"
                );
            }
        }
    }
}

#[test]
fn logic_truth_tables_and_short_circuit() {
    for a in [false, true] {
        for b in [false, true] {
            let bindings = [("a", OwnedValue::Bool(a)), ("b", OwnedValue::Bool(b))];
            for (source, expected) in [
                ("(and a b)", a && b),
                ("(or a b)", a || b),
                ("(not a)", !a),
                ("(and a b a)", a && b),
                ("(or a b a)", a || b),
            ] {
                assert_eq!(
                    eval_bool(source, &bindings),
                    expected,
                    "{source}: {bindings:?}"
                );
            }
        }
    }
    for (source, expected) in [
        ("(and)", true),
        ("(or)", false),
        ("(and f (unknown))", false),
        ("(or t (unknown))", true),
    ] {
        assert_eq!(
            eval_bool(
                source,
                &[
                    ("f", OwnedValue::Bool(false)),
                    ("t", OwnedValue::Bool(true))
                ]
            ),
            expected,
            "{source}"
        );
    }
    for (value, _) in value_pairs() {
        let truthy = !matches!(value, OwnedValue::Bool(false) | OwnedValue::Null);
        let bindings = [("a", value)];
        assert_eq!(eval_bool("(and a)", &bindings), truthy, "{bindings:?}");
        assert_eq!(eval_bool("(or a)", &bindings), truthy, "{bindings:?}");
        assert_eq!(eval_bool("(not a)", &bindings), !truthy, "{bindings:?}");
    }
}

#[test]
fn membership_for_each_value_type() {
    for (a, b) in value_pairs() {
        let bindings = [("a", a), ("b", b)];
        for (source, expected) in [
            ("(in a a)", true),
            ("(in a b)", false),
            ("(in a b a)", true),
        ] {
            assert_eq!(
                eval_bool(source, &bindings),
                expected,
                "{source}: {bindings:?}"
            );
        }
    }
    assert!(!eval_bool("(in missing missing)", &[]));
}

#[test]
fn membership_uses_promoted_numeric_equality() {
    for a in numeric_values(1) {
        for b in numeric_values(1) {
            let bindings = [("a", a.clone()), ("b", b)];
            assert!(eval_bool("(in a b)", &bindings), "{bindings:?}");
        }
    }
}

#[test]
fn between_uses_inclusive_numeric_bounds() {
    for ((lo, value), hi) in numeric_values(1)
        .into_iter()
        .zip(numeric_values(2))
        .zip(numeric_values(3))
    {
        let bindings = [("a", value), ("lo", lo), ("hi", hi)];
        for (source, expected) in [
            ("(between a lo hi)", true),
            ("(between lo lo hi)", true),
            ("(between hi lo hi)", true),
            ("(between lo a hi)", false),
            ("(between hi lo a)", false),
            ("(between a hi lo)", false),
        ] {
            assert_eq!(
                eval_bool(source, &bindings),
                expected,
                "{source}: {bindings:?}"
            );
        }
    }
}

#[test]
fn symbol_argument_validation() {
    for op in ["=", "!=", "<", "<=", ">", ">=", "in"] {
        for args in ["", "1"] {
            let source = format!("({op} {args})");
            assert!(
                lisp::eval_string(&mut lisp::get_interpreter(), &source).is_err(),
                "{source}"
            );
        }
    }
    for source in [
        "(not)",
        "(not 1 2)",
        "(is-null)",
        "(is-null 1 2)",
        "(is-not-null)",
        "(is-not-null 1 2)",
        "(between)",
        "(between 1 2)",
        "(between 1 2 3 4)",
        "(< a b)",
        "(>= 1 a)",
    ] {
        assert!(
            lisp::eval_string(&mut lisp::get_interpreter(), source).is_err(),
            "{source}"
        );
    }
}

#[test]
fn parsed_expressions_use_the_registered_symbols() {
    for (args, equal) in [
        ("1 1.0", true),
        ("1u16 2", false),
        ("\"x\" \"x\"", true),
        ("nil nil", true),
        ("nil 1", false),
    ] {
        assert_eq!(eval_bool(&format!("(= {args})"), &[]), equal, "{args}");
        assert_eq!(eval_bool(&format!("(!= {args})"), &[]), !equal, "{args}");
    }
}

#[test]
fn null_checks_respect_local_bindings() {
    let mut failures = Vec::new();
    for (value, _) in value_pairs() {
        let is_null = matches!(value, OwnedValue::Null);
        let bindings = [("a", value)];
        for (source, expected) in [("(is-null a)", is_null), ("(is-not-null a)", !is_null)] {
            if eval_bool(source, &bindings) != expected {
                failures.push(format!("{source}: {bindings:?}: expected {expected}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(eval_bool("(let [a 1] (is-not-null a))", &[]));
    assert!(eval_bool("(let [a ()] (is-null a))", &[]));
}

#[test]
fn comparison_and_null_checks_use_shared_global_fields() {
    for (a, b) in value_pairs() {
        let is_null = matches!(a, OwnedValue::Null);
        let global = OwnedValue::Map(OwnedMap::from_pairs(
            vec![("a".into(), a), ("b".into(), b)].into_iter(),
        ));
        let shared = global.shared();
        let mut interpreter = lisp::get_interpreter();
        interpreter.set_global_val(&shared);
        // A null field stays unresolved during evaluation, like a missing field.
        for (source, expected) in [
            ("(= a a)", true),
            ("(!= a a)", false),
            ("(!= a b)", true),
            ("(not (= a b))", true),
            ("(is-null a)", is_null),
            ("(is-not-null a)", !is_null),
            ("(is-null missing)", true),
            ("(is-not-null missing)", false),
        ] {
            assert_eq!(
                lisp::eval_string(&mut interpreter, source)
                    .unwrap()
                    .owned_val(),
                Some(OwnedValue::Bool(expected)),
                "{source}: {global:?}",
            );
        }
    }
}
