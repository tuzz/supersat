use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};

use crate::variable::Variable;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Clause {
    positive_literals: BTreeSet<Variable>,
    negative_literals: BTreeSet<Variable>,
}

impl Clause {
    pub fn new() -> Self {
        let positive_literals = BTreeSet::new();
        let negative_literals = BTreeSet::new();

        Self { positive_literals, negative_literals }
    }

    pub fn positive(&mut self, variable: Variable) {
        self.positive_literals.insert(variable);
    }

    pub fn negative(&mut self, variable: Variable) {
        self.negative_literals.insert(variable);
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for variable in &self.positive_literals {
            write!(f, "{} ", variable)?;
        }

        for variable in &self.negative_literals {
            write!(f, "-{} ", variable)?;
        }

        write!(f, "0")
    }
}

#[cfg(test)]
mod test;
