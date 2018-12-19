use super::*;
use crate::variable::Variable;

fn positive(number: usize) -> Literal {
    Literal::positive(Variable::new(number))
}

fn negative(number: usize) -> Literal {
    Literal::negative(Variable::new(number))
}

fn dimacs(formula: &Formula) -> Vec<String> {
    let formatted = format!("{}", formula);

    let mut strings = formatted.lines().skip(1)
        .map(String::from).collect::<Vec<_>>();

    strings.sort();

    strings
}

mod new {
    use super::*;

    #[test]
    fn it_builds_the_struct_with_a_mutable_reference_to_the_formula() {
        let mut formula = Formula::new();
        let logic = Logic::new(&mut formula);

        let formatted = format!("{}", logic.formula);
        assert_eq!(formatted, "p cnf 0 0\n");
    }
}

mod tautology {
    use super::*;

    #[test]
    fn it_adds_a_clause_for_each_literal() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        logic.tautology(&[
            positive(123),
            negative(456),
            positive(789),
        ]);

        assert_eq!(dimacs(&formula), &["-456 0", "123 0", "789 0"]);
    }
}

mod contradiction {
    use super::*;

    #[test]
    fn it_adds_a_clause_for_each_negated_literal() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        logic.contradiction(&[
            positive(123),
            negative(456),
            positive(789),
        ]);

        assert_eq!(dimacs(&formula), &["-123 0", "-789 0", "456 0"]);
    }
}

mod implies {
    use super::*;

    #[test]
    fn it_adds_clauses_that_the_condition_implies_the_consequent() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        let condition = &[positive(111), negative(222)];
        let consequent = &[negative(333), positive(444)];

        logic.implies(condition, consequent);

        // (if a then b) is equivalent to
        // (a implies b) which is equivalent to
        // (not a or b)

        assert_eq!(dimacs(&formula), &["-111 222 -333 0", "-111 222 444 0"]);
    }
}

mod and {
    use super::*;

    #[test]
    fn it_combines_the_literals() {
        let a = vec![positive(111), negative(222)];
        let b = vec![negative(333)];

        let expected = vec![positive(111), negative(222), negative(333)];

        assert_eq!(Logic::and(&a, &b), expected);
    }
}

mod negate {
    use super::*;

    #[test]
    fn it_negates_the_literals() {
        let literals = vec![positive(123), negative(456)];
        let expected = vec![negative(123), positive(456)];

        assert_eq!(Logic::negate(&literals), expected);
    }
}
