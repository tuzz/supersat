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

mod display {
    use super::*;

    #[test]
    fn it_formats_the_variables_number() {
        let subject = Subject::new(123);
        let formatted = format!("{}", subject);

        assert_eq!(formatted, "123");
    }
}
