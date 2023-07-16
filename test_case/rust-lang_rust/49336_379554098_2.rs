rust
fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("fib 1", |b| b.iter(|| 1));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("fib 2", |b| b.iter(|| 3));
}
