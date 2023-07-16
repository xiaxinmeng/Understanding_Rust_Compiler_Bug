rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

#[derive(Clone, Copy)]
pub struct Blueprint {
    pub fuel_tank_size: u32,
    pub payload: u32,
    pub wheel_diameter: u32,
    pub wheel_width: u32,
    pub storage: u32,
}

pub fn eq0(a: &Blueprint, b: &Blueprint) -> bool {
    (a.fuel_tank_size == b.fuel_tank_size)
        && (a.payload == b.payload)
        && (a.wheel_diameter == b.wheel_diameter)
        && (a.wheel_width == b.wheel_width)
        && (a.storage == b.storage)
}

pub fn eq2(a: &Blueprint, b: &Blueprint) -> bool {
    if a.fuel_tank_size != b.fuel_tank_size {
        return false;
    }
    if a.payload != b.payload {
        return false;
    }
    if a.wheel_diameter != b.wheel_diameter {
        return false;
    }
    if a.wheel_width != b.wheel_width {
        return false;
    }
    if a.storage != b.storage {
        return false;
    }
    true
}

struct Data {
    original: Vec<Blueprint>,
    diff_random_field: Vec<Blueprint>,
    diff_last_field: Vec<Blueprint>,
}

fn generate_data(n: usize) -> Data {
    use rand::prelude::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    let mut rng = ChaCha8Rng::seed_from_u64(357u64);
    let mut data = vec![0; n * 5];
    rng.fill(data.as_mut_slice());

    let original: Vec<Blueprint> = data
        .windows(5)
        .map(|w| Blueprint {
            fuel_tank_size: w[0],
            payload: w[1],
            wheel_diameter: w[2],
            wheel_width: w[3],
            storage: w[4],
        })
        .collect();
    let diff_random_field: Vec<Blueprint> = original
        .iter()
        .map(|&x| unsafe {
            let mut arr: [u32; 5] = std::mem::transmute(x);
            let index = rng.gen_range(0..5);
            arr[index] = arr[index].wrapping_add(5);
            std::mem::transmute(arr)
        })
        .collect();
    let diff_last_field: Vec<Blueprint> = original
        .iter()
        .copied()
        .map(|mut x| {
            x.storage += 5;
            x
        })
        .collect();
    Data {
        original,
        diff_random_field,
        diff_last_field,
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let Data {
        original,
        diff_random_field,
        diff_last_field,
    } = generate_data(1000);

    let mut group = c.benchmark_group("cmp");

    let original_copy = original.clone();

    group.bench_with_input(BenchmarkId::new("eq0: Self", 0), &original_copy, |b, d| {
        b.iter(|| -> Vec<bool> {
            original
                .iter()
                .zip(d.iter())
                .map(|(a, b)| eq0(a, b))
                .collect()
        })
    });
    group.bench_with_input(
        BenchmarkId::new("eq0: Random field", 0),
        &diff_random_field,
        |b, d| {
            b.iter(|| -> Vec<bool> {
                original
                    .iter()
                    .zip(d.iter())
                    .map(|(a, b)| eq0(a, b))
                    .collect()
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("eq0: Last field", 0),
        &diff_last_field,
        |b, d| {
            b.iter(|| -> Vec<bool> {
                original
                    .iter()
                    .zip(d.iter())
                    .map(|(a, b)| eq0(a, b))
                    .collect()
            })
        },
    );

    group.bench_with_input(BenchmarkId::new("eq2: Self", 0), &original_copy, |b, d| {
        b.iter(|| -> Vec<bool> {
            original
                .iter()
                .zip(d.iter())
                .map(|(a, b)| eq2(a, b))
                .collect()
        })
    });
    group.bench_with_input(
        BenchmarkId::new("eq2: Random field", 0),
        &diff_random_field,
        |b, d| {
            b.iter(|| -> Vec<bool> {
                original
                    .iter()
                    .zip(d.iter())
                    .map(|(a, b)| eq2(a, b))
                    .collect()
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("eq2: Last field", 0),
        &diff_last_field,
        |b, d| {
            b.iter(|| -> Vec<bool> {
                original
                    .iter()
                    .zip(d.iter())
                    .map(|(a, b)| eq2(a, b))
                    .collect()
            })
        },
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
