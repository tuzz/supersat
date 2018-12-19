use std::ops::Range;

use crate::variable::Variable;
use crate::formula::Formula;
use crate::state::State;

#[derive(Debug, Eq, PartialEq)]
pub struct Rank {
    variables: Vec<Variable>,
    states: Vec<State>,
}

impl Rank {
    pub fn new(number_of_states: usize, formula: &mut Formula) -> Self {
        let number_of_variables = Self::log_2_ceil(number_of_states);

        let variables = (0..number_of_variables)
            .map(|_| formula.new_variable())
            .collect::<Vec<_>>();

        let states = (0..number_of_states)
            .map(|i| State::new(i, &variables))
            .collect();

        Self { variables, states }
    }

    pub fn state(&self, name: &[usize], n: usize) -> &State {
        self.state_by_index(State::index(name, n))
    }

    pub fn state_by_index(&self, index: usize) -> &State {
        &self.states[index]
    }

    pub fn invalid_range(&self) -> (Range<usize>, &Vec<Variable>) {
        let min_invalid = self.states.len();
        let max_invalid = Self::capacity(self.variables.len());

        (min_invalid..max_invalid, &self.variables)
    }

    fn log_2_ceil(number: usize) -> usize {
        let mut bits = 1;

        while Self::capacity(bits) < number {
            bits += 1;
        }

        bits as usize
    }

    fn capacity(bits: usize) -> usize {
        2_usize.pow(bits as u32)
    }
}

#[cfg(test)]
mod test;
