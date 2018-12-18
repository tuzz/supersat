use crate::formula::Formula;
use crate::state::State;
use crate::rank::Rank;
use crate::utility::Utility;

struct Goal {
    ranks: Vec<Rank>,
}

impl Goal {
    fn new(n: usize, length_of_string: usize, formula: &mut Formula) -> Self {
        let ranks = (0..Utility::factorial(n))
            .map(|i| Rank::new(i, length_of_string, formula))
            .collect();

        Self { ranks }
    }

    fn subgoal(&self, name: &[usize]) -> &Rank {
        let index = State::index(name, name.len()) - 1;

        &self.ranks[index]
    }

    pub fn max_states(&self) -> Vec<&State> {
        self.ranks.iter().map(|r| r.max_state()).collect()
    }
}

#[cfg(test)]
mod test;
