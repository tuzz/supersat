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
        let subject = Subject::new(N, &bounds, &mut formula);

        assert_eq!(subject.n, N);
        assert_eq!(subject.registers.len(), 7);
    }
}

mod at_time {
    use super::*;

    #[test]
    fn it_returns_the_register_for_the_given_point_in_time() {
        let mut formula = Formula::new();
        let bounds = Bounds::new(N, LENGTH, &[3, 5]);
        let subject = Subject::new(N, &bounds, &mut formula);

        assert_eq!(subject.at_time(2), &subject.registers[0]);
        assert_eq!(subject.at_time(3), &subject.registers[1]);
        assert_eq!(subject.at_time(4), &subject.registers[2]);
    }
}
