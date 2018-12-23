use std::ops::RangeInclusive;

use crate::variable::Variable;
use crate::formula::Formula;

struct Register {
    range: RangeInclusive<usize>,
    variables: Vec<Variable>,
}

impl Register {
    fn new(range: RangeInclusive<usize>, formula: &mut Formula) -> Self {
        let variables = range
            .clone()
            .map(|_| formula.new_variable())
            .collect();

        Self { range, variables }
    }

    fn variable_for_count(&self, count: usize) -> Option<&Variable> {
        let start = *self.range.start();
        let index = count.checked_sub(start)?;

        self.variables.get(index)
    }
}

#[cfg(test)]
mod test;
