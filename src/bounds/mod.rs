use std::ops::RangeInclusive;
use std::cmp::min;

use crate::utility::Utility;

pub struct Bounds {
    n: usize,
    length_of_string: usize,
    max_permutations: Vec<usize>,
}

impl Bounds {
    pub fn new(n: usize, length_of_string: usize, max_permutations: &[usize]) -> Self {
        let max_permutations = max_permutations.to_vec();

        Self { n, length_of_string, max_permutations }
    }

    pub fn allowed_waste(&self, index: usize) -> RangeInclusive<usize> {
        let mut wasted_symbols = vec![];

        // println!("trying to reach goal of {} permutations", self.goal_permutations());
        // println!();

        for w in 0..=self.goal_wasted_symbols() {
            let current_permutations = match self.number_of_permutations(index, w) {
                Some(p) => p,
                None => continue,
            };

            // Skip wasted symbols that can't produce the current permutations.
            if w < self.minimum_wasted_symbols(current_permutations) {
                continue;
            }

            // Calculate the maximum number of permutations there were when the
            // last symbol in the current string was wasted.
            let checkpoint_permutations = self.max_perms_at_last_waste(w, current_permutations);

            // Calculate how many wasted symbols can be used to reach the goal.
            let surplus_wasted_symbols = self.goal_wasted_symbols() - w;

            // Calculate the number of permutations that could be added with the
            // surplus wasted symbols.
            let additional_permutations = self.max_perms_added(surplus_wasted_symbols, checkpoint_permutations);

            // Calculate the maximum number of permutations we could reach from
            // the checkpoint with the additional permutations.
            let furthest_permutations = checkpoint_permutations + additional_permutations;

            // println!("the furthest permutations for index {} and waste {} is {}", index, w, furthest_permutations);
            // println!("  - current_permutations: {}", current_permutations);
            // println!("  - checkpoint_permutations: {}", checkpoint_permutations);
            // println!("  - surplus_wasted_symbols: {}", surplus_wasted_symbols);
            // println!("  - additional_permutations: {}", additional_permutations);
            // println!();

            // Push the current wasted symbols if the goal can be reached.
            if furthest_permutations >= self.goal_permutations() {
                wasted_symbols.push(w)
            }

        }

        *wasted_symbols.first().unwrap()..=*wasted_symbols.last().unwrap()
    }

    fn max_perms_at_last_waste(&self, wasted_symbols: usize, current_permutations: usize) -> usize {
        if wasted_symbols == 0 {
            return 0;
        }

        let last_waste = wasted_symbols - 1;
        let max_permutations = self.max_permutations[last_waste];

        min(max_permutations, current_permutations)
    }

    fn max_perms_added(&self, wasted_symbols: usize, current_permutations: usize) -> usize {
        let all_permutations = Utility::factorial(self.n);
        let remainder = all_permutations - current_permutations;

        if wasted_symbols >= self.max_permutations.len() {
            return remainder;
        }

        let max_permutations = self.max_permutations[wasted_symbols];

        min(max_permutations, remainder)
    }

    fn goal_index(&self) -> usize {
        self.length_of_string - 1
    }

    fn goal_wasted_symbols(&self) -> usize {
        self.max_permutations.len()
    }

    fn goal_permutations(&self) -> usize {
        self.number_of_permutations(self.goal_index(), self.goal_wasted_symbols()).unwrap()
    }

    fn number_of_permutations(&self, index: usize, wasted_symbols: usize) -> Option<usize> {
        let leading_symbols = self.n - 1;
        let total_waste = leading_symbols + wasted_symbols;

        index.checked_sub(total_waste - 1)
    }

    fn minimum_wasted_symbols(&self, permutations: usize) -> usize {
        let len = self.max_permutations.len();

        self.max_permutations.iter()
            .position(|p| *p >= permutations)
            .unwrap_or(len)
    }
}

#[cfg(test)]
mod test;
