use crate::parser::util::tanf64;
use alloc::boxed::Box;
use core::fmt::Debug;
use core::intrinsics::{cosf64, sinf64};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Immediate(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Trig(Trig, Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Immediate(val) => *val,
            Expression::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            Expression::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expression::Div(lhs, rhs) => lhs.evaluate() / rhs.evaluate(),
            Expression::Trig(f, a) => unsafe {
                match f {
                    Trig::Sin => sinf64(a.evaluate()),
                    Trig::Cos => cosf64(a.evaluate()),
                    Trig::Tan => tanf64(a.evaluate()),
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
