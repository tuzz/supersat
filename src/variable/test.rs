use super::*;

type Subject = Variable;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_variable() {
        let subject = Subject::new(123);

        assert_eq!(subject.number, 123);
    }
}
