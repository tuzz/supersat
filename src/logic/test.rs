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

mod at_least_one {
    use super::*;

    #[test]
    fn it_adds_a_clause_containing_a_disjunction_of_the_literals() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        logic.at_least_one(&[
            positive(123),
            negative(456),
            positive(789),
        ]);

        assert_eq!(dimacs(&formula), &["123 -456 789 0"]);
    }
}

mod all_of_them {
    use super::*;

    #[test]
    fn it_adds_a_clause_for_each_literal_to_be_conjuncted() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        logic.all_of_them(&[
            positive(123),
            negative(456),
            positive(789),
        ]);

        assert_eq!(dimacs(&formula), &["-456 0", "123 0", "789 0"]);
    }
}

mod if_then {
    use super::*;

    #[test]
    fn it_adds_clauses_that_are_logically_equivalent_to_if_then() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        let condition = &[positive(111), negative(222)];
        let consequent = &[negative(333), positive(444)];

        logic.if_then(condition, consequent);

        // (if a then b) is equivalent to
        // (a implies b) which is equivalent to
        // (not a or b)

        assert_eq!(dimacs(&formula), &["-111 222 -333 0", "-111 222 444 0"]);
    }
}

mod if_all_then {
    use super::*;

    #[test]
    fn it_adds_clauses_that_causes_the_consequent_to_be_true_if_all_conditions_are_met() {
        let mut formula = Formula::new();
        let mut logic = Logic::new(&mut formula);

        let first_condition = &[positive(111), negative(222)];
        let second_condition = &[negative(333), positive(444)];
        let third_condition = &[positive(555), negative(666)];

        let conditions: &[&[Literal]] = &[
            first_condition, second_condition, third_condition
        ];

        let consequent = &[negative(777), positive(888)];

        logic.if_all_then(conditions, consequent);

        assert_eq!(dimacs(&formula), &[
            "-111 222 333 -444 -555 666 -777 0",
            "-111 222 333 -444 -555 666 888 0",
        ]);
    }
}

mod negate {
    use super::*;

    #[test]
    fn it_negates_the_literals() {
        let literals = vec![positive(123), negative(456)];
        let negated = Logic::negate(&literals);

        let expected = vec![negative(123), positive(456)];

        assert_eq!(negated, expected);
    }
}
