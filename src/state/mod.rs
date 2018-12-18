use std::collections::HashSet;

use crate::binary::Binary;
use crate::variable::Variable;
use crate::literal::Literal;
use crate::utility::Utility;

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    binary: Binary,
}

impl State {
    pub fn new(index: usize, variables: &[Variable]) -> Self {
        Self { binary: Binary::from_decimal(index, variables) }
    }

    pub fn literals(&self) -> &Vec<Literal> {
        &self.binary.bits
    }

    pub fn index(name: &[usize], n: usize) -> usize {
        if Self::is_dead_state(name) {
            return 0;
        }

        let mut seen = HashSet::new();
        let iter = name.iter().take(n - 1);
        let divisor = Utility::factorial(n - name.len());

        iter.clone().enumerate().map(|(index, symbol)| {
            let smaller = Self::count_smaller(*symbol, &mut seen);
            let position = n - index - 1;

            let radix = Utility::factorial(position) / divisor;
            let digit = symbol - smaller - 1;

            radix * digit
        }).sum::<usize>() + Self::dead_state_offset(name)
    }

    fn count_smaller(symbol: usize, set: &mut HashSet<usize>) -> usize {
        let count = set.iter().filter(|s| **s < symbol).count();

        set.insert(symbol);

        count
    }

    fn is_dead_state(name: &[usize]) -> bool {
        name.iter().all(|s| *s == 0)
    }

    fn dead_state_offset(name: &[usize]) -> usize {
        match name.len() { 1 => 0, _ => 1 }
    }
}

#[cfg(test)]
mod test;
