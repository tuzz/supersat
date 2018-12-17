use crate::literal::Literal;
use crate::clause::Clause;
use crate::formula::Formula;

struct Logic<'a> {
    formula: &'a mut Formula,
}

impl<'a> Logic<'a> {
    pub fn new(formula: &'a mut Formula) -> Self {
        Self { formula }
    }

    pub fn at_least_one(&mut self, literals: &[Literal]) {
        let mut clause = Clause::new();

        for literal in literals {
            clause.add(*literal);
        }

        self.formula.add_clause(clause);
    }

    pub fn all_of_them(&mut self, literals: &[Literal]) {
        for literal in literals {
            let mut clause = Clause::new();

            clause.add(*literal);

            self.formula.add_clause(clause);
        }
    }
}

#[cfg(test)]
mod test;
