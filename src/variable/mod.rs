pub struct Variable {
    number: usize,
}

impl Variable {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

#[cfg(test)]
mod test;
