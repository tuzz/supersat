use super::*;
use crate::state::State;
use crate::formula::Formula;

type Subject<'a> = Problem<'a>;

const N: usize = 2;
const LENGTH: usize = 3;

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
        let machine = Machine::new(N, LENGTH, &mut formula);
        let mut logic = Logic::new(&mut formula);
        let subject = Subject::new(N, LENGTH, &machine, &mut logic);

        assert_eq!(subject.machine, &machine);
    }
}

mod the_machine_starts_in_the_dead_states {
    use super::*;

    #[test]
    fn it_adds_clauses_that_set_the_machine_to_the_dead_states_at_time_zero() {
        let n = 3;

        let mut formula = Formula::new();
        let machine = Machine::new(n, LENGTH, &mut formula);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(n, LENGTH, &machine, &mut logic);

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

mod the_machine_changes_state_when_it_reads_input {
    use super::*;

    #[test]
    fn it_adds_clauses_that_transitions_the_machines_states_over_time() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH, &mut formula);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &mut logic);

        subject.the_machine_changes_state_when_it_reads_input();

        let time_0 = machine.at_time(0);
        let time_1 = machine.at_time(1);
        let time_2 = machine.at_time(2);

        // Time 0, Rank 0:
        assert_eq!(literals(time_0.state(&[1])), "-1");
        assert_eq!(literals(time_0.state(&[2])), "1");

        // Time 0, Rank 1:
        assert_eq!(literals(time_0.state(&[1, 2])), "2 -3");
        assert_eq!(literals(time_0.state(&[2, 1])), "-2 3");

        // Time 1, Rank 0:
        assert_eq!(literals(time_1.state(&[1])), "-4");
        assert_eq!(literals(time_1.state(&[2])), "4");

        // Time 1, Rank 1:
        assert_eq!(literals(time_1.state(&[1, 2])), "5 -6");
        assert_eq!(literals(time_1.state(&[2, 1])), "-5 6");

        // Time 2, Rank 0:
        assert_eq!(literals(time_2.state(&[1])), "-7");
        assert_eq!(literals(time_2.state(&[2])), "7");

        // Time 2, Rank 1:
        assert_eq!(literals(time_2.state(&[1, 2])), "8 -9");
        assert_eq!(literals(time_2.state(&[2, 1])), "-8 9");

        assert_eq!(dimacs(&formula), &[
            // State(t=0, n=2) ^ State(t=1, n=1) -> State(t=1, n=21)
            "-1 4 -5 0",
            "-1 4 6 0",

            // State(t=1, n=2) ^ State(t=2, n=1) -> State(t=2, n=21)
            "-4 7 -8 0",
            "-4 7 9 0",

            // State(t=0, n=1) ^ State(t=1, n=2) -> State(t=1, n=12)
            "1 -4 -6 0",
            "1 -4 5 0",

            // State(t=1, n=1) ^ State(t=2, n=2) -> State(t=2, n=12)
            "4 -7 -9 0",
            "4 -7 8 0"
        ]);
    }
}
