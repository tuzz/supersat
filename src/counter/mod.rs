use crate::formula::Formula;
use crate::bounds::Bounds;
use crate::register::Register;

pub struct Counter {
    registers: Vec<Register>,
}

impl Counter {
    pub fn new(bounds: &Bounds, formula: &mut Formula) -> Self {
        let registers = bounds.wasted_symbol_ranges()
            .iter()
            .map(|range| Register::new(range.clone(), formula))
            .collect();

        Self { registers }
    }

    pub fn register(&self, index: usize) -> &Register {
        &self.registers[index]
    }
}

#[cfg(test)]
mod test;
