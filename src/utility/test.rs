use super::*;

type Subject = Utility;

mod factorial {
    use super::*;

    #[test]
    fn it_returns_the_factorial_of_n() {
        assert_eq!(Subject::factorial(0), 1);
        assert_eq!(Subject::factorial(1), 1);
        assert_eq!(Subject::factorial(2), 2);
        assert_eq!(Subject::factorial(3), 6);
        assert_eq!(Subject::factorial(4), 24);
    }
}
