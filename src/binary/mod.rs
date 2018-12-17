use crate::variable::Variable;
use crate::literal::Literal;
use ::bitvec::Bits;

#[derive(Debug, Eq, PartialEq)]
pub struct Binary {
    bits: Vec<Literal>,
}

impl Binary {
    pub fn from_decimal(decimal: usize, variables: &[Variable]) -> Self {
        if decimal > Self::max_value(variables) {
            panic!("There aren't enough variables to represent {}", decimal);
        }

        let decimal = decimal as u64;

        let bits = variables
            .iter()
            .enumerate()
            .map(|(i, variable)| {
                let boolean = decimal.get(i as u8);
                Literal::new(*variable, boolean)
            })
            .collect();

        Self { bits }
    }

    fn max_value(variables: &[Variable]) -> usize {
        2_usize.pow(variables.len() as u32) - 1
    }
}

#[cfg(test)]
mod test;
