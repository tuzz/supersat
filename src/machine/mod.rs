use std::ops::Range;

use crate::variable::Variable;
use crate::formula::Formula;
use crate::snapshot::Snapshot;

#[derive(Debug, Eq, PartialEq)]
pub struct Machine {
    snapshots: Vec<Snapshot>,
}

impl Machine {
    pub fn new(n: usize, length_of_string: usize, formula: &mut Formula) -> Self {
        let snapshots = (0..length_of_string)
            .map(|_| Snapshot::new(n, formula)).collect();

        Self { snapshots }
    }

    pub fn at_time(&self, point_in_time: usize) -> &Snapshot {
        &self.snapshots[point_in_time]
    }

    pub fn invalid_ranges(&self) -> Vec<(Range<usize>, &Vec<Variable>)> {
        self.snapshots.iter().flat_map(|s| s.invalid_ranges()).collect()
    }
}

#[cfg(test)]
mod test;
