mod variable;
mod literal;
mod clause;
mod formula;
mod binary;
mod state;
mod rank;
mod snapshot;
mod machine;
mod bounds;
mod register;
mod counter;
mod goal;
mod logic;
mod problem;
mod solver;
mod utility;

use std::fs::create_dir_all;
use std::cmp::min;

use crate::formula::Formula;
use crate::machine::Machine;
use crate::goal::Goal;
use crate::bounds::Bounds;
use crate::logic::Logic;
use crate::problem::Problem;
use crate::solver::Solver;
use crate::utility::Utility;

fn main() {
    create_dir_all("scratch").unwrap();

    for n in 2..=6 {
        println!("------------------------------------------------------------");
        println!("Trying to find superpermutations for {} symbols.", n);
        println!("------------------------------------------------------------");
        println!();

        let mut max_permutations = vec![];

        let mut permutations = max_permutations.last().unwrap_or(&0) + n;
        let mut wasted_symbols;
        let mut length_of_string;

        loop {
            wasted_symbols = n - 1 + max_permutations.len();
            length_of_string = permutations + wasted_symbols;

            let filename = format!("scratch/{}-symbols-{}-perms-{}-waste.dimacs", n, permutations, wasted_symbols);

            Formula::generate(&filename, |formula| {
                println!("Generating {}...", filename);

                let machine = Machine::new(n, length_of_string, formula);
                let goal = Goal::new(n, length_of_string, formula);
                let bounds = Bounds::new(n, length_of_string, &max_permutations);
                let mut logic = Logic::new(formula);
                let mut problem = Problem::new(n, length_of_string, &machine, &goal, &bounds, &mut logic);

                problem.the_machine_starts_in_the_dead_states();
                problem.the_machine_changes_state_when_it_reads_input();
                problem.each_permutation_appears_at_most_once();
                problem.the_string_starts_with_ascending_numbers();
                problem.all_binary_representations_map_to_states();
                problem.the_number_of_wasted_symbols_is_within_bounds();
            });

            print!("Searching for a string of length {} that contains ", length_of_string);
            println!("{} permutations and wastes {} symbols...", permutations, wasted_symbols);

            if Solver::solve(&filename) {
                max_permutations.push(permutations);
                println!("Setting max permutations to {:?}", max_permutations);

                if permutations == Utility::factorial(n) {
                    break;
                }

                permutations = min(permutations + n, Utility::factorial(n));
            } else {
                permutations -= 1;
                println!("None exist, backtracking by one permutation.");
            }

            println!();
        }

        println!();
        println!("The shortest superpermutation for {} symbols is {}.", n, length_of_string);
        println!();
    }
}
