use crate::parser::expression::Expression;
use crate::parser::tokenizer::Token;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;
use core::error::Error;
use core::fmt::{Display, Formatter};
use alloc::borrow::ToOwned;

pub mod expression;
mod tokenizer;
mod util;

#[derive(Debug, PartialEq)]
pub enum ExpressionError {
    InvalidSyntax(usize),
    UnknownVariable(String),
    DivisionByZero,
}

impl Display for ExpressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("Parse error")
    }
}

impl Error for ExpressionError {}

#[derive(Debug, PartialEq, Clone)]
enum HalfParsed<'t> {
    Token(Token<'t>),
    Expression(Expression),
}

impl<'t> HalfParsed<'t> {
    fn is_expression(&self) -> bool {
        match self {
            HalfParsed::Token(_) => false,
            HalfParsed::Expression(_) => true,
        }
    }

    fn is_token(&self) -> bool {
        match self {
            HalfParsed::Token(_) => true,
            HalfParsed::Expression(_) => false,
        }
    }

    fn expression(self) -> Expression {
        match self {
            HalfParsed::Token(_) => panic!("Called expression on non-expression!"),
            HalfParsed::Expression(b) => b,
        }
    }

    fn token(self) -> Token<'t> {
        match self {
            HalfParsed::Token(t) => t,
            HalfParsed::Expression(_) => panic!("Called token on non-token!"),
        }
    }
}

impl<'t> PartialEq<Token<'t>> for HalfParsed<'t> {
    fn eq(&self, other: &Token) -> bool {
        match self {
            HalfParsed::Token(t) => t == other,
            HalfParsed::Expression(_) => false,
        }
    }
}

pub fn parse(input: &str) -> Result<Expression, ExpressionError> {
    let mut tokens = tokenizer::tokenize(input)?;
    // Reverse so we can pop() from the beginning
    tokens.reverse();
    if tokens.is_empty() {
        return Ok(Expression::Immediate(0.0));
    }
    let mut symbol_stack = Vec::<HalfParsed>::new();
    while !tokens.is_empty() || symbol_stack.len() > 1 || symbol_stack[0].is_token() {
        // Try Reducing
        // Decimal Literals
        if let Some(HalfParsed::Token(Token::DecLiteral(val))) = symbol_stack.last() {
            let val = *val;
            symbol_stack.pop();
            symbol_stack.push(HalfParsed::Expression(Expression::Immediate(val)));
            continue;
        }

        // Variables
        if let Some(HalfParsed::Token(Token::Identifier(val))) = symbol_stack.last() {
            let val = *val;
            symbol_stack.pop();
            symbol_stack.push(HalfParsed::Expression(Expression::Var(val.to_owned())));
            continue;
        }

        // Binary Expression
        let next_token = tokens.last();
        if symbol_stack.len() >= 3
            && symbol_stack[symbol_stack.len() - 1].is_expression()
            && symbol_stack[symbol_stack.len() - 2].is_token()
            && symbol_stack[symbol_stack.len() - 3].is_expression()
        {
            // Precedence for multiplication/division
            if !(symbol_stack[symbol_stack.len() - 2] == Token::Add
                || symbol_stack[symbol_stack.len() - 2] == Token::Subtract)
                || (next_token.is_none()
                    || (next_token.unwrap() != &Token::Multiply
                        && next_token.unwrap() != &Token::Divide))
            {
                let expr1 = symbol_stack.pop().unwrap().expression();
                let op = symbol_stack.pop().unwrap().token();
                let expr2 = symbol_stack.pop().unwrap().expression();

                let combined_expr: Expression = match op {
                    Token::Multiply => Expression::Mul(Box::new(expr2), Box::new(expr1)),
                    Token::Divide => Expression::Div(Box::new(expr2), Box::new(expr1)),
                    Token::Add => Expression::Add(Box::new(expr2), Box::new(expr1)),
                    Token::Subtract => Expression::Sub(Box::new(expr2), Box::new(expr1)),
                    _ => continue,
                };

                symbol_stack.push(HalfParsed::Expression(combined_expr));
                continue;
            }
        }

        // Parenthesis
        if symbol_stack.len() >= 3
            && symbol_stack[symbol_stack.len() - 1] == Token::RParen
            && symbol_stack[symbol_stack.len() - 2].is_expression()
            && symbol_stack[symbol_stack.len() - 3] == Token::LParen
        {
            symbol_stack.pop();
            let expr = symbol_stack.pop().unwrap().expression();
            symbol_stack.pop();
            symbol_stack.push(HalfParsed::Expression(expr));
            continue;
        }

        // Try Shifting
        // Error if we cant reduce and there are no more tokens to shift
        if tokens.is_empty() {
            return Err(ExpressionError::InvalidSyntax(0));
        }
        symbol_stack.push(HalfParsed::Token(tokens.pop().unwrap()));
    }

    assert_eq!(symbol_stack.len(), 1);
    assert!(tokens.is_empty());

    Ok(symbol_stack.pop().unwrap().expression())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        // Test Order of operations
        assert_eq!(
            parse("1+2*3"),
            Ok(Expression::Add(
                Box::new(Expression::Immediate(1.0)),
                Box::new(Expression::Mul(
                    Box::new(Expression::Immediate(2.0)),
                    Box::new(Expression::Immediate(3.0))
                ))
            ))
        );
        assert_eq!(
            parse("1*2+3"),
            Ok(Expression::Add(
                Box::new(Expression::Mul(
                    Box::new(Expression::Immediate(1.0)),
                    Box::new(Expression::Immediate(2.0))
                )),
                Box::new(Expression::Immediate(3.0))
            ))
        );
        assert_eq!(
            parse("489"),
            Ok(Expression::Immediate(489.0))
        )
    }
}
