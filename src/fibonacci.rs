#[derive(Debug)]
pub struct Fibonacci {
    x: usize,
    y: usize,
}

impl Fibonacci {
    pub fn new(x: usize, y: usize) -> Fibonacci {
        Fibonacci { x: x, y: y }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let (a, b) = (self.x, self.y);
        self.x = b;
        self.y = a + b;
        Some(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_basic() {
        let first_10: Vec<usize> = Fibonacci::new(0, 1).take(10).collect();
        assert_eq!(first_10, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
        let fib_50: usize = Fibonacci::new(0, 1).nth(49).unwrap();
        assert_eq!(12586269025, fib_50);
    }
}
