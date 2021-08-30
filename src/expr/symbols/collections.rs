use crate::types::SharedMap;
use crate::types::OwnedMap;

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

pub fn hashmap(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() & 2 == 0 {
        return Err(format!(
            "Map require even number of parameters. Found {}",
            exprs.len()
        ));
    }
    let mut exprs = exprs.into_iter();
    let mut hashmap = HashMap::new();
    while let (Some(k), Some(v)) = (exprs.next(), exprs.next()) {
        if let (Some(SharedValue::String(k_str)), SExpr::Value(v)) = (k.val(), v) {
            let k_str = k_str.to_owned();
            match v{
                Value::Shared(v) => {
                    // TODO: Try not own elements 
                    hashmap.insert(k_str, v.owned());
                },
                Value::Owned(v) => {
                    hashmap.insert(k_str, v);
                }
            } 
        } else {
            return Err(format!("Wrong hashmap key value data type. Key should be a string and value should be a value"));
        }
    }
    return Ok(SExpr::owned_value(OwnedValue::Map(OwnedMap::from_hash_map(hashmap))));
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
                },
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
