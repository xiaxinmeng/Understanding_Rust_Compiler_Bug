rust
use criterion::{black_box, Criterion};

fn main() {
    let mut c = Criterion::default();
    c.bench_function("partialeq-array", |b| {
        b.iter(|| {
            let a: [i64; 12] = black_box([0; 12]);
            let b: [i64; 12] = black_box([0; 12]);
            for _ in 0..1_000_000 {
                black_box(a.eq(&b));
            }
        })
    });
    c.bench_function("partialeq-tuple", |b| {
        b.iter(|| {
            let a: (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) =
                black_box((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
            let b: (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) =
                black_box((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
            for _ in 0..1_000_000 {
                black_box(a.eq(&b));
            }
        })
    });
}
