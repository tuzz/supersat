use super::*;

type Subject = Register;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_register_with_literals_each_count_in_the_range() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        assert_eq!(subject.range, 4..=6);
        assert_eq!(subject.literals.len(), 3);
    }

    #[test]
    fn it_does_not_build_a_literal_for_zero_waste() {
        let mut formula = Formula::new();
        let subject = Subject::new(0..=2, &mut formula);

        assert_eq!(subject.literals.len(), 2);
    }
}

mod literal_for_count {
    use super::*;

    #[test]
    fn it_returns_the_literal_for_the_given_count() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        let literal_for_4 = subject.literals[0];
        let literal_for_5 = subject.literals[1];
        let literal_for_6 = subject.literals[2];

        assert_eq!(subject.literal_for_count(4), Some(literal_for_4));
        assert_eq!(subject.literal_for_count(5), Some(literal_for_5));
        assert_eq!(subject.literal_for_count(6), Some(literal_for_6));
    }

    #[test]
    fn it_returns_none_if_there_is_no_literal_for_the_given_count() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        assert_eq!(subject.literal_for_count(3), None);
        assert_eq!(subject.literal_for_count(7), None);
    }

    #[test]
    fn it_offsets_the_index_to_account_for_no_zero_waste_literal() {
        let mut formula = Formula::new();
        let subject = Subject::new(0..=2, &mut formula);

        let literal_for_1 = subject.literals[0];
        let literal_for_2 = subject.literals[1];

        assert_eq!(subject.literal_for_count(0), None);
        assert_eq!(subject.literal_for_count(1), Some(literal_for_1));
        assert_eq!(subject.literal_for_count(2), Some(literal_for_2));
    }
}
