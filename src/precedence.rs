use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Add<usize> for Precedence {
    type Output = Self;

    fn add(self, u: usize) -> Self::Output {
        let r: usize = self.into();
        (r + u).try_into().unwrap_or(self)
    }
}

impl Sub<usize> for Precedence {
    type Output = Self;

    fn sub(self, u: usize) -> Self::Output {
        let r: usize = self.into();
        (r - u).try_into().unwrap_or(self)
    }
}

impl From<Precedence> for usize {
    fn from(value: Precedence) -> usize {
        match value {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 2,
            Precedence::And => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
        }
    }
}

impl TryFrom<usize> for Precedence {
    type Error = ();

    fn try_from(value: usize) -> Result<Precedence, ()> {
        match value {
            0 => Ok(Precedence::None),
            1 => Ok(Precedence::Assignment),
            2 => Ok(Precedence::Or),
            3 => Ok(Precedence::And),
            4 => Ok(Precedence::Equality),
            5 => Ok(Precedence::Comparison),
            6 => Ok(Precedence::Term),
            7 => Ok(Precedence::Factor),
            8 => Ok(Precedence::Unary),
            9 => Ok(Precedence::Call),
            10 => Ok(Precedence::Primary),
            // Really shouldn't have to be used, since the error is captured in Add and Sub
            _ => Err(())
        }
    }
}
