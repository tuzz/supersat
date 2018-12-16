use super::*;

type Subject = State;
const N: usize = 3;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_state_with_binary_based_on_the_index() {
        let bit0 = Variable::new(123);
        let bit1 = Variable::new(456);
        let bit2 = Variable::new(789);

        let variables = vec![bit0, bit1, bit2];

        let subject = Subject::new(5, &variables);
        let expected = Binary::from_decimal(5, &variables);

        assert_eq!(subject.binary, expected);
    }
}

mod index {
    use super::*;

    #[test]
    fn it_returns_indexes_based_on_the_lexical_ordering_of_state_names() {
        assert_eq!(Subject::index(&[1], N), 0);
        assert_eq!(Subject::index(&[2], N), 1);
        assert_eq!(Subject::index(&[3], N), 2);
    }

    #[test]
    fn it_starts_the_index_from_one_for_longer_names_to_allow_for_dead_states() {
        assert_eq!(Subject::index(&[1, 2], N), 1);
        assert_eq!(Subject::index(&[1, 3], N), 2);
        assert_eq!(Subject::index(&[2, 1], N), 3);
        assert_eq!(Subject::index(&[2, 3], N), 4);
        assert_eq!(Subject::index(&[3, 1], N), 5);
        assert_eq!(Subject::index(&[3, 2], N), 6);

        assert_eq!(Subject::index(&[1, 2, 3], N), 1);
        assert_eq!(Subject::index(&[1, 3, 2], N), 2);
        assert_eq!(Subject::index(&[2, 1, 3], N), 3);
        assert_eq!(Subject::index(&[2, 3, 1], N), 4);
        assert_eq!(Subject::index(&[3, 1, 2], N), 5);
        assert_eq!(Subject::index(&[3, 2, 1], N), 6);

        let n = 4;

        assert_eq!(Subject::index(&[1, 2, 3, 4], n), 1);
        assert_eq!(Subject::index(&[1, 2, 4, 3], n), 2);
        assert_eq!(Subject::index(&[1, 3, 2, 4], n), 3);
        assert_eq!(Subject::index(&[1, 3, 4, 2], n), 4);
        assert_eq!(Subject::index(&[1, 4, 2, 3], n), 5);
        assert_eq!(Subject::index(&[1, 4, 3, 2], n), 6);
        assert_eq!(Subject::index(&[2, 1, 3, 4], n), 7);
        assert_eq!(Subject::index(&[2, 1, 4, 3], n), 8);
        // ...
        assert_eq!(Subject::index(&[4, 3, 1, 2], n), 23);
        assert_eq!(Subject::index(&[4, 3, 2, 1], n), 24);
    }

    #[test]
    fn it_returns_zero_for_dead_states() {
        assert_eq!(Subject::index(&[0, 0], N), 0);
        assert_eq!(Subject::index(&[0, 0, 0], N), 0);
    }
}
