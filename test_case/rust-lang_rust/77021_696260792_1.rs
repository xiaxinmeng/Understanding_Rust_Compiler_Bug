rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_is_ascii_whitespace(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_ascii_whitespace");
    group.bench_function("std", |b| {
        b.iter(|| {
            let mut n = 0;
            for i in 0..128u8 {
                if is_whitespace::std_is_ascii_whitespace(black_box(&(i as char))) {
                    n += 1;
                }
            }
            black_box(n);
        })
    });
    group.bench_function("pr", |b| {
        b.iter(|| {
            let mut n = 0;
            for i in 0..128u8 {
                if is_whitespace::pr_is_ascii_whitespace(black_box(&(i as char))) {
                    n += 1;
                }
            }
            black_box(n);
        })
    });
}

criterion_group!(benches, bench_is_ascii_whitespace);
criterion_main!(benches);
