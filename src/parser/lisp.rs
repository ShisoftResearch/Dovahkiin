use bifrost_hasher::hash_str;
use expr::SExpr;
use lexer::lisp::Token;
use std::vec::IntoIter;
use types::{OwnedValue as Value};

fn parse_list(iter: &mut IntoIter<Token>) -> Result<SExpr, String> {
    let mut contents = Vec::new();
    while let Some(token) = iter.next() {
        match token {
            Token::RightParentheses => {
                return Ok(SExpr::List(contents));
            }
            _ => {
                contents.push(parse_token(token, iter)?);
            }
        }
    }
    Err(String::from("Unexpected EOF, expect ')'"))
}

fn parse_vec(iter: &mut IntoIter<Token>) -> Result<SExpr, String> {
    let mut contents = Vec::new();
    while let Some(token) = iter.next() {
        match token {
            Token::RightVecParentheses => {
                return Ok(SExpr::Vec(contents));
            }
            _ => {
                contents.push(parse_token(token, iter)?);
            }
        }
    }
    Err(String::from("Unexpected EOF, expect ']'"))
}

fn parse_symbol(name: String) -> SExpr {
    SExpr::ISymbol(hash_str(&name), name)
}

fn parse_int(num_str: String, unit: String) -> Result<SExpr, String> {
    match unit.as_ref() {
        "u8" => num_str.parse::<u8>().map(Value::U8),
        "u16" => num_str.parse::<u16>().map(Value::U16),
        "u32" => num_str.parse::<u32>().map(Value::U32),
        "u64" => num_str.parse::<u64>().map(Value::U64),
        "i8" => num_str.parse::<i8>().map(Value::I8),
        "i16" => num_str.parse::<i16>().map(Value::I16),
        "i32" => num_str.parse::<i32>().map(Value::I32),
        "i64" => num_str.parse::<i64>().map(Value::I64),
        _ => return Err(format!("Unknown int number type {}", unit)),
    }
    .map_err(|e| {
        format!(
            "Cannot parse int {} with unit {}, reason: {:?}",
            num_str, unit, e
        )
    })
    .map(SExpr::Value)
}

fn parse_float(num_str: String, unit: String) -> Result<SExpr, String> {
    match unit.as_ref() {
        "f32" => num_str.parse::<f32>().map(Value::F32),
        "f64" => num_str.parse::<f64>().map(Value::F64),
        _ => return Err(format!("Unknown float number type {}", unit)),
    }
    .map_err(|e| {
        format!(
            "Cannot parse float {} with unit {}, reason: {:?}",
            num_str, unit, e
        )
    })
    .map(SExpr::Value)
}

fn parse_string(str: String) -> SExpr {
    SExpr::Value(Value::String(str))
}

fn parse_token(token: Token, iter: &mut IntoIter<Token>) -> Result<SExpr, String> {
    match token {
        Token::LeftParentheses => Ok(parse_list(iter)?), // list
        Token::Symbol(name) => Ok(parse_symbol(name)),
        Token::IntNumber(num, unit) => Ok(parse_int(num, unit)?),
        Token::FloatNumber(num, unit) => Ok(parse_float(num, unit)?),
        Token::String(str) => Ok(parse_string(str)),
        Token::LeftVecParentheses => Ok(parse_vec(iter)?),
        _ => Err(format!("Unexpected start token {}", token.to_string())),
    }
}

pub fn parse_to_sexpr(tokens: Vec<Token>) -> Result<Vec<SExpr>, String> {
    let mut exprs: Vec<SExpr> = Vec::new();
    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        exprs.push(parse_token(token, &mut iter)?)
    }
    Ok(exprs)
}
