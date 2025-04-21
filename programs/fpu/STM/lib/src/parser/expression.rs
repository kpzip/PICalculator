use crate::parser::util::tanf64;
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
    Trig(Trig, Box<Expression>),
    Var(String),
}

impl Expression {
    pub fn evaluate(&self, lvt: &mut BTreeMap<String, f64>, overrides: &[(String, f64)]) -> Result<f64, ExpressionError> {
        match self {
            Expression::Immediate(val) => Ok(*val),
            Expression::Add(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides)? + rhs.evaluate(lvt, overrides)?),
            Expression::Sub(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides)? - rhs.evaluate(lvt, overrides)?),
            Expression::Mul(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides)? * rhs.evaluate(lvt, overrides)?),
            Expression::Div(lhs, rhs) => Ok(lhs.evaluate(lvt, overrides)? / rhs.evaluate(lvt, overrides)?),
            Expression::Trig(f, a) => unsafe {
                match f {
                    Trig::Sin => Ok(sinf64(a.evaluate(lvt, overrides)?)),
                    Trig::Cos => Ok(cosf64(a.evaluate(lvt, overrides)?)),
                    Trig::Tan => Ok(tanf64(a.evaluate(lvt, overrides)?)),
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
pub enum Trig {
    Sin,
    Cos,
    Tan,
}
