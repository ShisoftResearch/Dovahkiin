use super::*;
use std::ops::{Index, IndexMut};
use serde;

pub trait ToValue: serde::Serialize{
    fn value(self) -> Value;
}
impl <'a> ToValue for &'a str {
    fn value(self) -> Value {
        Value::String(self.to_string())
    }
}
impl ToValue for Value {
    fn value(self) -> Value {
        self
    }
}

impl <'a> Index <&'a str> for Value {
    type Output = Value;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            &Value::Map(ref map) => map.get(index),
            _ => &NULL_VALUE
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            &Value::Array(ref array) => array.get(index).unwrap_or(&NULL_VALUE),
            &Value::Map(ref map) => map.get_by_key_id(index as u64),
            _ => &NULL_VALUE
        }
    }
}

impl Index<u64> for Value {
    type Output = Value;

    fn index(&self, index: u64) -> &Self::Output {
        match self {
            &Value::Map(ref map) => map.get_by_key_id(index),
            &Value::Array(ref array) => array.get(index as usize).unwrap_or(&NULL_VALUE),
            _ => &NULL_VALUE
        }
    }
}

static MISSING_ARRAY_ITEM: &'static str ="Cannot get item from array";
static DATA_TYPE_DONT_SUPPORT_INDEXING: &'static str ="Data type don't support indexing";

impl <'a> IndexMut <&'a str> for Value {
    fn index_mut<'b>(&'b mut self, index: &'a str) -> &'b mut Self::Output {
        match self {
            &mut Value::Map(ref mut map) => map.get_mut(index),
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING)
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        match self {
            &mut Value::Array(ref mut array) => array.get_mut(index).expect(MISSING_ARRAY_ITEM),
            &mut Value::Map(ref mut map) => map.get_mut_by_key_id(index as u64),
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING)
        }
    }
}

impl IndexMut<u64> for Value {
    fn index_mut<'a>(&'a mut self, index: u64) -> &'a mut Self::Output {
        match self {
            &mut Value::Map(ref mut map) => map.get_mut_by_key_id(index),
            &mut Value::Array(ref mut array) => array.get_mut(index as usize).expect(MISSING_ARRAY_ITEM),
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING)
        }
    }
}