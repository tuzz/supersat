use super::*;

type Subject = Machine;
const N: usize = 3;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_machine_with_a_snapshot_for_each_symbol_in_the_string() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, 9, &mut formula);

        assert_eq!(subject.snapshots.len(), 9);
    }
}

mod at_time {
    use super::*;

    #[test]
    fn it_returns_a_reference_to_the_snapshot_for_a_point_in_time() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, 9, &mut formula);

        assert_eq!(subject.at_time(3), &subject.snapshots[3]);
    }
}

mod max_states {
    use super::*;

    #[test]
    fn it_returns_the_max_states_from_each_snapshot() {
        let mut formula = Formula::new();
        let subject = Subject::new(N, 2, &mut formula);

        let states = subject.max_states();

        let mut a = subject.snapshots[0].max_states();
        let mut b = subject.snapshots[1].max_states();

        a.append(&mut b);

        assert_eq!(states, a);
    }
}
