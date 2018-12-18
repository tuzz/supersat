pub struct Utility {

}

impl Utility {
    pub fn factorial(n: usize) -> usize {
        match n { 0 => 1, _ => n * Self::factorial(n - 1) }
    }
}

#[cfg(test)]
mod test;
