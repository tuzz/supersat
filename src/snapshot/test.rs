use super::*;

type Subject = Snapshot;
const N: usize = 3;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_snapshot_with_n_ranks() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, &mut formula);

        assert_eq!(subject.ranks.len(), N);
    }
}

mod state {
    use super::*;

    #[test]
    fn it_returns_a_reference_to_the_named_state() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, &mut formula);

        let state = subject.state(&[3, 1]);

        let expected_rank = &subject.ranks[1];
        let expected_state = expected_rank.state(&[3, 1], N);

        assert_eq!(state, expected_state);
    }
}

mod invalid_ranges {
    use super::*;

    #[test]
    fn it_returns_an_invalid_range_per_rank() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, &mut formula);

        let invalid_ranges = subject.invalid_ranges();
        assert_eq!(invalid_ranges.len(), 3);
    }
}

mod number_of_states {
    use super::*;

    #[test]
    fn it_returns_an_increasing_number_of_states_based_on_the_index() {
        let dead_state = 1;
        assert_eq!(Subject::number_of_states(0, N), 3);
        assert_eq!(Subject::number_of_states(1, N), 3 * 2 + dead_state);
        assert_eq!(Subject::number_of_states(2, N), 3 * 2 * 1 + dead_state);

        let n = 6;

        assert_eq!(Subject::number_of_states(0, n), 6);
        assert_eq!(Subject::number_of_states(1, n), 6 * 5 + dead_state);
        assert_eq!(Subject::number_of_states(2, n), 6 * 5 * 4 + dead_state);
        assert_eq!(Subject::number_of_states(3, n), 6 * 5 * 4 * 3 + dead_state);
    }
}
