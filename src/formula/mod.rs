use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

use crate::variable::Variable;
use crate::clause::Clause;

pub struct Formula {
    variables: HashSet<Variable>,
    clauses: HashSet<Clause>,
}

impl Formula {
    pub fn new() -> Self {
        let variables = HashSet::new();
        let clauses = HashSet::new();

        Self { variables, clauses }
    }

    pub fn new_variable(&mut self) -> Variable {
        let number = self.variables.len() + 1;
        let variable = Variable::new(number);

        self.variables.insert(variable);

        variable
    }

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.insert(clause);
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "p cnf {} {}\n", self.variables.len(), self.clauses.len())?;

        for clause in &self.clauses {
            write!(f, "{}\n", clause)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test;
