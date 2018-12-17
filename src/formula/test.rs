use super::*;

type Subject = Formula;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_formula() {
        let subject = Subject::new();

        assert_eq!(subject.variables.len(), 0);
        assert_eq!(subject.clauses.len(), 0);
    }
}

mod new_variable {
    use super::*;

    #[test]
    fn it_adds_a_new_variable_to_the_formula() {
        let mut subject = Subject::new();

        subject.new_variable();
        assert_eq!(subject.variables.len(), 1);
    }

    #[test]
    fn it_auto_increments_the_variables_number() {
        let mut subject = Subject::new();

        let variable = subject.new_variable();
        assert_eq!(variable.number, 1);

        let variable = subject.new_variable();
        assert_eq!(variable.number, 2);
    }
}

mod add_clause {
    use super::*;

    #[test]
    fn it_adds_an_existing_clause_to_the_formula() {
        let mut subject = Subject::new();
        let clause = Clause::new();

        subject.add_clause(clause);
        assert_eq!(subject.clauses.len(), 1);
    }

    #[test]
    fn it_can_add_clauses_containing_variables_from_the_formula() {
        let mut subject = Subject::new();

        let a = subject.new_variable();
        let b = subject.new_variable();

        let mut a_implies_b = Clause::new();
        a_implies_b.negative(a);
        a_implies_b.positive(b);

        subject.add_clause(a_implies_b);

        assert_eq!(subject.clauses.len(), 1);
    }

    #[test]
    fn it_de_duplicates() {
        let mut subject = Subject::new();

        let clause = Clause::new();
        subject.add_clause(clause);
        assert_eq!(subject.clauses.len(), 1);

        let clause = Clause::new();
        subject.add_clause(clause);
        assert_eq!(subject.clauses.len(), 1);
    }
}

mod display {
    use super::*;

    #[test]
    fn it_formats_the_formula_as_dimacs() {
        let mut subject = Subject::new();

        let a = subject.new_variable();
        let b = subject.new_variable();

        let mut a_implies_b = Clause::new();
        a_implies_b.negative(a);
        a_implies_b.positive(b);

        subject.add_clause(a_implies_b);

        let formatted = format!("{}", subject);
        assert_eq!(formatted, "p cnf 2 1\n-1 2 0\n");
    }
}
