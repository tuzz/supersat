use crate::variable::Variable;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Literal {
    pub variable: Variable,
    pub positive: bool,
}

impl Literal {
    pub fn new(variable: Variable, positive: bool) -> Self {
        Self { variable, positive }
    }

    pub fn positive(variable: Variable) -> Self {
        Self::new(variable, true)
    }

    pub fn negative(variable: Variable) -> Self {
        Self::new(variable, false)
    }

    pub fn negate(&self) -> Self {
        Self::new(self.variable, !self.positive)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.positive {
            true => write!(f, "{}", self.variable),
            false => write!(f, "-{}", self.variable)
        }
    }
}

#[cfg(test)]
mod test;
