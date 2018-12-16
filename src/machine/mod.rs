use crate::formula::Formula;
use crate::snapshot::Snapshot;

struct Machine {
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
}

#[cfg(test)]
mod test;
