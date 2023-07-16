rust
use criterion::{criterion_group, criterion_main, Criterion};

pub fn new_string1() -> String {
    String::with_capacity(10)
}

pub fn new_string2() -> String {
    let mut string = String::new();
    string.reserve_exact(10);
    string
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_string1", |b| b.iter(|| new_string1()));
    c.bench_function("new_string2", |b| b.iter(|| new_string2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
