use super::*;

type Subject = Rank;
const N: usize = 4;

mod new {
    use super::*;

    #[test]
    fn it_builds_enough_variables_to_represent_all_the_state_indexes() {
        let mut formula = Formula::new();

        let subject = Subject::new(50, &mut formula);
        assert_eq!(subject.variables.len(), 6); // 2^6 >= 50 states

        let subject = Subject::new(128, &mut formula);
        assert_eq!(subject.variables.len(), 7); // 2^7 >= 128 states

        let subject = Subject::new(129, &mut formula);
        assert_eq!(subject.variables.len(), 8); // 2^8 >= 129 states

        let subject = Subject::new(5000, &mut formula);
        assert_eq!(subject.variables.len(), 13); // 2^13 >= 5000 states
    }

    #[test]
    fn it_builds_all_the_states_from_the_same_set_of_shared_variables() {
        let mut formula = Formula::new();

        let subject = Subject::new(50, &mut formula);
        let variables = subject.variables;

        assert_eq!(subject.states[0], State::new(0, &variables));
        assert_eq!(subject.states[1], State::new(1, &variables));
        assert_eq!(subject.states[2], State::new(2, &variables));
        assert_eq!(subject.states[3], State::new(3, &variables));
        assert_eq!(subject.states[4], State::new(4, &variables));
        assert_eq!(subject.states[5], State::new(5, &variables));
    }
}

mod state {
    use super::*;

    #[test]
    fn it_returns_a_reference_to_the_named_state() {
        let mut formula = Formula::new();
        let subject = Subject::new(50, &mut formula);

        let state = subject.state(&[3, 1], N);
        let expected = State::index(&[3, 1], N);

        assert_eq!(state, &subject.states[expected]);
    }
}

mod max_state {
    use super::*;

    #[test]
    fn it_returns_the_last_state_in_the_rank() {
        let mut formula = Formula::new();
        let subject = Subject::new(50, &mut formula);

        assert_eq!(subject.max_state(), &subject.states[49]);
    }
}
