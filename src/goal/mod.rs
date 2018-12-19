use std::ops::Range;

use crate::variable::Variable;
use crate::formula::Formula;
use crate::state::State;
use crate::rank::Rank;
use crate::utility::Utility;

pub struct Goal {
    ranks: Vec<Rank>,
}

impl Goal {
    pub fn new(n: usize, length_of_string: usize, formula: &mut Formula) -> Self {
        let ranks = (0..Utility::factorial(n))
            .map(|_| Rank::new(length_of_string, formula))
            .collect();

        Self { ranks }
    }

    pub fn subgoal(&self, name: &[usize]) -> &Rank {
        let index = State::index(name, name.len()) - 1;

        &self.ranks[index]
    }

    pub fn invalid_ranges(&self) -> Vec<(Range<usize>, &Vec<Variable>)> {
        self.ranks.iter().map(|r| r.invalid_range()).collect()
    }
}

#[cfg(test)]
mod test;
