use itertools::Itertools;
use std::iter::repeat;

use crate::literal::Literal;
use crate::binary::Binary;
use crate::machine::Machine;
use crate::goal::Goal;
use crate::bounds::Bounds;
use crate::logic::Logic;

pub struct Problem<'a> {
    n: usize,
    length_of_string: usize,
    machine: &'a Machine,
    goal: &'a Goal,
    bounds: &'a Bounds,
    logic: &'a mut Logic<'a>,
}

impl<'a> Problem<'a> {
    pub fn new(n: usize, length_of_string: usize, machine: &'a Machine, goal: &'a Goal, bounds: &'a Bounds, logic: &'a mut Logic<'a>) -> Self {
        Self { n, length_of_string, machine, goal, bounds, logic }
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
                    self.logic.implies(&transition, travel_to);
                }
            }
        }
    }

    pub fn each_permutation_appears_at_most_once(&mut self) {
        for name in (1..=self.n).permutations(self.n) {
            for time in 1..self.length_of_string {
                let machine_state = self.machine.at_time(time).state(&name);
                let goal_state = self.goal.subgoal(&name).state_by_index(time);

                self.logic.implies(&machine_state.literals(), &goal_state.literals());
            }
        }
    }

    pub fn the_string_starts_with_ascending_numbers(&mut self) {
        let ascending = (1..=self.n).collect::<Vec<_>>();
        self.the_string_starts_with(&ascending);
    }

    pub fn the_string_starts_with(&mut self, symbols: &[usize]) {
        for (time, symbol) in symbols.iter().enumerate() {
            let start_state = self.machine.at_time(time).state(&[*symbol]);
            self.logic.tautology(start_state.literals());
        }
    }

    pub fn the_number_of_wasted_symbols_is_within_bounds(&mut self) {
        let wasted_symbols = self.literals_for_wasted_symbols();
        self.logic.within(&self.bounds, &wasted_symbols);
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

    fn literals_for_wasted_symbols(&mut self) -> Vec<Literal> {
        let range = (self.n - 1)..self.length_of_string;

        range.map(|time| {
            let last_rank = self.n - 1;
            let name = Self::dead_state_name(last_rank);

            let snapshot = self.machine.at_time(time);
            let dead_state = snapshot.state(&name);

            self.logic.alias(dead_state.literals())
        }).collect()
    }
}

#[cfg(test)]
mod test;
