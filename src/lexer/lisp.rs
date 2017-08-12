use expr::SExpr;
use std::str::Chars;

pub enum Tokens {
    LeftParentheses,
    RightParentheses,
    WhiteSpace,
    Symbol(String),
    IntNumber(String),
    FloatNumber(String),
    String(String),
    LeftVecParentheses,
    RightVecParentheses,
    Quote
}

static WHITESPACE_CHARSET: &'static str = " \t\r\n";
static NEGATIVE_CHAR: char = '-';
static POINT_CHAR: char = '.';
static NUM_TYPES: [&'static str; 10] = [
    "u8", "u16", "u32", "u64",
    "i8", "i16", "i32", "i64",
    "f32", "f64"];
static LEFT_PARENTHESES: char = '(';
static RIGHT_PARENTHESES: char = ')';
static QUOTE: char = '\'';

macro_rules! defpattern {
    ($name: ident: $($pattern: pat)|*) => {macro_rules! $name {
        () => {$($pattern)|*}
    }};
}

defpattern!(NUMBER_PATTERN: '0'...'9');
// defpattern!(WHITESPACE_PATTERN: ' '|'\t'|'\r'|'\n'); // unreliable

pub fn chars_iter_to_sexpr(iter: &mut Chars) -> Result<SExpr, String> {
    while let Some(c) = iter.next() {
        match c {
            NUMBER_PATTERN!() => {

            },
            ' '|'\t'|'\r'|'\n' => { // whitespaces

            },
            '\'' => { // quote

            },
            '(' => {

            },
            ')' => {

            },
            '"' => { // string

            },
            _ => { // symbol with utf8 chars including emojis

            }
        }
    }
    unimplemented!()
}

pub fn str_to_sexpr<'a>(str: &'a str) -> Result<SExpr, String> {
    let mut iter = str.chars();
    chars_iter_to_sexpr(&mut iter)
}