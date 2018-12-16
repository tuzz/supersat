use crate::variable::Variable;
use crate::formula::Formula;
use crate::state::State;

#[derive(Debug, Eq, PartialEq)]
pub struct Rank {
    variables: Vec<Variable>,
    states: Vec<State>,
}

impl Rank {
    pub fn new(index: usize, n: usize, formula: &mut Formula) -> Self {
        let mut number_of_states = (0..=index).map(|i| (n - i)).product::<usize>();
        if index > 0 { number_of_states += 1; }

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
        let index = State::index(name, n);

        &self.states[index]
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
