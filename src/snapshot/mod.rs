use crate::formula::Formula;
use crate::state::State;
use crate::rank::Rank;

pub struct Snapshot {
    ranks: Vec<Rank>,
}

impl Snapshot {
    pub fn new(n: usize, formula: &mut Formula) -> Self {
        let ranks = (0..n).map(|i| Rank::new(i, n, formula)).collect();

        Self { ranks }
    }

    pub fn state(&self, name: &[usize]) -> &State {
        let rank = &self.ranks[name.len() - 1];
        let n = self.ranks.len();

        rank.state(name, n)
    }
}

#[cfg(test)]
mod test;