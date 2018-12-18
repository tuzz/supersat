use itertools::Itertools;
use std::iter::repeat;

use crate::machine::Machine;
use crate::logic::Logic;

struct Problem<'a> {
    n: usize,
    length_of_string: usize,
    machine: &'a Machine,
    logic: &'a mut Logic<'a>,
}

impl<'a> Problem<'a> {
    fn new(n: usize, length_of_string: usize, machine: &'a Machine, logic: &'a mut Logic<'a>) -> Self {
        Self { n, length_of_string, machine, logic }
    }

    fn the_machine_starts_in_the_dead_states(&mut self) {
        for rank in 1..self.n {
            let name = Self::dead_state_name(rank);
            let dead_state = self.machine.at_time(0).state(&name);

            self.logic.tautology(dead_state.literals());
        }
    }

    fn the_machine_changes_state_when_it_reads_input(&mut self) {
        for time in 1..self.length_of_string {
            let current_time = self.machine.at_time(time);
            let previous_time = self.machine.at_time(time - 1);

            for rank in 2..=self.n {
                for to_name in (1..=self.n).permutations(rank) {
                    let last = to_name.len() - 1;
                    let (from_name, symbol_name) = to_name.split_at(last);

                    let travel_from = previous_time.state(from_name).literals();
                    let travel_to = current_time.state(&to_name).literals();
                    let on_symbol = current_time.state(symbol_name).literals();

                    let transition = Logic::and(travel_from, on_symbol);

                    self.logic.implies(travel_to, &transition);
                }
            }
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
