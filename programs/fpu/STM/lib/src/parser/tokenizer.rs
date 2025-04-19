use crate::parser::ParseError;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'s> {
    DecLiteral(f64),
    Multiply,
    Divide,
    Add,
    Subtract,
    Identifier(&'s str),
    Equals,
    LParen,
    RParen,
    Whitespace,
}

pub fn get_next_token(input: &str) -> Result<(Token, &str), ParseError> {

    if let Some(val) = token_decimal(input) {
        let (token_str, remainder) = input.split_at_checked(val).unwrap_or((input, ""));
        Ok((Token::DecLiteral(token_str.parse().unwrap()), remainder))
    } else if let Some(val) = token_multiply(input) {
        let (_, remainder) = input.split_at_checked(val).unwrap_or((input, ""));
        Ok((Token::Multiply, remainder))
    } else if let Some(val) = token_divide(input) {
        let (_, remainder) = input.split_at_checked(val).unwrap_or((input, ""));
        Ok((Token::Divide, remainder))
    } else if let Some(val) = token_plus(input) {
        let (_, remainder) = input.split_at_checked(val).unwrap_or((input, ""));
        Ok((Token::Add, remainder))
    } else if let Some(val) = token_minus(input) {
        let (_, remainder) = input.split_at_checked(val).unwrap_or((input, ""));
        Ok((Token::Subtract, remainder))
    } else {
        Err(ParseError::InvalidSyntax)
    }
}

// Returned value is the index one past the last character
fn token_decimal(input: &str) -> Option<usize> {

    // In writing our own parser, we discovered the rust tokenizer is broken
    const ZERO: u8 = '0' as u8;
    const NINE: u8 = '9' as u8;
    const DOT: u8 = '.' as u8;

    let mut counter: usize = 0;
    let mut has_seen_decimal = false;
    for c in input.bytes() {
        match c {
            ZERO..=NINE => {},
            DOT => {
                if has_seen_decimal {
                    return Some(counter);
                }
                has_seen_decimal = true;
            },
            _ => {
                return if counter == 0 {
                    None
                } else {
                    Some(counter)
                }
            },
        }
        counter += 1;
    }
    Some(counter)
}

fn token_multiply(input: &str) -> Option<usize> {
    if input.starts_with('*') {
        Some(1)
    } else {
        None
    }
}

fn token_divide(input: &str) -> Option<usize> {
    if input.starts_with('/') {
        Some(1)
    } else {
        None
    }
}

fn token_plus(input: &str) -> Option<usize> {
    if input.starts_with('+') {
        Some(1)
    } else {
        None
    }
}

fn token_minus(input: &str) -> Option<usize> {
    if input.starts_with('-') {
        Some(1)
    } else {
        None
    }
}

pub fn tokenize(mut input: &str) -> Result<Vec<Token>, ParseError> {
    let mut ret = Vec::new();
    while input.len() > 0 {
        let values = get_next_token(input)?;
        input = values.1;
        if values.0 != Token::Whitespace {
            ret.push(values.0);
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn tokenizer_test() {
        assert_eq!(
            tokenize(".2*3"),
            Ok(vec![
                Token::DecLiteral(0.2),
                Token::Multiply,
                Token::DecLiteral(3.0),
            ])
        );
    }
}
