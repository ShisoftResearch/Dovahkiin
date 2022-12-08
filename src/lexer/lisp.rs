use std::collections::HashSet;

#[derive(Debug)]
pub enum Token {
    LeftParentheses,
    RightParentheses,
    Symbol(String),
    IntNumber(String, String),
    FloatNumber(String, String),
    String(String),
    LeftVecParentheses,
    RightVecParentheses,
    // Quote
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            &Token::LeftParentheses => String::from("("),
            &Token::RightParentheses => String::from(")"),
            &Token::Symbol(ref s) => s.clone(),
            &Token::IntNumber(ref n, ref u) => format!("{}{}", n, u),
            &Token::FloatNumber(ref n, ref u) => format!("{}{}", n, u),
            &Token::String(ref s) => format!("\"{}\"", s),
            &Token::LeftVecParentheses => String::from("["),
            &Token::RightVecParentheses => String::from("]"),
        }
    }
}

pub struct CharIter {
    chars: Vec<char>,
    current_pos: usize,
}

impl CharIter {
    pub fn new(data: Vec<char>) -> CharIter {
        CharIter {
            chars: data,
            current_pos: 0,
        }
    }
    pub fn next(&mut self) -> Option<char> {
        self.current_pos += 1;
        self.chars.get(self.current_pos).cloned()
    }
    pub fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current_pos + 1).cloned()
    }

    pub fn current(&mut self) -> Option<char> {
        self.chars.get(self.current_pos).cloned()
    }
}

lazy_static! {
    static ref INT_NUM_TYPES: HashSet<String> =
        vec!["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
            .into_iter()
            .map(|str| str.to_string())
            .collect();
    static ref FLOAT_NUM_TYPES: HashSet<String> = vec!["f32", "f64"]
        .into_iter()
        .map(|str| str.to_string())
        .collect();
}

macro_rules! defpattern {
    ($name: ident: $($pattern: pat)*) => {macro_rules! $name {
        () => {$($pattern)*}
    }};
}

defpattern!(NUMBER_PATTERN: '0'..='9');
// defpattern!(WHITESPACE_PATTERN: ' '|'\t'|'\r'|'\n'); // unreliable

fn readout_whitespaces(iter: &mut CharIter) {
    while let Some(c) = iter.next() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {}
            _ => {
                break;
            }
        }
    }
}

fn read_number(first: char, iter: &mut CharIter) -> Result<Token, String> {
    let mut digit_chars = vec![first];
    let mut unit_chars = Vec::new();
    let mut is_float_number = false;
    while let Some(c) = iter.next() {
        match c {
            NUMBER_PATTERN!() => {
                match (unit_chars.first(), unit_chars.get(1), c) {
                    // terminal states
                    (Some(&'u'), None,       '8') | // u8
                    (Some(&'u'), Some(&'1'), '6') | // u16
                    (Some(&'u'), Some(&'3'), '2') | // u32
                    (Some(&'u'), Some(&'6'), '4') | // u64
                    (Some(&'i'), None,       '8') | // i8
                    (Some(&'i'), Some(&'1'), '6') | // i16
                    (Some(&'i'), Some(&'3'), '2') | // i32
                    (Some(&'i'), Some(&'6'), '4') | // i64
                    (Some(&'f'), Some(&'3'), '2') | // f32
                    (Some(&'f'), Some(&'6'), '4')   // f64
                    => {
                        unit_chars.push(c);
                        break;
                    }
                    // mid states
                    (Some(&'u'), None, '1') | // u16
                    (Some(&'u'), None, '3') | // u32
                    (Some(&'u'), None, '6') | // u64
                    (Some(&'i'), None, '1') | // i16
                    (Some(&'i'), None, '3') | // i32
                    (Some(&'i'), None, '6') | // i64
                    (Some(&'f'), None, '3') | // f32
                    (Some(&'f'), None, '6')   // f64
                    => {
                        unit_chars.push(c);
                    },
                    (None, _, _) => {
                        digit_chars.push(c);
                    }
                    _ => {
                        return Err(format!("Unexpected token '{}' for number unit", c))
                    }
                }
            }
            '.' => {
                if !is_float_number {
                    is_float_number = true;
                    digit_chars.push(c);
                } else {
                    return Err("There is a floating point in the number already".to_string());
                }
            }
            'u' | 'i' | 'f' => {
                unit_chars.push(c);
            }
            ' ' | '\t' | '\r' | '\n' => {
                break;
            }
            _ => return Err(format!("Unexpected token '{}' for number", c)),
        }
    }
    let digit_part: String = digit_chars.into_iter().collect();
    let unit_part: String = unit_chars.into_iter().collect();
    iter.next();
    if is_float_number {
        if !FLOAT_NUM_TYPES.contains(&unit_part) {
            return Err(format!(
                "Invalid float number '{}{}'",
                digit_part, unit_part
            ));
        }
        return Ok(Token::FloatNumber(digit_part, unit_part));
    } else {
        if !INT_NUM_TYPES.contains(&unit_part) {
            return Err(format!(
                "Invalid integer number '{}{}'",
                digit_part, unit_part
            ));
        }
        return Ok(Token::IntNumber(digit_part, unit_part));
    }
}

