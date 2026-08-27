//! Numeric promotion for arithmetic and comparison across value widths.
//!
//! Every numeric operator used to dispatch on the exact type of its first
//! operand and reject anything else, so `(* score 2)` failed with "Type not
//! match, expect U64 found I64(2)" whenever `score` was stored as a u64 --
//! because a bare literal lexes as i64 -- and `(= label 1)` was silently
//! FALSE for a u16 column. A query language cannot ask users to know the
//! storage width of every field they compare against.
//!
//! Integers promote to i128 so no width can overflow the intermediate; the
//! result narrows back to i64 when it fits, else to u64, else to f64. Any
//! float operand makes the whole operation f64. Non-numeric operands are not
//! promoted -- strings still compare as strings -- and an operation whose
//! operands all share one exact type keeps that type, so existing behaviour
//! for homogeneous arguments is unchanged.

use crate::types::{OwnedValue, SharedValue};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    pub fn from_shared(value: &SharedValue<'_>) -> Option<Num> {
        Some(match value {
            SharedValue::U8(v) => Num::Int(**v as i128),
            SharedValue::U16(v) => Num::Int(**v as i128),
            SharedValue::U32(v) => Num::Int(**v as i128),
            SharedValue::U64(v) => Num::Int(**v as i128),
            SharedValue::I8(v) => Num::Int(**v as i128),
            SharedValue::I16(v) => Num::Int(**v as i128),
            SharedValue::I32(v) => Num::Int(**v as i128),
            SharedValue::I64(v) => Num::Int(**v as i128),
            SharedValue::F32(v) => Num::Float(**v as f64),
            SharedValue::F64(v) => Num::Float(**v),
            _ => return None,
        })
    }

    pub fn as_f64(self) -> f64 {
        match self {
            Num::Int(i) => i as f64,
            Num::Float(f) => f,
        }
    }

    /// Narrow a promoted result to the widest ordinary type that holds it.
    pub fn into_owned(self) -> OwnedValue {
        match self {
            Num::Float(f) => OwnedValue::F64(f),
            Num::Int(i) => {
                if let Ok(v) = i64::try_from(i) {
                    OwnedValue::I64(v)
                } else if let Ok(v) = u64::try_from(i) {
                    OwnedValue::U64(v)
                } else {
                    OwnedValue::F64(i as f64)
                }
            }
        }
    }

    pub fn partial_cmp(self, other: Num) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Some(a.cmp(&b)),
            (a, b) => a.as_f64().partial_cmp(&b.as_f64()),
        }
    }

    pub fn equals(self, other: Num) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

/// Fold numeric operands with `int_op` / `float_op`, promoting as needed.
/// `None` when any operand is not numeric, so callers can fall back to their
/// exact-type paths and error messages.
pub fn fold(
    values: &[Option<SharedValue<'_>>],
    int_op: impl Fn(i128, i128) -> Option<i128>,
    float_op: impl Fn(f64, f64) -> f64,
) -> Option<Result<OwnedValue, String>> {
    let nums = values
        .iter()
        .map(|v| v.as_ref().and_then(Num::from_shared))
        .collect::<Option<Vec<_>>>()?;
    let (first, rest) = nums.split_first()?;
    let any_float = nums.iter().any(|n| matches!(n, Num::Float(_)));
    Some(if any_float {
        let mut acc = first.as_f64();
        for n in rest {
            acc = float_op(acc, n.as_f64());
        }
        Ok(OwnedValue::F64(acc))
    } else {
        let mut acc = match first {
            Num::Int(i) => *i,
            Num::Float(_) => unreachable!(),
        };
        for n in rest {
            let Num::Int(i) = n else { unreachable!() };
            match int_op(acc, *i) {
                Some(v) => acc = v,
                None => return Some(Err("integer arithmetic overflowed or divided by zero".into())),
            }
        }
        Ok(Num::Int(acc).into_owned())
    })
}

/// True when every operand is numeric AND they are not all the same exact
/// type -- the case the exact-type fast path cannot handle.
pub fn needs_promotion(values: &[Option<SharedValue<'_>>]) -> bool {
    let mut kinds = values.iter().map(|v| v.as_ref().map(std::mem::discriminant));
    let Some(Some(first)) = kinds.next() else { return false };
    let all_numeric = values
        .iter()
        .all(|v| v.as_ref().and_then(Num::from_shared).is_some());
    all_numeric && kinds.any(|k| k != Some(first))
}

#[cfg(test)]
mod tests {
    use crate::integrated::lisp::{eval_string, get_interpreter};
    use crate::expr::SExpr;
    use crate::types::OwnedValue;

    fn ev(src: &str) -> OwnedValue {
        match eval_string(&mut get_interpreter(), src).unwrap() {
            SExpr::Value(v) => v.into_owned(),
            other => panic!("not a value: {other:?}"),
        }
    }

    #[test]
    fn equality_is_by_value_across_widths() {
        assert_eq!(ev("(= 1u16 1)"), OwnedValue::Bool(true));
        assert_eq!(ev("(= 2u8 2i32 2)"), OwnedValue::Bool(true));
        assert_eq!(ev("(= 1u16 2)"), OwnedValue::Bool(false));
        assert_eq!(ev("(= 1 1.0)"), OwnedValue::Bool(true));
    }

    #[test]
    fn non_numeric_equality_is_unchanged() {
        assert_eq!(ev("(= \"a\" \"a\")"), OwnedValue::Bool(true));
        assert_eq!(ev("(= 1 \"1\")"), OwnedValue::Bool(false));
    }

    #[test]
    fn ordering_across_widths_and_floats() {
        assert_eq!(ev("(< 1u8 2i64 3.0)"), OwnedValue::Bool(true));
        assert_eq!(ev("(>= 3u64 3)"), OwnedValue::Bool(true));
        assert_eq!(ev("(> 1u16 2)"), OwnedValue::Bool(false));
    }

    #[test]
    fn arithmetic_promotes_and_narrows() {
        assert_eq!(ev("(* 3u64 2)"), OwnedValue::I64(6));
        assert_eq!(ev("(+ 1 2.5)"), OwnedValue::F64(3.5));
        assert_eq!(ev("(- 1u8 2)"), OwnedValue::I64(-1), "unsigned minus signed must not wrap");
        assert_eq!(ev("(/ 7u16 2)"), OwnedValue::I64(3), "integer division stays integral");
        assert_eq!(ev("(/ 7 2.0)"), OwnedValue::F64(3.5));
    }

    #[test]
    fn homogeneous_operands_keep_their_type() {
        assert_eq!(ev("(+ 1u64 2u64)"), OwnedValue::U64(3));
        assert_eq!(ev("(* 2 3)"), OwnedValue::I64(6));
    }

    #[test]
    fn division_by_zero_is_an_error_not_a_panic() {
        assert!(eval_string(&mut get_interpreter(), "(/ 1u8 0)").is_err());
    }
}
