rust
use std::collections::HashSet;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn get_random_ints(amount: usize) -> Vec<usize> {
    let mut v = vec![0; amount];
    ChaCha20Rng::seed_from_u64(500).fill(v.as_mut_slice());
    v
}

pub fn bench_iterations(c: &mut Criterion) {
    let random_ints = get_random_ints(50);
    let make_hash_set = |capacity: usize| -> HashSet<usize> {
        let mut set = HashSet::with_capacity(capacity);
        set.extend(random_ints.iter());
        set
    };

    let mut group = c.benchmark_group("HashTableIteration");
    let caps = [64, 256, 1024, 4096, 16384];
    for capacity in caps {
        group.bench_with_input(
            BenchmarkId::new("HashTableIterationFull", capacity),
            &capacity,
            |b, &capacity| {
                b.iter_batched(
                    || make_hash_set(capacity),
                    |set| {
                        let sum: usize = set.iter().copied().sum();
                        (black_box(set), black_box(sum))
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }
    for capacity in caps {
        group.bench_with_input(
            BenchmarkId::new("HashTableIterationLimited", capacity),
            &capacity,
            |b, &capacity| {
                b.iter_batched(
                    || make_hash_set(capacity),
                    |set| {
                        let sum: usize = set.iter().copied().take(set.len()).sum();
                        (black_box(set), black_box(sum))
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }
}

criterion_group!(benches, bench_iterations);
criterion_main!(benches);

