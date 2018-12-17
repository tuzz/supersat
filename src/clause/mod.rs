use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};

use crate::variable::Variable;
use crate::literal::Literal;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Clause {
    literals: BTreeSet<Literal>,
}

impl Clause {
    pub fn new() -> Self {
        Self { literals: BTreeSet::new() }
    }

    pub fn add(&mut self, literal: Literal) {
        self.literals.insert(literal);
    }

    pub fn positive(&mut self, variable: Variable) {
        self.add(Literal::positive(variable));
    }

    pub fn negative(&mut self, variable: Variable) {
        self.add(Literal::negative(variable));
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for literal in &self.literals {
            write!(f, "{} ", literal)?;
        }

        write!(f, "0")
    }
}

#[cfg(test)]
mod test;
