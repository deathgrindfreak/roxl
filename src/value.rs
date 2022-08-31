use crate::error::InterpretError;

use std::fmt;
use std::str::FromStr;
use std::num::ParseFloatError;
use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Add<Value> for Value {
    type Output = Result<Self, InterpretError>;

    fn add(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            _ => Err(InterpretError::ValueError("Can only add 2 number values")),
        }
    }
}

impl Sub<Value> for Value {
    type Output = Result<Self, InterpretError>;

    fn sub(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            _ => Err(InterpretError::ValueError("Can only subtract 2 number values")),
        }
    }
}

impl Mul<Value> for Value {
    type Output = Result<Self, InterpretError>;

    fn mul(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(InterpretError::ValueError("Can only multiply 2 number values")),
        }
    }
}

impl Div<Value> for Value {
    type Output = Result<Self, InterpretError>;

    fn div(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            _ => Err(InterpretError::ValueError("Can only divide 2 number values")),
        }
    }
}

impl Neg for Value {
    type Output = Result<Self, InterpretError>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(InterpretError::ValueError("Can only negate number values")),
        }
    }
}

impl FromStr for Value {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "nil" => Value::Nil,
                _ => Value::Number(s.parse::<f64>()?)
            }
        )
    }
}
