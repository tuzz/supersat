use std::collections::HashSet;
use std::ops::Range;

use super::*;
use crate::variable::Variable;
use crate::state::State;
use crate::formula::Formula;
use crate::bounds::Bounds;

type Subject<'a> = Problem<'a>;

const N: usize = 2;
const LENGTH: usize = 3;

fn assert_dimacs(formula: &Formula, expected: &[&str]) {
    let expected = expected
        .iter()
        .cloned()
        .map(String::from)
        .collect::<HashSet<_>>();

    let actual = format!("{}", formula)
        .lines()
        .skip(1)
        .map(String::from)
        .collect::<HashSet<_>>();

    let missing_strings = actual
        .difference(&expected)
        .collect::<Vec<_>>();

    let additional_strings = expected
        .difference(&actual)
        .collect::<Vec<_>>();

    assert_eq!(missing_strings.len(), 0, "These strings are missing:\n{:?}", missing_strings);
    assert_eq!(additional_strings.len(), 0, "These strings are additional:\n{:?}", additional_strings);
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
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

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
        let goal = Goal::new(n, LENGTH, &mut formula);
        let bounds = Bounds::new(n, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(n, LENGTH, &machine, &goal, &bounds, &mut logic);

        subject.the_machine_starts_in_the_dead_states();

        let dead_state_1 = machine.at_time(0).state(&[0, 0]);
        let dead_state_2 = machine.at_time(0).state(&[0, 0, 0]);

        // Look up the literals for the dead states so we know what to assert.
        assert_eq!(literals(dead_state_1), "-3 -4 -5");
        assert_eq!(literals(dead_state_2), "-6 -7 -8");

        assert_dimacs(&formula, &[
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
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

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

        assert_dimacs(&formula, &[
            // State(t=1, n=12) -> State(t=0, n=1)
            "-1 -5 6 0",

            // State(t=1, n=12) -> State(t=1, n=2)
            "4 -5 6 0",

            // State(t=1, n=21) -> State(t=0, n=2)
            "1 5 -6 0",

            // State(t=1, n=21) -> State(t=1, n=1)
            "-4 5 -6 0",

            // State(t=2, n=12) -> State(t=1, n=1)
            "-4 -8 9 0",

            // State(t=2, n=12) -> State(t=2, n=2)
            "7 -8 9 0",

            // State(t=2, n=21) -> State(t=1, n=2)
            "4 8 -9 0",

            // State(t=2, n=21) -> State(t=2, n=1)
            "-7 8 -9 0",

            // Clauses for other implication direction:
            "-4 7 9 0",
            "-1 4 -5 0",
            "1 -4 -6 0",
            "4 -7 -9 0",
            "4 -7 8 0",
            "1 -4 5 0",
            "-4 7 -8 0",
            "-1 4 6 0",
        ]);
    }
}

mod the_string_starts_with_ascending_numbers {
    use super::*;

    #[test]
    fn it_adds_tautological_clauses_for_the_start_states() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH, &mut formula);
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

        subject.the_string_starts_with_ascending_numbers();

        let time_0_state_1 = machine.at_time(0).state(&[1]);
        let time_1_state_2 = machine.at_time(1).state(&[2]);

        // Look up the literals for the states so we know what to assert.
        assert_eq!(literals(time_0_state_1), "-1");
        assert_eq!(literals(time_1_state_2), "4");

        assert_dimacs(&formula, &["-1 0", "4 0"]);
    }
}

mod each_permutation_appears_at_most_once {
    use super::*;

