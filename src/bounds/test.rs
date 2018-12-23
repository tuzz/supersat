use super::*;

type Subject = Bounds;

const N: usize = 3;
const LENGTH: usize = 9;

mod new {
    use super::*;

    #[test]
    fn it_builds_the_struct() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.n, N);
        assert_eq!(subject.length_of_string, LENGTH);
        assert_eq!(subject.max_permutations, &[3, 5]);
    }
}

mod allowed_waste {
    use super::*;

    #[test]
    fn it_returns_how_many_wasted_symbols_can_reach_the_goal_for_each_index() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.allowed_waste(2), 0..=1);
        assert_eq!(subject.allowed_waste(3), 0..=1);
        assert_eq!(subject.allowed_waste(4), 0..=1);
        assert_eq!(subject.allowed_waste(5), 1..=2);
        assert_eq!(subject.allowed_waste(6), 1..=2);
        assert_eq!(subject.allowed_waste(7), 1..=2);
        assert_eq!(subject.allowed_waste(8), 2..=2);
    }
}

mod max_perms_at_last_waste {
    use super::*;

    #[test]
    fn it_returns_the_maximum_number_of_permutations_at_the_last_wasted_symbol() {
        let subject = Subject::new(N, 3, &[3, 5]);

        assert_eq!(subject.max_perms_at_last_waste(1, 2), 2);
        assert_eq!(subject.max_perms_at_last_waste(1, 3), 3);
        assert_eq!(subject.max_perms_at_last_waste(1, 4), 3);

        assert_eq!(subject.max_perms_at_last_waste(2, 4), 4);
        assert_eq!(subject.max_perms_at_last_waste(2, 5), 5);
        assert_eq!(subject.max_perms_at_last_waste(2, 6), 5);
    }

    #[test]
    fn it_returns_zero_for_no_wasted_symbols() {
        let subject = Subject::new(N, 3, &[3, 5]);

        assert_eq!(subject.max_perms_at_last_waste(0, 1), 0);
        assert_eq!(subject.max_perms_at_last_waste(0, 2), 0);
    }
}

mod max_perms_added {
    use super::*;

    #[test]
    fn it_returns_the_maximum_permutations_for_the_number_of_wasted_symbols() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.max_perms_added(0, 0), 3);
        assert_eq!(subject.max_perms_added(1, 0), 5);
    }

    #[test]
    fn it_returns_n_factorial_if_the_max_permutation_isnt_known() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.max_perms_added(2, 0), 6);
    }

    #[test]
    fn it_returns_the_remainder_of_n_factorial_minus_the_current_permutations() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.max_perms_added(2, 1), 5);
        assert_eq!(subject.max_perms_added(2, 2), 4);
        assert_eq!(subject.max_perms_added(2, 3), 3);
    }

    #[test]
    fn it_returns_the_minimum_of_max_permutations_and_the_remainder() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.max_perms_added(1, 2), 4);
    }
}

mod goal_index {
    use super::*;

    #[test]
    fn it_returns_the_index_of_the_goal() {
        let subject = Subject::new(N, LENGTH, &[]);
        assert_eq!(subject.goal_index(), LENGTH - 1);
    }
}

mod goal_wasted_symbols {
    use super::*;

    #[test]
    fn it_returns_the_number_of_wasted_symbols_for_the_goal() {
        let subject = Subject::new(N, LENGTH, &[]);
        assert_eq!(subject.goal_wasted_symbols(), 0);

        let subject = Subject::new(N, LENGTH, &[3]);
        assert_eq!(subject.goal_wasted_symbols(), 1);

        let subject = Subject::new(N, LENGTH, &[3, 5]);
        assert_eq!(subject.goal_wasted_symbols(), 2);
    }
}

mod goal_permutations {
    use super::*;

    #[test]
    fn it_returns_the_number_of_permutations_for_the_goal() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);
        assert_eq!(subject.goal_permutations(), 5);
    }
}

mod number_of_permutations {
    use super::*;

    #[test]
    fn it_returns_the_number_of_permutations_for_the_given_index_and_wasted_symbols() {
        let subject = Subject::new(N, 3, &[]);

        assert_eq!(subject.number_of_permutations(2, 0), Some(1));
        assert_eq!(subject.number_of_permutations(2, 1), Some(0));
        assert_eq!(subject.number_of_permutations(2, 2), None);

        assert_eq!(subject.number_of_permutations(3, 0), Some(2));
        assert_eq!(subject.number_of_permutations(3, 1), Some(1));
        assert_eq!(subject.number_of_permutations(3, 2), Some(0));
        assert_eq!(subject.number_of_permutations(3, 3), None);

        assert_eq!(subject.number_of_permutations(4, 0), Some(3));
        assert_eq!(subject.number_of_permutations(4, 1), Some(2));
        assert_eq!(subject.number_of_permutations(4, 2), Some(1));
        assert_eq!(subject.number_of_permutations(4, 3), Some(0));
        assert_eq!(subject.number_of_permutations(4, 4), None);
    }
}

mod minimum_wasted_symbols {
    use super::*;

    #[test]
    fn it() {
        let subject = Subject::new(N, LENGTH, &[3, 5]);

        assert_eq!(subject.minimum_wasted_symbols(1), 0);
        assert_eq!(subject.minimum_wasted_symbols(2), 0);
        assert_eq!(subject.minimum_wasted_symbols(3), 0);

        assert_eq!(subject.minimum_wasted_symbols(4), 1);
        assert_eq!(subject.minimum_wasted_symbols(5), 1);

        assert_eq!(subject.minimum_wasted_symbols(6), 2);
        assert_eq!(subject.minimum_wasted_symbols(7), 2);
    }
}
