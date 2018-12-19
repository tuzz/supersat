use crate::formula::Formula;
use crate::state::State;
use crate::rank::Rank;

#[derive(Debug, Eq, PartialEq)]
pub struct Snapshot {
    ranks: Vec<Rank>,
}

impl Snapshot {
    pub fn new(n: usize, formula: &mut Formula) -> Self {
        let ranks = (0..n)
            .map(|i| Rank::new(Self::number_of_states(i, n), formula))
            .collect();

        Self { ranks }
    }

    pub fn state(&self, name: &[usize]) -> &State {
        let rank = &self.ranks[name.len() - 1];
        let n = self.ranks.len();

        rank.state(name, n)
    }

    pub fn max_states(&self) -> Vec<&State> {
        self.ranks.iter().map(|r| r.max_state()).collect()
    }

    fn number_of_states(index: usize, n: usize) -> usize {
        let states = (0..=index).map(|i| (n - i)).product::<usize>();

        match index {
            0 => states,
            _ => states + 1,
        }
    }

}

#[cfg(test)]
mod test;
