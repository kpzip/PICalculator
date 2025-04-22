use crate::parser::expression::{Expression, Func};
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
            // If there is a parenthesis, wait to parse as either multiplication or function call
            if tokens.last() != Some(&Token::LParen) {
                let val = *val;
                symbol_stack.pop();
                symbol_stack.push(HalfParsed::Expression(Expression::Var(val.to_owned())));
                continue;
            }
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
                        && next_token.unwrap() != &Token::Divide
                        && next_token.unwrap() != &Token::LParen))
            {

                let op = symbol_stack[symbol_stack.len() - 2].clone().token();

                match op {
                    Token::Multiply => {
                        let expr1 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.pop();
                        let expr2 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.push(HalfParsed::Expression(Expression::Mul(Box::new(expr2), Box::new(expr1))));
                        continue;
                    },
                    Token::Divide => {
                        let expr1 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.pop();
                        let expr2 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.push(HalfParsed::Expression(Expression::Div(Box::new(expr2), Box::new(expr1))));
                        continue;
                    },
                    Token::Add => {
                        let expr1 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.pop();
                        let expr2 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.push(HalfParsed::Expression(Expression::Add(Box::new(expr2), Box::new(expr1))));
                        continue;
                    }
                    Token::Subtract => {
                        let expr1 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.pop();
                        let expr2 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.push(HalfParsed::Expression(Expression::Sub(Box::new(expr2), Box::new(expr1))));
                        continue;
                    },
                    _ => { },
                }
            }
        }

        // Parenthesis + function calls
        if symbol_stack.len() >= 3
            && symbol_stack[symbol_stack.len() - 1] == Token::RParen
            && symbol_stack[symbol_stack.len() - 2].is_expression()
            && symbol_stack[symbol_stack.len() - 3] == Token::LParen
        {
            symbol_stack.pop();
            let expr = symbol_stack.pop().unwrap().expression();
            symbol_stack.pop();
            if let Some(pre) = symbol_stack.last() {
                match pre {
                    HalfParsed::Token(t) => {
                        if let Token::Identifier(name) = t{
                            match *name {
                                // Match Known functions here
                                "sin" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Sin, Box::new(expr))));
                                }
                                "cos" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Cos, Box::new(expr))));
                                }
                                "tan" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Tan, Box::new(expr))));
                                }
                                "arcsin" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Arcsin, Box::new(expr))));
                                }
                                "arccos" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Arccos, Box::new(expr))));
                                }
                                "arctan" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Arctan, Box::new(expr))));
                                }
                                "gamma" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Gamma, Box::new(expr))));
                                }
                                "log" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Log, Box::new(expr))));
                                }
                                "ln" => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Func(Func::Ln, Box::new(expr))));
                                }
                                var => {
                                    symbol_stack.pop();
                                    symbol_stack.push(HalfParsed::Expression(Expression::Mul(Box::new(Expression::Var(var.to_owned())), Box::new(expr))));
                                }
                            }
                        } else {
                            symbol_stack.push(HalfParsed::Expression(expr));
                        }
                    }
                    HalfParsed::Expression(_) => {
                        // Multiplication
                        let expr2 = symbol_stack.pop().unwrap().expression();
                        symbol_stack.push(HalfParsed::Expression(Expression::Mul(Box::new(expr2), Box::new(expr))));
                    }
                }
            } else {
                symbol_stack.push(HalfParsed::Expression(expr));
            }
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
        );
        assert_eq!(
            parse("2(4+5)"),
            Ok(Expression::Mul(Box::new(Expression::Immediate(2.0)), Box::new(Expression::Add(Box::new(Expression::Immediate(4.0)), Box::new(Expression::Immediate(5.0))))))
        )
    }
}
