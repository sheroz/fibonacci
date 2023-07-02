fn main() {
    for n in 0..=11 {
        println!("fibonacci({n}) = {}", fibonacci::fibonacci(n));
    }
}