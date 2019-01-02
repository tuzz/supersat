use crate::literal::Literal;
use crate::clause::Clause;
use crate::formula::Formula;

use crate::bounds::Bounds;
use crate::counter::Counter;

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
        let mut clause = Clause::new();

        for literal in literals {
            clause.add(literal.negate());
        }

        self.formula.add_clause(clause);
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

    pub fn within(&mut self, bounds: &Bounds, literals: &[Literal]) {
        let counter = Counter::new(bounds, &mut self.formula);

        // (1) if Xi is true then the first bit of register i must be true
        for (index, &literal) in literals.iter().enumerate() {
            let register = counter.register(index);

            if let Some(count_bit) = register.literal_for_count(1) {
                self.implies(&[literal], &[count_bit]);
            }

            // [not included in the paper because 1 <= k <= n does not cover k=0]
            // don't allow wasted symbols if the maximum number allowed is zero
            if register.end() == 0 {
                self.contradiction(&[literal]);
            }
        }

        // (2) ensures that in the first register only the first bit can be true
        let first_register = counter.register(0);

        for w in 2..=first_register.end() {
            if let Some(count_bit) = first_register.literal_for_count(w) {
                self.contradiction(&[count_bit]);
            }
        }

        // (3) and (4) together constrain each register i (1 < i < n) to contain
        // the value of the previous register plus Xi
        for (index, &literal) in literals.iter().enumerate().skip(1) {
            let current_register = counter.register(index);
            let previous_register = counter.register(index - 1);

            // (3)
            for b in previous_register.start()..=current_register.end() {
                let previous_bit = match previous_register.literal_for_count(b) {
                    Some(literal) => literal,
                    None => continue,
                };

                match current_register.literal_for_count(b) {
                    Some(current_bit) => {
                        self.implies(&[previous_bit], &[current_bit]);
                    },
                    None => {
                        self.tautology(&[previous_bit]);
                    },
                }
            }

            // (4)
            for b in 2..=current_register.end() {
                let current_bit = match current_register.literal_for_count(b) {
                    Some(literal) => literal,
                    None => continue,
                };

                if let Some(previous_bit) = previous_register.literal_for_count(b - 1) {
                    let condition = Logic::and(&[previous_bit], &[literal]);
                    self.implies(&condition, &[current_bit]);
                }
            }

            // (5) asserts that there can’t be an overflow on any register as it
            // would indicate that more than k variables are true
            let k = current_register.end();
            if let Some(overflow) = previous_register.literal_for_count(k) {
                // I think there's a missing negation in the paper:
                self.implies(&[literal], &[overflow.negate()]);
            }
        }
    }

    pub fn alias(&mut self, literals: &[Literal]) -> Literal {
        let variable = self.formula.new_variable();
        let literal = Literal::positive(variable);

        self.implies(literals, &[literal]);
        self.implies(&[literal], literals);

        literal
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
