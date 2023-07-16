rust
#![feature(test)]

extern crate test;

use test::black_box;

fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("fib 1", |b| b.iter(|| fibonacci(black_box(3))));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("fib 2", |b| b.iter(|| fibonacci(black_box(3))));
}
