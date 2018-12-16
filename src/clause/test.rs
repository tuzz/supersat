use super::*;

type Subject = Clause;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_clause() {
        let subject = Subject::new();

        assert_eq!(subject.positive_literals.len(), 0);
        assert_eq!(subject.negative_literals.len(), 0);
    }
}

mod positive {
    use super::*;

    #[test]
    fn it_adds_the_variable_as_a_positive_literal() {
        let mut subject = Subject::new();
        let variable = Variable::new(123);

        subject.positive(&variable);

        assert_eq!(subject.positive_literals.contains(&variable), true);
    }

    #[test]
    fn it_de_duplicates() {
        let mut subject = Subject::new();
        let variable = Variable::new(123);

        subject.positive(&variable);
        subject.positive(&variable);
        subject.positive(&variable);

        assert_eq!(subject.positive_literals.len(), 1);
    }
}

mod negative {
    use super::*;

    #[test]
    fn it_adds_the_variable_as_a_negative_literal() {
        let mut subject = Subject::new();
        let variable = Variable::new(123);

        subject.negative(&variable);

        assert_eq!(subject.negative_literals.contains(&variable), true);
    }

    #[test]
    fn it_de_duplicates() {
        let mut subject = Subject::new();
        let variable = Variable::new(123);

        subject.negative(&variable);
        subject.negative(&variable);
        subject.negative(&variable);

        assert_eq!(subject.negative_literals.len(), 1);
    }
}
