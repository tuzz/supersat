use std::collections::HashSet;

use crate::variable::Variable;
use crate::clause::Clause;

pub struct Formula {
    variables: HashSet<Variable>,
    clauses: HashSet<Clause>,
}

impl Formula {
    fn new() -> Self {
        let variables = HashSet::new();
        let clauses = HashSet::new();

        Self { variables, clauses }
    }

    fn new_variable(&mut self) -> Variable {
        let number = self.variables.len() + 1;
        let variable = Variable::new(number);

        self.variables.insert(variable);

        variable
    }

    fn add_clause(&mut self, clause: Clause) {
        self.clauses.insert(clause);
    }
}

#[cfg(test)]
mod test;
