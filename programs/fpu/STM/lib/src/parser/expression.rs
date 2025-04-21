use crate::parser::util::{tanf64, to_radians};
use alloc::boxed::Box;
use core::fmt::Debug;
use core::intrinsics::{cosf64, sinf64};
use alloc::collections::BTreeMap;
use alloc::string::String;
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
}
