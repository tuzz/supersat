use itertools::Itertools;
use std::iter::repeat;

use crate::literal::Literal;
use crate::binary::Binary;
use crate::machine::Machine;
use crate::goal::Goal;
use crate::counter::Counter;
use crate::logic::Logic;

pub struct Problem<'a> {
    n: usize,
    length_of_string: usize,
    machine: &'a Machine,
    goal: &'a Goal,
    counter: &'a Counter,
    logic: &'a mut Logic<'a>,
    wasted: Option<Vec<Literal>>,
}

impl<'a> Problem<'a> {
    pub fn new(n: usize, length_of_string: usize, machine: &'a Machine, goal: &'a Goal, counter: &'a Counter, logic: &'a mut Logic<'a>) -> Self {
        Self { n, length_of_string, machine, goal, counter, logic, wasted: None }
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
        for time in 0..self.n {
            let symbol = time + 1;
            let state = self.machine.at_time(time).state(&[symbol]);

            self.logic.tautology(state.literals());
        }
    }

    pub fn the_number_of_wasted_symbols_is_within_bounds(&mut self) {
        self.allocate_literals_for_wasted_symbols();

        // (1) if Xi is true then the first bit of register i must be true
        for time in (self.n - 1)..self.length_of_string {
            let wasted = self.is_wasted_symbol(time);
            let register = self.counter.at_time(time);

            if let Some(literal) = register.literal_for_count(1) {
                self.logic.implies(&[wasted], &[literal]);
            }

            // [not included in the paper because 1 <= k <= n does not cover k=0]
            // don't allow wasted symbols if the maximum number allowed is zero
            if register.end() == 0 {
                self.logic.contradiction(&[wasted]);
            }
        }

        // (2) ensures that in the first register only the first bit can be true
        let first_register = self.counter.at_time(self.n - 1);

        for w in 2..=first_register.end() {
            if let Some(literal) = first_register.literal_for_count(w) {
                self.logic.contradiction(&[literal]);
            }
        }

        // (3) and (4) together constrain each register i (1 < i < n) to contain
        // the value of the previous register plus Xi
        for time in self.n..self.length_of_string {
            let current_time = self.counter.at_time(time);
            let previous_time = self.counter.at_time(time - 1);

            let wasted = self.is_wasted_symbol(time);

            // (3)
            for w in previous_time.start()..=current_time.end() {
                let current_bit = match current_time.literal_for_count(w) {
                    Some(literal) => literal,
                    None => continue,
                };

                match previous_time.literal_for_count(w) {
                    Some(previous_bit) => {
                        self.logic.implies(&[previous_bit], &[current_bit])
                    },
                    None => {
                        self.logic.tautology(&[current_bit])
                    },
                }
            }

            // (4)
            for w in 2..=current_time.end() {
                let current_bit = match current_time.literal_for_count(w) {
                    Some(literal) => literal,
                    None => continue,
                };

                let condition = match previous_time.literal_for_count(w - 1) {
                    Some(literal) => Logic::and(&[literal], &[wasted]),
                    None => vec![wasted],
                };

                self.logic.implies(&condition, &[current_bit]);
            }

            // (5) asserts that there can’t be an overflow on any register as it
            // would indicate that more than k variables are true
            let k = current_time.end();
            if let Some(overflow) = previous_time.literal_for_count(k) {
                // I think there's a missing negation in the paper:
                let not_overflow = Logic::negate(&[overflow]);

                self.logic.implies(&[wasted], &not_overflow);
            }
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

    fn is_wasted_symbol(&self, time: usize) -> Literal {
        if let Some(vec) = &self.wasted {
            return vec[time - (self.n - 1)];
        }

        panic!("Must allocate literals for wasted symbols first.");
    }

    fn allocate_literals_for_wasted_symbols(&mut self) {
        let range = (self.n - 1)..self.length_of_string;

        let literals = range.map(|time| {
            let last_rank = self.n - 1;
            let name = Self::dead_state_name(last_rank);

            let snapshot = self.machine.at_time(time);
            let dead_state = snapshot.state(&name);

            self.logic.alias(dead_state.literals())
        }).collect();

        self.wasted = Some(literals);
    }
}

#[cfg(test)]
mod test;
