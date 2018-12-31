use super::*;

type Subject = Counter;

const N: usize = 3;
const LENGTH: usize = 9;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_counter_with_a_register_for_each_time_except_the_first_n_minus_one() {
        let mut formula = Formula::new();
        let bounds = Bounds::new(N, LENGTH, &[3, 5]);
        let subject = Subject::new(&bounds, &mut formula);

        assert_eq!(subject.registers.len(), 7);
    }
}

mod register {
    use super::*;

    #[test]
    fn it_returns_the_register_by_index() {
        let mut formula = Formula::new();
        let bounds = Bounds::new(N, LENGTH, &[3, 5]);
        let subject = Subject::new(&bounds, &mut formula);

        assert_eq!(subject.register(0), &subject.registers[0]);
        assert_eq!(subject.register(1), &subject.registers[1]);
        assert_eq!(subject.register(2), &subject.registers[2]);
    }
}
