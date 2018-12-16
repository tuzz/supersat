#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Variable {
    pub number: usize,
}

impl Variable {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

#[cfg(test)]
mod test;
