use log::kv;

use crate::types::{Map, OwnedMap};

use super::*;
use std::collections::HashMap;

pub fn size_(vals: &Vec<SExpr>) -> Result<u64, String> {
    let mut result: u64 = 0;
    for val in vals {
        result += match &val {
            &SExpr::Vec(ref v) => v.len(),
            &SExpr::Value(val) => {
                let v = val.norm();
                match v {
                    SharedValue::Array(ref a) => a.len(),
                    SharedValue::String(ref s) => s.len(),
                    SharedValue::Map(ref m) => m.len(),
                    _ => return Err(format!("Cannot measure size for value {:?}", val)),
                }
            }
            _ => return Err(format!("Cannot measure size for {:?}", val)),
        } as u64;
    }
    return Ok(result);
}

pub fn size(vals: Vec<SExpr>) -> Result<SExpr, String> {
    Ok(SExpr::owned_value(OwnedValue::U64(size_(&vals)?)))
}

pub fn concat(lists: Vec<SExpr>) -> Result<SExpr, String> {
    let total_size = size_(&lists)?;
    let mut result = Vec::with_capacity(total_size as usize);
    let mut vec_lists = Vec::new();
    for list in lists {
        vec_lists.push(if let SExpr::Vec(v) = stream::to_vec(list)? {
            v
        } else {
            return Err("Unexpected error on concat".to_string());
        });
    }
    for mut vec in vec_lists {
        result.append(&mut vec);
    }
    return Ok(SExpr::Vec(result));
}

pub fn hashmap(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() == 1 {
        match exprs.into_iter().next().unwrap() {
            SExpr::Vec(l) | SExpr::List(l) => {
                exprs = l
            },
            v => {
                return Err(format!("Single patameter only support list and seq, found {:?}", v))
            }
        }
    } else if exprs.len() & 2 == 0 {
        return Err(format!(
            "Map require even number of parameters. Found {}",
            exprs.len()
        ));
    }
    let mut exprs = exprs.into_iter();
    let mut hashmap = HashMap::new();
    while let (Some(k), Some(v)) = (exprs.next(), exprs.next()) {
        match (k, v) {
            (SExpr::Value(k_val), SExpr::Value(v)) => {
                let k_norm = k_val.norm();
                let k_str_opt = k_norm.string();
                match (k_str_opt, v) {
                    (Some(k_str), Value::Shared(v)) => {
                        // TODO: Try not own elements
                        hashmap.insert(k_str.to_owned(), v.owned());
                    }
                    (Some(k_str), Value::Owned(v)) => {
                        hashmap.insert(k_str.to_owned(), v);
                    }
                    (None, _) => return Err(format!("Only string key is allowed, got {:?}", k_val)),
                }
            }
            (SExpr::Keyword(_, kw), SExpr::Value(v)) => {
                match v {
                    Value::Shared(v) => {
                        // TODO: Try not own elements
                        hashmap.insert(kw, v.owned());
                    }
                    Value::Owned(v) => {
                        hashmap.insert(kw, v);
                    }
                } 
            }
            _ => {
                return Err(format!("Wrong hashmap key value data type. Key should be a string or keyword and value should be a value"));
            }
        }
    }
    return Ok(SExpr::owned_value(OwnedValue::Map(
        OwnedMap::from_hash_map(hashmap),
    )));
}

pub fn merge<'a>(exprs: Vec<SExpr<'a>>) -> Result<SExpr<'a>, String> {
    let mut value_map = HashMap::new();
    let mut field_names = Vec::new();
    for expr in exprs {
        if let SExpr::Value(val) = expr {
            match val {
                Value::Shared(SharedValue::Map(m)) => {
                    let map = m.map;
                    let mut fields = m.fields;
                    for (k, v) in map.into_iter() {
                        // TODO: try not own it
                        value_map.insert(k, v.owned());
                    }
                    field_names.append(&mut fields);
                }
                Value::Owned(OwnedValue::Map(m)) => {
                    let map = m.map;
                    let mut fields = m.fields;
                    for (k, v) in map.into_iter() {
                        value_map.insert(k, v);
                    }
                    field_names.append(&mut fields);
                }
                _ => return Err(format!("Only map value can be merged. Found {:?}", val)),
            }
        }
    }
    field_names.dedup();
    Ok(SExpr::owned_value(OwnedValue::Map(OwnedMap {
        map: value_map,
        fields: field_names,
    })))
}

pub fn conj(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let list = stream::to_vec(exprs.remove(0));
    if let Ok(SExpr::Vec(mut vec)) = list {
        vec.append(&mut exprs);
        return Ok(SExpr::Vec(vec));
    } else {
        return Err(format!("Cannot concat. {:?}", list));
    }
}
