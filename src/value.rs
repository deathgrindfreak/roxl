use crate::error::InterpretError;

use std::fmt;
use std::str::FromStr;
use std::num::ParseFloatError;
use std::ops::{Add, Sub, Mul, Neg, Div};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectType {
    Str(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Object(ObjectType),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Object(ObjectType::Str(s)) => write!(f, "\"{}\"", s),
        }
    }
}

impl Add<Value> for Value {
    type Output = Result<Self, InterpretError>;

    fn add(self, o: Value) -> Self::Output {
        match (self, o) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::Object(ObjectType::Str(s1)), Value::Object(ObjectType::Str(s2))) => {
                Ok(Value::Object(ObjectType::Str(s1 + &s2)))
            },
            _ => Err(InterpretError::ValueError("Can only add 2 number or string values")),
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

    // NOTE: Right now we only try to parse numeric strings into Values
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Value::Number(s.parse::<f64>()?))
    }
}
