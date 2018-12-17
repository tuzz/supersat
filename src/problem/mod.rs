use std::iter::repeat;

use crate::machine::Machine;
use crate::logic::Logic;

struct Problem<'a> {
    n: usize,
    machine: &'a Machine,
    logic: &'a mut Logic<'a>,
}

impl<'a> Problem<'a> {
    fn new(n: usize, machine: &'a Machine, logic: &'a mut Logic<'a>) -> Self {
        Self { n, machine, logic }
    }

    fn the_machine_starts_in_the_dead_states(&mut self) {
        for rank in 1..self.n {
            let name = Self::dead_state_name(rank);
            let dead_state = self.machine.at_time(0).state(&name);

            self.logic.all_of_them(dead_state.literals());
        }
    }

    fn dead_state_name(rank: usize) -> Vec<usize> {
        if rank == 0 {
            panic!("There is no dead state in the first rank.");
        }

        repeat(0).take(rank + 1).collect()
    }
}

#[cfg(test)]
mod test;
