use super::*;

type Subject = Goal;

const N: usize = 3;
const LENGTH: usize = 9;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_goal_with_a_rank_for_each_permutation() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, LENGTH, &mut formula);

        assert_eq!(subject.ranks.len(), 6);
    }
}

mod subgoal {
    use super::*;

    #[test]
    fn it_returns_a_reference_to_the_subgoal_for_a_permutation() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, LENGTH, &mut formula);

        assert_eq!(subject.subgoal(&[1, 2, 3]), &subject.ranks[0]);
        assert_eq!(subject.subgoal(&[1, 3, 2]), &subject.ranks[1]);
        assert_eq!(subject.subgoal(&[2, 1, 3]), &subject.ranks[2]);
        assert_eq!(subject.subgoal(&[2, 3, 1]), &subject.ranks[3]);
        assert_eq!(subject.subgoal(&[3, 1, 2]), &subject.ranks[4]);
        assert_eq!(subject.subgoal(&[3, 2, 1]), &subject.ranks[5]);
    }
}

mod invalid_ranges {
    use super::*;

    #[test]
    fn it_returns_an_invalid_range_per_rank() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, LENGTH, &mut formula);

        let invalid_ranges = subject.invalid_ranges();
        assert_eq!(invalid_ranges.len(), 6);
    }
}
