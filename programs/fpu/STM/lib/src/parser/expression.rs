use crate::parser::util::{tanf64, to_radians};
use alloc::boxed::Box;
use core::fmt::Debug;
use core::intrinsics::{cosf64, sinf64};
use alloc::collections::BTreeMap;
use alloc::string::{ParseError, String};
use std::intrinsics::{log10f64, logf64};
use crate::parser::ExpressionError;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Immediate(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Func(Func, Box<Expression>),
    Var(String),
}

impl Expression {
    pub fn evaluate(&self, lvt: &mut BTreeMap<String, f64>, overrides: &[(String, f64)], degrees: bool) -> Result<f64, ExpressionError> {
        match self {
            Expression::Immediate(val) => Ok(*val),
            Expression::Add(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides, degrees)? + rhs.evaluate(lvt, overrides, degrees)?),
            Expression::Sub(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides, degrees)? - rhs.evaluate(lvt, overrides, degrees)?),
            Expression::Mul(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides, degrees)? * rhs.evaluate(lvt, overrides, degrees)?),
            Expression::Div(lhs, rhs) => {
                let rhs = rhs.evaluate(lvt, overrides, degrees)?;
                if rhs == 0.0 {
                    Err(ExpressionError::DivisionByZero)
                } else {
                    Ok(lhs.evaluate(lvt, overrides, degrees)? / rhs)
                }
            },
            Expression::Func(f, a) => unsafe {
                let inner = a.evaluate(lvt, overrides, degrees)?;
                let inner_rad = if degrees { to_radians(inner) } else { inner };
                match f {
                    Func::Sin => Ok(sinf64(inner_rad)),
                    Func::Cos => Ok(cosf64(inner_rad)),
                    Func::Tan => Ok(tanf64(inner_rad)),
                    Func::Arcsin => { Err(ExpressionError::InvalidSyntax(0)) }
                    Func::Arccos => { Err(ExpressionError::InvalidSyntax(0)) }
                    Func::Arctan => { Err(ExpressionError::InvalidSyntax(0)) }
                    Func::Gamma => { Err(ExpressionError::InvalidSyntax(0)) }
                    Func::Log => { Ok(log10f64(inner))}
                    Func::Ln => { Ok(logf64(inner)) }
                }
            },
            Expression::Var(name) => {
                let overrided = overrides.iter().find(|o| o.0 == *name);
                if let Some(value) = overrided {
                    Ok(value.1)
                } else {
                    lvt.get(name).copied().ok_or(ExpressionError::UnknownVariable(name.clone()))
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Gamma,
    Log,
    Ln,
}
