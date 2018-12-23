use std::ops::RangeInclusive;

use crate::variable::Variable;
use crate::formula::Formula;

#[derive(Debug, Eq, PartialEq)]
pub struct Register {
    range: RangeInclusive<usize>,
    variables: Vec<Variable>,
}

impl Register {
    pub fn new(range: RangeInclusive<usize>, formula: &mut Formula) -> Self {
        let variables = range
            .clone()
            .map(|_| formula.new_variable())
            .collect();

        Self { range, variables }
    }

    pub fn variable_for_count(&self, count: usize) -> Option<&Variable> {
        let start = *self.range.start();
        let index = count.checked_sub(start)?;

        self.variables.get(index)
    }
}

#[cfg(test)]
mod test;
