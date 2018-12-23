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
mod goal;
mod logic;
mod problem;

mod utility;

use crate::formula::Formula;
use crate::machine::Machine;
use crate::goal::Goal;
use crate::logic::Logic;
use crate::problem::Problem;

const N: usize = 3;
const LENGTH_OF_STRING: usize = 9;

fn main() {
    let mut formula = Formula::new();

    let machine = Machine::new(N, LENGTH_OF_STRING, &mut formula);
    let goal = Goal::new(N, LENGTH_OF_STRING, &mut formula);

    let mut logic = Logic::new(&mut formula);
    let mut problem = Problem::new(N, LENGTH_OF_STRING, &machine, &goal, &mut logic);

    problem.the_machine_starts_in_the_dead_states();
    problem.the_machine_changes_state_when_it_reads_input();
    problem.the_goal_to_include_all_permutations_is_met();
    problem.the_string_starts_with_ascending_numbers();
    problem.all_binary_representations_map_to_states();

    println!("{}", formula);
}
