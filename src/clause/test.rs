use super::*;
use crate::variable::Variable;

type Subject = Clause;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_clause() {
        let subject = Subject::new();

        assert_eq!(subject.literals.len(), 0);
    }
}

mod display {
    use super::*;

    #[test]
    fn it_formats_the_clause_as_dimacs() {
        let mut subject = Clause::new();

        let a = Variable::new(123);
        let b = Variable::new(456);

        subject.add(Literal::positive(a));
        subject.add(Literal::negative(b));

        let formatted = format!("{}", subject);
        assert_eq!(formatted, "123 -456 0");
    }
}
