rust
fn fib(n: u64) -> u64 {
    if n < 3 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
