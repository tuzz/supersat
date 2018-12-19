use crate::literal::Literal;
use crate::clause::Clause;
use crate::formula::Formula;

pub struct Logic<'a> {
    formula: &'a mut Formula,
}

impl<'a> Logic<'a> {
    pub fn new(formula: &'a mut Formula) -> Self {
        Self { formula }
    }

    pub fn tautology(&mut self, literals: &[Literal]) {
        for literal in literals {
            let mut clause = Clause::new();

            clause.add(*literal);

            self.formula.add_clause(clause);
        }
    }

    pub fn contradiction(&mut self, literals: &[Literal]) {
        for literal in literals {
            let mut clause = Clause::new();

            clause.add(literal.negate());

            self.formula.add_clause(clause);
        }
    }

    pub fn implies(&mut self, condition: &[Literal], consequent: &[Literal]) {
        let mut template = Clause::new();

        for literal in Self::negate(condition) {
            template.add(literal);
        }

        for literal in consequent {
            let mut clause = template.clone();

            clause.add(*literal);

            self.formula.add_clause(clause);
        }
    }

    pub fn and(a: &[Literal], b: &[Literal]) -> Vec<Literal> {
        a.iter().chain(b.iter()).cloned().collect()
    }

    pub fn negate(literals: &[Literal]) -> Vec<Literal> {
        literals.iter().map(|l| l.negate()).collect()
    }
}

#[cfg(test)]
mod test;
