rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("fib 1", |b| b.iter(|| fibonacci(1)));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("fib 2", |b| b.iter(|| fibonacci(3)));
}
