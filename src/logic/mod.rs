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

    pub fn if_then(&mut self, condition: &[Literal], consequent: &[Literal]) {
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

    pub fn if_all_then(&mut self, conditions: &[&[Literal]], consequent: &[Literal]) {
        let condition = conditions.iter()
            .cloned().flatten()
            .cloned().collect::<Vec<_>>();

        self.if_then(&condition, consequent);
    }

    pub fn negate(literals: &[Literal]) -> Vec<Literal> {
        literals.iter().map(|l| l.negate()).collect()
    }
}

#[cfg(test)]
mod test;
