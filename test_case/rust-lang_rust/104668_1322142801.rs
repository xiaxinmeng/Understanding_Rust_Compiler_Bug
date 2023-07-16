rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn push(c: &mut Criterion) {
    c.bench_function("push", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in black_box(1..1 << 18) {
                v.push(i);
            }
            black_box(v);
        })
    });
}

criterion_group!(benches, push);
criterion_main!(benches);
