use std::collections::HashSet;
use crate::variable::Variable;

pub struct Clause {
    positive_literals: HashSet<Variable>,
    negative_literals: HashSet<Variable>,
}

impl Clause {
    pub fn new() -> Self {
        let positive_literals = HashSet::new();
        let negative_literals = HashSet::new();

        Self { positive_literals, negative_literals }
    }

    pub fn positive(&mut self, variable: &Variable) {
        self.positive_literals.insert(variable.clone());
    }

    pub fn negative(&mut self, variable: &Variable) {
        self.negative_literals.insert(variable.clone());
    }
}

#[cfg(test)]
mod test;