fn read_escaped_char(iter: &mut CharIter) -> Result<char, String> {
    while let Some(c) = iter.next() {
        match c {
            'u' | 'U' => {
                // read 6 digit hex number as unicode
                let mut hex_chars: Vec<char> = Vec::new();
                for _ in 0..6 {
                    if let Some(c) = iter.next() {
                        let c = c.to_ascii_lowercase();
                        match c {
                            NUMBER_PATTERN!() | 'a'..='f' => {
                                hex_chars.push(c);
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }
                let unicode_hex: String = hex_chars.into_iter().collect();
                let unicode = u32::from_str_radix(&unicode_hex, 16).map_err(|_| {
                    format!("Cannot parse hex for escape character 0x{}", unicode_hex)
                })?;
                return ::std::char::from_u32(unicode)
                    .ok_or(format!("Cannot escape character \\u{}", unicode_hex));
            }
            't' => return Ok('\t'),
            'n' => return Ok('\n'),
            'r' => return Ok('\r'),
            '\'' => return Ok('\''),
            '"' => return Ok('"'),
            '\\' => return Ok('\''),
            _ => return Err(format!("Unknown escape character '{}'", c)),
        }
    }
    return Err("Unexpected EOF".to_string());
}

fn read_string(iter: &mut CharIter) -> Result<Token, String> {
    let mut chars = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            '\\' => {
                // escaping
                chars.push(read_escaped_char(iter)?);
            }
            '"' => {
                break;
            }
            _ => {
                chars.push(c);
            }
        }
    }
    return Ok(Token::String(chars.into_iter().collect()));
}

fn read_symbol(first: char, iter: &mut CharIter) -> Result<Token, String> {
    let mut chars = vec![first];
    while let Some(c) = iter.next() {
        match c {
            ' ' | '\t' | '\r' | '\n' | '(' | ')' | '[' | ']' | '\'' => {
                break;
            }
            _ => {
                chars.push(c);
            }
        }
    }
    return Ok(Token::Symbol(chars.into_iter().collect()));
}

pub fn tokenize_chars_iter(iter: &mut CharIter) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    while let Some(c) = iter.current() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                // whitespaces
                // will do nothing
                readout_whitespaces(iter);
            }
            '(' => {
                tokens.push(Token::LeftParentheses);
                iter.next();
            }
            ')' => {
                tokens.push(Token::RightParentheses);
                iter.next();
            }
            '[' => {
                tokens.push(Token::LeftVecParentheses);
                iter.next();
            }
            ']' => {
                tokens.push(Token::RightVecParentheses);
                iter.next();
            }
            NUMBER_PATTERN!() => {
                tokens.push(read_number(c, iter)?);
            }
            // match negative number need next char to be a digit
            '-' if match iter.peek_next() {
                Some(NUMBER_PATTERN!()) => true,
                _ => false,
            } =>
            {
                tokens.push(read_number(c, iter)?);
            }
            // '\'' => { // quote
            //     tokens.push(Token::Quote);
            //     iter.next();
            // },
            '"' => {
                // string
                tokens.push(read_string(iter)?);
            }
            _ => {
                // symbol with utf8 chars including emojis
                tokens.push(read_symbol(c, iter)?);
            }
        }
    }
    return Ok(tokens);
}

pub fn tokenize_str<'a>(str: &'a str) -> Result<Vec<Token>, String> {
    let mut iter = CharIter::new(str.chars().collect());
    tokenize_chars_iter(&mut iter)
}
