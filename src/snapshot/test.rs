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

mod max_states {
    use super::*;

    #[test]
    fn it_returns_the_max_state_from_each_rank() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, &mut formula);

        let states = subject.max_states();

        let a = subject.ranks[0].max_state();
        let b = subject.ranks[1].max_state();
        let c = subject.ranks[2].max_state();

        assert_eq!(states, vec![a, b, c]);
    }
}
