use std::ops::RangeInclusive;
use std::cmp::max;

use crate::literal::Literal;
use crate::formula::Formula;

#[derive(Debug, Eq, PartialEq)]
pub struct Register {
    range: RangeInclusive<usize>,
    literals: Vec<Literal>,
}

impl Register {
    pub fn new(range: RangeInclusive<usize>, formula: &mut Formula) -> Self {
        let literals = range
            .clone()
            .skip_while(|w| *w == 0)
            .map(|_| formula.new_variable())
            .map(|v| Literal::positive(v))
            .collect();

        Self { range, literals }
    }

    pub fn start(&self) -> usize {
        *self.range.start()
    }

    pub fn end(&self) -> usize {
        *self.range.end()
    }

    pub fn literal_for_count(&self, count: usize) -> Option<Literal> {
        self.literals.get(self.index(count)?).cloned()
    }

    fn index(&self, count: usize) -> Option<usize> {
        let start = max(self.start(), 1);

        count.checked_sub(start)
    }
}

#[cfg(test)]
mod test;
