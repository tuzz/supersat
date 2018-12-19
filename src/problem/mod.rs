use itertools::Itertools;
use std::iter::repeat;

use crate::binary::Binary;
use crate::machine::Machine;
use crate::goal::Goal;
use crate::logic::Logic;

pub struct Problem<'a> {
    n: usize,
    length_of_string: usize,
    machine: &'a Machine,
    goal: &'a Goal,
    logic: &'a mut Logic<'a>,
}

impl<'a> Problem<'a> {
    pub fn new(n: usize, length_of_string: usize, machine: &'a Machine, goal: &'a Goal, logic: &'a mut Logic<'a>) -> Self {
        Self { n, length_of_string, machine, goal, logic }
    }

    pub fn the_machine_starts_in_the_dead_states(&mut self) {
        for rank in 1..self.n {
            let name = Self::dead_state_name(rank);
            let dead_state = self.machine.at_time(0).state(&name);

            self.logic.tautology(dead_state.literals());
        }
    }

    pub fn the_machine_changes_state_when_it_reads_input(&mut self) {
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

    pub fn the_goal_to_include_all_permutations_is_met(&mut self) {
        for name in (1..=self.n).permutations(self.n) {
            for time in 0..self.length_of_string {
                let machine_state = self.machine.at_time(time).state(&name);
                let goal_state = self.goal.subgoal(&name).state_by_index(time);

                self.logic.implies(&goal_state.literals(), &machine_state.literals());
            }
        }
    }

    pub fn the_string_starts_with_ascending_numbers(&mut self) {
        for time in 0..self.n {
            let symbol = time + 1;
            let state = self.machine.at_time(time).state(&[symbol]);

            self.logic.tautology(state.literals());
        }
    }

    pub fn all_binary_representations_map_to_states(&mut self) {
        for (range, variables) in self.machine.invalid_ranges() {
            for number in range {
                let binary = Binary::from_decimal(number, &variables);
                self.logic.contradiction(&binary.bits);
            }
        }

        for (range, variables) in self.goal.invalid_ranges() {
            for number in range {
                let binary = Binary::from_decimal(number, &variables);
                self.logic.contradiction(&binary.bits);
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
