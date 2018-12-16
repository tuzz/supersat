use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Variable {
    pub number: usize,
}

impl Variable {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.number.to_string())
    }
}

#[cfg(test)]
mod test;
