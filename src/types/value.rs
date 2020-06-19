use super::*;
use std::collections::HashMap;
use std::iter::Iterator;
use std::ops::{Index, IndexMut};
use std::vec::IntoIter;

pub trait ToValue {
    fn value(self) -> Value;
}

impl<'a> ToValue for &'a str {
    fn value(self) -> Value {
        Value::String(self.to_string())
    }
}

impl ToValue for Value {
    fn value(self) -> Value {
        self
    }
}

impl ToValue for Vec<Value> where {
    fn value(self) -> Value {
        return Value::Array(self);
    }
}

impl<V> ToValue for Vec<HashMap<String, V>>
where
    V: ToValue,
{
    fn value(self) -> Value {
        return Value::Array(self.into_iter().map(|m| m.value()).collect());
    }
}

impl<V> ToValue for HashMap<String, V>
where
    V: ToValue,
{
    fn value(self) -> Value {
        let mut map = Map::new();
        for (k, v) in self {
            map.insert_value(&k, v);
        }
        return Value::Map(map);
    }
}

impl<'a> Index<&'a str> for Value {
    type Output = Value;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            &Value::Map(ref map) => map.get(index),
            _ => &NULL_VALUE,
        }
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            &Value::Array(ref array) => array.get(index).unwrap_or(&NULL_VALUE),
            &Value::Map(ref map) => map.get_by_key_id(index as u64),
            _ => &NULL_VALUE,
        }
    }
}

impl Index<u64> for Value {
    type Output = Value;

    fn index(&self, index: u64) -> &Self::Output {
        match self {
            &Value::Map(ref map) => map.get_by_key_id(index),
            &Value::Array(ref array) => array.get(index as usize).unwrap_or(&NULL_VALUE),
            _ => &NULL_VALUE,
        }
    }
}

static MISSING_ARRAY_ITEM: &'static str = "Cannot get item from array";
static DATA_TYPE_DONT_SUPPORT_INDEXING: &'static str = "Data type don't support indexing";

impl<'a> IndexMut<&'a str> for Value {
    fn index_mut<'b>(&'b mut self, index: &'a str) -> &'b mut Self::Output {
        match self {
            &mut Value::Map(ref mut map) => map.get_mut(index),
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING),
        }
    }
}

impl IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            &mut Value::Array(ref mut array) => array.get_mut(index).expect(MISSING_ARRAY_ITEM),
            &mut Value::Map(ref mut map) => map.get_mut_by_key_id(index as u64),
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING),
        }
    }
}

impl IndexMut<u64> for Value {
    fn index_mut<'a>(&'a mut self, index: u64) -> &'a mut Self::Output {
        match self {
            &mut Value::Map(ref mut map) => map.get_mut_by_key_id(index),
            &mut Value::Array(ref mut array) => {
                array.get_mut(index as usize).expect(MISSING_ARRAY_ITEM)
            }
            _ => panic!(DATA_TYPE_DONT_SUPPORT_INDEXING),
        }
    }
}

impl IntoIterator for Value {
    type Item = Value;
    type IntoIter = IntoIter<Value>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        if let Value::Array(array) = self {
            return array.into_iter();
        } else {
            panic!()
        }
    }
}

pub struct ValueIter<'a> {
    array: &'a Vec<Value>,
    cursor: usize,
}

impl<'a> ValueIter<'a> {
    pub fn new(array: &'a Vec<Value>) -> ValueIter<'a> {
        ValueIter { array, cursor: 0 }
    }
}

impl<'a> Iterator for ValueIter<'a> {
    type Item = &'a Value;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.iter_next()
    }
}

impl <'a> ValueIter<'a> {
    fn iter_next(&mut self) -> Option<<Self as Iterator>::Item> {
        let val_opt = self.array.get(self.cursor);
        if let Some(ref _val) = val_opt {
            self.cursor += 1;
        }
        return val_opt;
    }
}

impl Eq for Value {
    // TODO: elaborate it
}

pub trait Compound {}
