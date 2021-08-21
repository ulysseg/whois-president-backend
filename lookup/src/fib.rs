pub struct FibonacciSequence {
    current: u64,
    previous: u64,
}

impl FibonacciSequence {

    pub fn new() -> FibonacciSequence {
        FibonacciSequence {
            current: 1,
            previous: 1,
        }
    }

    pub fn next(&mut self) -> u64 {
        let new_previous = self.current;
        self.current += self.previous;
        self.previous = new_previous;
        self.current
    }

    pub fn previous(&mut self) -> u64 {
        if self.current > 1 {
            let new_previous = self.current - self.previous;
            self.current = self.previous;
            self.previous = new_previous;
        }
        self.current
    }

    pub fn current(&self) -> u64 {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use crate::FibonacciSequence;

    #[test]
    fn fib() {
        let mut fib = FibonacciSequence::new();
        assert_eq!(fib.current(), 1);
        assert_eq!(fib.next(), 2);
        assert_eq!(fib.next(), 3);
        assert_eq!(fib.next(), 5);
        assert_eq!(fib.previous(), 3);
        assert_eq!(fib.previous(), 2);
        assert_eq!(fib.previous(), 1);
        assert_eq!(fib.previous(), 1);
        assert_eq!(fib.previous(), 1);
        assert_eq!(fib.next(), 2);
    }
}