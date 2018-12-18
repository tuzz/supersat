mod variable;
mod literal;
mod clause;
mod formula;

mod binary;
mod state;
mod rank;
mod snapshot;
mod machine;

mod logic;
mod problem;

mod utility;

fn main() {
    let mut formula = crate::formula::Formula::new();
    let machine = crate::machine::Machine::new(3, 9, &mut formula);
    let mut logic = crate::logic::Logic::new(&mut formula);
    let mut problem = crate::problem::Problem::new(3, 9, &machine, &mut logic);

    problem.the_machine_starts_in_the_dead_states();
    problem.the_machine_changes_state_when_it_reads_input();
    problem.the_machine_sees_every_final_state();

    problem.the_string_starts_with_ascending_numbers();

    println!("{}", formula);
}
