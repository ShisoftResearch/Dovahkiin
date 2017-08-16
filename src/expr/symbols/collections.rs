use super::*;
use types::Value;

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