use super::*;

type Subject = Register;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_register_with_a_variables_each_count_in_the_range() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        assert_eq!(subject.range, 4..=6);
        assert_eq!(subject.variables.len(), 3);
    }
}

mod variable_for_count {
    use super::*;

    #[test]
    fn it_returns_the_variable_for_the_given_count() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        let variable_for_4 = subject.variables[0];
        let variable_for_5 = subject.variables[1];
        let variable_for_6 = subject.variables[2];

        assert_eq!(subject.variable_for_count(4), Some(&variable_for_4));
        assert_eq!(subject.variable_for_count(5), Some(&variable_for_5));
        assert_eq!(subject.variable_for_count(6), Some(&variable_for_6));
    }

    #[test]
    fn it_returns_none_if_there_is_no_variable_for_the_given_count() {
        let mut formula = Formula::new();
        let subject = Subject::new(4..=6, &mut formula);

        assert_eq!(subject.variable_for_count(3), None);
        assert_eq!(subject.variable_for_count(7), None);
    }
}
