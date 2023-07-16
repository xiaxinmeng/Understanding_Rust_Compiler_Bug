 rust
|| {
    let mut n = 20;
    black_box(&mut n); // doesn't change it, but LLVM can't tell.
    fib(n)
}
