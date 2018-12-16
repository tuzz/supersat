use std::collections::HashSet;

use crate::binary::Binary;
use crate::variable::Variable;

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    binary: Binary,
}

impl State {
    pub fn new(index: usize, variables: &[Variable]) -> Self {
        Self { binary: Binary::from_decimal(index, variables) }
    }

    pub fn index(name: &[usize], n: usize) -> usize {
        if Self::is_dead_state(name) {
            return 0;
        }

        let mut seen = HashSet::new();
        let iter = name.iter().take(n - 1);

        iter.clone().enumerate().map(|(index, symbol)| {
            let smaller = Self::count_smaller(*symbol, &mut seen);
            let position = iter.len() - index;

            let radix = Self::factorial(position);
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

    fn factorial(n: usize) -> usize {
        match n { 0 => 1, _ => n * Self::factorial(n - 1) }
    }
}

#[cfg(test)]
mod test;