    #[test]
    fn it_adds_clauses_forcing_the_goal_state_to_point_to_a_single_position_in_the_string() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH, &mut formula);
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

        subject.each_permutation_appears_at_most_once();

        // Machine states:
        let time_1_state_1 = machine.at_time(1).state(&[1, 2]);
        let time_1_state_2 = machine.at_time(1).state(&[2, 1]);

        let time_2_state_1 = machine.at_time(2).state(&[1, 2]);
        let time_2_state_2 = machine.at_time(2).state(&[2, 1]);

        assert_eq!(literals(time_1_state_1), "5 -6");
        assert_eq!(literals(time_1_state_2), "-5 6");

        assert_eq!(literals(time_2_state_1), "8 -9");
        assert_eq!(literals(time_2_state_2), "-8 9");

        // Goal states:
        let time_1_goal_1 = goal.subgoal(&[1, 2]).state_by_index(1);
        let time_1_goal_2 = goal.subgoal(&[2, 1]).state_by_index(1);

        let time_2_goal_1 = goal.subgoal(&[1, 2]).state_by_index(2);
        let time_2_goal_2 = goal.subgoal(&[2, 1]).state_by_index(2);

        assert_eq!(literals(time_1_goal_1), "10 -11");
        assert_eq!(literals(time_1_goal_2), "12 -13");

        assert_eq!(literals(time_2_goal_1), "-10 11");
        assert_eq!(literals(time_2_goal_2), "-12 13");

        assert_dimacs(&formula, &[
            // S(t=1, n=12) implies G(t=1, n=12)
            "-5 6 10 0",
            "-5 6 -11 0",

            // S(t=1, n=21) implies G(t=1, n=21)
            "5 -6 12 0",
            "5 -6 -13 0",

            // S(t=2, n=12) implies G(t=2, n=12)
            "-8 9 -10 0",
            "-8 9 11 0",

            // S(t=2, n=21) implies G(t=2, n=21)
            "8 -9 13 0",
            "8 -9 -12 0",
        ]);
    }
}

mod the_number_of_wasted_symbols_is_within_bounds {
    use super::*;

    #[test]
    fn it_limits_the_number_of_wasted_symbols_in_the_string() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH, &mut formula);
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

        subject.the_number_of_wasted_symbols_is_within_bounds();

        assert_eq!(bounds.wasted_symbol_ranges(),  &[0..=1, 1..=1]);

        // Look up the literals so we know what to assert.
        let time_1_dead_state = machine.at_time(1).state(&[0, 0]);
        let time_2_dead_state = machine.at_time(2).state(&[0, 0]);

        assert_eq!(literals(time_1_dead_state), "-5 -6"); // S(t=1, n=00)
        assert_eq!(literals(time_2_dead_state), "-8 -9"); // S(t=2, n=00)

        assert_dimacs(&formula, &[
            // (5 and 6) implies 14 which an alias for S(t=1, n=00)
            "5 6 14 0",

            // 14 implies (5 and 6)
            "-5 -14 0",
            "-6 -14 0",

            // (8 and 9) implies 15 which is an alias for S(t=2, n=00)
            "8 9 15 0",

            // 15 implies (8 and 9)
            "-8 -15 0",
            "-9 -15 0",

            // (1)  S(t=1, n=00) implies R(t=1, c=1)
            "-14 16 0",

            // (1)  S(t=2, n=00) implies R(t=2, c=1)
            "-15 17 0",

            // (3)  R(t=1, c=1) implies R(t=2, c=1)
            "-16 17 0",

            // (5)  S(t=2, n=00) implies -R(t=1, c=1)
            "-15 -16 0",
        ]);
    }
}

mod all_binary_representations_map_to_states {
    use super::*;

    #[test]
    fn it_adds_contradiction_clauses_for_invalid_binary_representations() {
        let mut formula = Formula::new();
        let machine = Machine::new(N, LENGTH, &mut formula);
        let goal = Goal::new(N, LENGTH, &mut formula);
        let bounds = Bounds::new(N, LENGTH, &[1]);
        let mut logic = Logic::new(&mut formula);
        let mut subject = Subject::new(N, LENGTH, &machine, &goal, &bounds, &mut logic);

        subject.all_binary_representations_map_to_states();

        let invalid = machine.invalid_ranges();
        assert_eq!(format_invalid_range(&invalid[0]), "2..2, variables: 1");
        assert_eq!(format_invalid_range(&invalid[1]), "3..4, variables: 2, 3");
        assert_eq!(format_invalid_range(&invalid[2]), "2..2, variables: 4");
        assert_eq!(format_invalid_range(&invalid[3]), "3..4, variables: 5, 6");
        assert_eq!(format_invalid_range(&invalid[4]), "2..2, variables: 7");
        assert_eq!(format_invalid_range(&invalid[5]), "3..4, variables: 8, 9");
        assert_eq!(invalid.len(), 6);

        assert_dimacs(&formula, &[
            // Clauses for machine:
            "-2 -3 0",
            "-5 -6 0",
            "-8 -9 0",

            // Clauses for goal:
            "-10 -11 0",
            "-12 -13 0",
        ]);
    }
}

fn format_invalid_range((range, variables): &(Range<usize>, &Vec<Variable>)) -> String {
    let variables = variables.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    format!("{:?}, variables: {}", range, variables)
}
