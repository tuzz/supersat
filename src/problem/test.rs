use super::*;
use crate::state::State;
use crate::formula::Formula;

type Subject<'a> = Problem<'a>;

const N: usize = 3;
const LENGTH_OF_STRING: usize = 4;

fn dimacs(formula: &Formula) -> Vec<String> {
    let formatted = format!("{}", formula);

    let mut strings = formatted.lines().skip(1)
        .map(String::from).collect::<Vec<_>>();

    strings.sort();

    strings
}

fn literals(state: &State) -> String {
    state.literals().iter()
        .map(|l| format!("{}", l))
        .collect::<Vec<_>>()
        .join(" ")
}

mod new {
    use super::*;

    #[test]
    fn it_builds_the_struct_with_references_to_the_machine_and_logic() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH_OF_STRING, &mut formula);
        let mut logic = Logic::new(&mut formula);
        let subject = Subject::new(N, &machine, &mut logic);

        assert_eq!(subject.machine, &machine);
    }
}

mod the_machine_starts_in_the_dead_states {
    use super::*;

    #[test]
    fn it_adds_clauses_that_set_the_machine_to_the_dead_states_at_time_zero() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH_OF_STRING, &mut formula);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, &machine, &mut logic);

        subject.the_machine_starts_in_the_dead_states();

        let dead_state_1 = machine.at_time(0).state(&[0, 0]);
        let dead_state_2 = machine.at_time(0).state(&[0, 0, 0]);

        // Look up the literals for the dead states so we know what to assert.
        assert_eq!(literals(dead_state_1), "-3 -4 -5");
        assert_eq!(literals(dead_state_2), "-6 -7 -8");

        assert_eq!(dimacs(&formula), &[
            "-3 0", "-4 0", "-5 0",  // We're in dead state 1.
            "-6 0", "-7 0", "-8 0",  // We're in dead state 2.
        ]);
    }
}
