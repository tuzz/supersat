use std::collections::BTreeSet;
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

#[cfg(test)]
mod test;
