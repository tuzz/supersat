use super::*;

type Subject = Literal;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_literal() {
        let variable = Variable::new(123);
        let subject = Subject::new(variable, true);

        assert_eq!(subject.variable, variable);
        assert_eq!(subject.positive, true);
    }
}

mod positive {
    use super::*;

    #[test]
    fn it_builds_a_positive_literal() {
        let variable = Variable::new(123);
        let subject = Subject::positive(variable);

        assert_eq!(subject.variable, variable);
        assert_eq!(subject.positive, true);
    }
}

mod negative {
    use super::*;

    #[test]
    fn it_builds_a_negative_literal() {
        let variable = Variable::new(123);
        let subject = Subject::negative(variable);

        assert_eq!(subject.variable, variable);
        assert_eq!(subject.positive, false);
    }
}
