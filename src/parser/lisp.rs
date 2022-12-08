use bifrost_hasher::hash_str;
use crate::lexer::lisp::Token;
use crate::types::OwnedValue as Value;
use std::{vec::IntoIter, marker::PhantomData};

use crate::expr::{SExpr, serde::Expr};

pub trait ParserExpr: Sized {
    fn list(data: Vec<Self>) -> Self;
    fn vec(data: Vec<Self>) -> Self;
    fn symbol(name: String) -> Self;
    fn owned_val(val: Value) -> Self;
}

pub struct Parser<E: ParserExpr> {
    _p: PhantomData<E>
}

pub type SExprParser<'a> = Parser<SExpr<'a>>;
pub type SerdeExprParser = Parser<Expr>;

impl <E: ParserExpr> Parser <E> {
    fn parse_list<'a>(iter: &mut IntoIter<Token>) -> Result<E, String> {
        let mut contents = Vec::new();
        while let Some(token) = iter.next() {
            match token {
                Token::RightParentheses => {
                    return Ok(E::list(contents));
                }
                _ => {
                    contents.push(Self::parse_token(token, iter)?);
                }
            }
        }
        Err(String::from("Unexpected EOF, expect ')'"))
    }
    
    fn parse_vec<'a>(iter: &mut IntoIter<Token>) -> Result<E, String> {
        let mut contents = Vec::new();
        while let Some(token) = iter.next() {
            match token {
                Token::RightVecParentheses => {
                    return Ok(E::vec(contents));
                }
                _ => {
                    contents.push(Self::parse_token(token, iter)?);
                }
            }
        }
        Err(String::from("Unexpected EOF, expect ']'"))
    }
    
    fn parse_symbol<'a>(name: String) -> E {
        E::symbol(name)
    }
    
    fn parse_int<'a>(num_str: String, unit: String) -> Result<E, String> {
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
        .map(E::owned_val)
    }
    
    fn parse_float<'a>(num_str: String, unit: String) -> Result<E, String> {
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
        .map(E::owned_val)
    }
    
    fn parse_string<'a>(str: String) -> E {
        E::owned_val(Value::String(str))
    }
    
    fn parse_token<'a>(token: Token, iter: &mut IntoIter<Token>) -> Result<E, String> {
        match token {
            Token::LeftParentheses => Ok(Self::parse_list(iter)?), // list
            Token::Symbol(name) => Ok(Self::parse_symbol(name)),
            Token::IntNumber(num, unit) => Ok(Self::parse_int(num, unit)?),
            Token::FloatNumber(num, unit) => Ok(Self::parse_float(num, unit)?),
            Token::String(str) => Ok(Self::parse_string(str)),
            Token::LeftVecParentheses => Ok(Self::parse_vec(iter)?),
            _ => Err(format!("Unexpected start token {}", token.to_string())),
        }
    }
    
    pub fn parse_to_expr<'a>(tokens: Vec<Token>) -> Result<Vec<E>, String> {
        let mut exprs: Vec<E> = Vec::new();
        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            exprs.push(Self::parse_token(token, &mut iter)?)
        }
        Ok(exprs)
    }    
}