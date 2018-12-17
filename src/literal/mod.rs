use crate::variable::Variable;

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
}

#[cfg(test)]
mod test;
