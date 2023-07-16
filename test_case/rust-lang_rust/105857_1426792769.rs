
pub fn fibonacci_vec(fib: &mut [u64]) {
    // The compiler now knows that `fib` is fixed-size
    // and we are iterating exactly up to its length
    for i in 2..fib.len() {
        fib[i] = fib[i-1] + fib[i-2]; // no more bounds checks!
    }
}
