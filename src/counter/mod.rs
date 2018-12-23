use crate::formula::Formula;
use crate::bounds::Bounds;
use crate::register::Register;

struct Counter {
    n: usize,
    registers: Vec<Register>,
}

impl Counter {
    fn new(n: usize, bounds: &Bounds, formula: &mut Formula) -> Self {
        let registers = bounds.wasted_symbol_ranges()
            .iter()
            .map(|range| Register::new(range.clone(), formula))
            .collect();

        Self { n, registers }
    }

    fn at_time(&self, point_in_time: usize) -> &Register {
        &self.registers[point_in_time - (self.n - 1)]
    }
}

#[cfg(test)]
mod test;
