rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;

const NUM_ITEMS: usize = 20000;

fn generate_unique_nums() -> Vec<usize> {
    (0..NUM_ITEMS).collect()
}

fn generate_unique_strings() -> Vec<String> {
    (0..NUM_ITEMS).map(|x| x.to_string()).collect()
}

fn generate_duplicates<T: Clone>(v: Vec<T>) -> Vec<T> {
    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(546);
    v.into_iter()
        .flat_map(|s| {
            if rng.gen_bool(0.7) {
                vec![s]
            } else {
                vec![s.clone(), s]
            }
        })
        .take(NUM_ITEMS)
        .collect()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dedup");

    group.bench_function("Unique num", |b| {
        b.iter_batched(
            || black_box(generate_unique_nums()),
            |mut v| {
                v.dedup();
                v
            },
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("Duplicate num", |b| {
        b.iter_batched(
            || {
                let uniq = generate_unique_nums();
                let dup = generate_duplicates(uniq);
                black_box(dup)
            },
            |mut v| {
                v.dedup();
                v
            },
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("Unique string", |b| {
        b.iter_batched(
            || black_box(generate_unique_strings()),
            |mut v| {
                v.dedup();
                v
            },
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("With duplicate string", |b| {
        b.iter_batched(
            || {
                let uniq = generate_unique_strings();
                let dup = generate_duplicates(uniq);
                black_box(dup)
            },
            |mut v| {
                v.dedup();
                v
            },
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("Duplicate ZSTs", |b| {
        b.iter_batched(
            || black_box(vec![(); NUM_ITEMS]),
            |mut v| {
                v.dedup();
                v
            },
            criterion::BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
