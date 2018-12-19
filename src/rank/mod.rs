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

    pub fn max_state(&self) -> &State {
        self.states.last().unwrap()
    }

    fn log_2_ceil(number: usize) -> usize {
        let mut bits = 1;

        while 2_usize.pow(bits) < number {
            bits += 1;
        }

        bits as usize
    }
}

#[cfg(test)]
mod test;
