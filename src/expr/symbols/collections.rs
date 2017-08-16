use super::*;
use types::Value;
use std::collections::HashMap;
use types::custom_types::map::Map;

pub fn size_(vals: &Vec<SExpr>) -> Result<u64, String> {
    let mut result: u64 = 0;
    for val in vals {
        result += match val {
            &SExpr::Vec(ref v) => v.len(),
            &SExpr::Value(Value::Array(ref a)) => a.len(),
            &SExpr::Value(Value::String(ref s)) => s.len(),
            &SExpr::Value(Value::Map(ref m)) => m.len(),
            _ => return Err(format!("Cannot measure size for {:?}", val))
        } as u64;
    }
    return Ok(result)
}

pub fn size(vals: Vec<SExpr>) -> Result<SExpr, String> {
    Ok(SExpr::Value(Value::U64(size_(&vals)?)))
}

pub fn concat(lists: Vec<SExpr>) -> Result<SExpr, String> {
    let total_size = size_(&lists)?;
    let mut result = Vec::with_capacity(total_size as usize);
    let mut vec_lists = Vec::new();
    for list in lists {
        vec_lists.push(if let SExpr::Vec(v) = stream::to_vec(list)? { v } else {
            return Err("Unexpected error on concat".to_string());
        });
    }
    for mut vec in vec_lists {
        result.append(&mut vec);
    }
    return Ok(SExpr::Vec(result));
}

pub fn hashmap(mut exprs: Vec<SExpr>) -> Result<SExpr, String> {
    if exprs.len() & 2 == 0 {
        return Err(format!("Map require even number of parameters. Found {}", exprs.len()));
    }
    let mut exprs = exprs.into_iter();
    let mut hashmap = HashMap::new();
    while let (Some(k), Some(v)) = (exprs.next(), exprs.next()) {
        if let (SExpr::Value(Value::String(k_str)), SExpr::Value(value)) = (k , v) {
            hashmap.insert(k_str, value);
        } else {
            return Err(format!("Wrong hashmap key value data type. Key should be a string and value should be a value"));
        }
    }
    return Ok(SExpr::Value(Value::Map(Map::from_hash_map(hashmap))))
}

pub fn merge(exprs: Vec<SExpr>) -> Result<SExpr, String> {
    let mut value_map = HashMap::new();
    let mut field_names = HashSet::new();
    for expr in exprs {
        match expr {
            SExpr::Value(Value::Map(m)) => {
                let mut map = m.map;
                let mut fields = m.fields;
                for (k, v) in map.into_iter() {
                    value_map.insert(k, v);
                }
                for field in fields {
                    field_names.insert(field);
                }
            },
            _ => return Err(format!("Only map value can be merged. Found {:?}", expr))
        }
    }
    Ok(SExpr::Value(Value::Map(Map {
        map: value_map,
        fields: field_names
    })))
}