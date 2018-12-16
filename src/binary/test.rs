use super::*;

type Subject = Binary;

mod from_decimal {
    use super::*;

    #[test]
    fn it_uses_the_variables_to_build_the_struct_according_to_the_decimals_binary_representation() {
        let bit0 = Variable::new(123);
        let bit1 = Variable::new(456);

        let variables = vec![bit0, bit1];

        let zero = Subject::from_decimal(0, &variables);
        let one = Subject::from_decimal(1, &variables);
        let two = Subject::from_decimal(2, &variables);
        let three = Subject::from_decimal(3, &variables);

        assert_eq!(zero.bits,  &[(bit0, false), (bit1, false)]);
        assert_eq!(one.bits,   &[(bit0, true),  (bit1, false)]);
        assert_eq!(two.bits,   &[(bit0, false), (bit1, true)]);
        assert_eq!(three.bits, &[(bit0, true),  (bit1, true)]);
    }

    #[test]
    #[should_panic(expected = "There aren't enough variables to represent 4")]
    fn it_panics_if_there_arent_enough_variables_to_represent_the_decimal() {
        let bit0 = Variable::new(123);
        let bit1 = Variable::new(456);

        let variables = vec![bit0, bit1];

        Subject::from_decimal(4, &variables);
    }
}
