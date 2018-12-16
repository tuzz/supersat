use super::*;

type Subject = Rank;
const N: usize = 4;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_increasing_number_of_states_based_on_the_index_of_the_rank() {
        let mut formula = Formula::new();
        let dead_state = 1;

        let subject = Subject::new(0, N, &mut formula);
        assert_eq!(subject.states.len(), 4);

        let subject = Subject::new(1, N, &mut formula);
        assert_eq!(subject.states.len(), 4 * 3 + dead_state);

        let subject = Subject::new(2, N, &mut formula);
        assert_eq!(subject.states.len(), 4 * 3 * 2 + dead_state);

        let subject = Subject::new(3, N, &mut formula);
        assert_eq!(subject.states.len(), 4 * 3 * 2 * 1 + dead_state);
    }

    #[test]
    fn it_builds_enough_variables_to_represent_all_the_state_indexes() {
        let mut formula = Formula::new();

        let subject = Subject::new(0, N, &mut formula);
        assert_eq!(subject.variables.len(), 2); // 2^2 >= 4 states

        let subject = Subject::new(1, N, &mut formula);
        assert_eq!(subject.variables.len(), 4); // 2^4 >= 13 states

        let subject = Subject::new(2, N, &mut formula);
        assert_eq!(subject.variables.len(), 5); // 2^5 >= 25 states

        let subject = Subject::new(3, N, &mut formula);
        assert_eq!(subject.variables.len(), 5); // 2^5 >= 25 states
    }

    #[test]
    fn it_builds_all_the_states_from_the_same_set_of_shared_variables() {
        let mut formula = Formula::new();

        let subject = Subject::new(0, N, &mut formula);
        let variables = subject.variables;

        assert_eq!(subject.states[0], State::new(0, &variables));
        assert_eq!(subject.states[1], State::new(1, &variables));
        assert_eq!(subject.states[2], State::new(2, &variables));
        assert_eq!(subject.states[3], State::new(3, &variables));
    }
}
