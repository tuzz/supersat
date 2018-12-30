use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::path::Path;

use crate::variable::Variable;
use crate::clause::Clause;

#[derive(Debug, Eq, PartialEq)]
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

    pub fn generate<F: Fn(&mut Self)>(filename: &String, callback: F) {
        if Path::new(filename).exists() {
            return;
        }

        let mut formula = Self::new();

        callback(&mut formula);

        let file = File::create(filename).unwrap();
        let mut buffer = BufWriter::new(file);

        write!(buffer, "p cnf {} {}\n", formula.variables.len(), formula.clauses.len()).unwrap();

        for clause in &formula.clauses {
            write!(buffer, "{}\n", clause).unwrap();
        }
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
