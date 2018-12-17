use super::*;
use crate::variable::Variable;

fn positive(number: usize) -> Literal {
    Literal::positive(Variable::new(number))
}

fn negative(number: usize) -> Literal {
    Literal::negative(Variable::new(number))
}

fn dimacs(logic: &Logic) -> Vec<String> {
    let formatted = format!("{}", logic.formula);

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

        assert_eq!(dimacs(&logic), &["123 -456 789 0"]);
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

        assert_eq!(dimacs(&logic), &["-456 0", "123 0", "789 0"]);
    }
}
