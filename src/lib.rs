
pub fn fibonacci(n: u32) -> u32 {
    fibonacci_iterative(n)
} 

pub fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

pub fn fibonacci_iterative(n: u32) -> u32 {
    
    if n <= 1 {
        return n;
    }

    let mut n1 = 0;
    let mut n2 = 1;

    let mut count = 1;    
    while count < n {
        let n3 = n1 + n2;
        n1 = n2;
        n2 = n3;
        count += 1; 
    }
    n2
}

#[cfg(test)]
mod tests {
    fn fibonacci_sequence_sample() -> Vec<u32> {
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
    }

    #[test]
    fn fibonacci_recursive_test() {
        let s = fibonacci_sequence_sample();
        let r: Vec<_> = (0..s.len()).map(|n| super::fibonacci_recursive(n as u32)).collect();
        assert_eq!(r, s);
    }

    #[test]
    fn fibonacci_iterative_test() {
        let s = fibonacci_sequence_sample();
        let r: Vec<_> = (0..s.len()).map(|n| super::fibonacci_iterative(n as u32)).collect();
        assert_eq!(r, s);
    }
}
