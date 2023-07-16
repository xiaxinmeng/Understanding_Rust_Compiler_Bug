rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
enum Foo {
    Zero = 0,
    One = 1,
    Two = 2,
}

struct Data {
    sorted: Vec<Foo>,
    unordered: Vec<Foo>,
}

fn generate_data(n: usize) -> Data {
    use rand::prelude::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    let mut rng = ChaCha8Rng::seed_from_u64(6464u64);
    let distribution = rand::distributions::Uniform::new(0, 3);
    let mut data = Vec::with_capacity(n);
    for _ in 0..n {
        let num: usize = rng.sample(distribution);
        data.push(num);
    }
    fn convert(num: usize) -> Foo {
        [Foo::Zero, Foo::One, Foo::Two][num]
    }
    let mut sorted = data.clone();
    sorted.sort_unstable();

    Data {
        sorted: sorted.into_iter().map(convert).collect(),
        unordered: data.into_iter().map(convert).collect(),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let Data { sorted, unordered } = generate_data(1000);

    let mut group = c.benchmark_group("cmp");

    let pairs = [("Sorted", sorted), ("Unordered", unordered)];
    for (name, data) in pairs.iter().cloned() {
        group.bench_with_input(BenchmarkId::new("comparisons", name), &data, |b, data| {
            b.iter_batched(
                || -> (Vec<bool>, Vec<Foo>) {
                    let buffer = Vec::with_capacity(data.len());
                    let data = data.clone();
                    (buffer, data)
                },
                |(mut out_buff, data)| {
                    let comparisons = data.windows(2).map(|x| {
                        assert_eq!(x.len(), 2);
                        x[0] <= x[1]
                    });
                    out_buff.extend(comparisons);
                    (out_buff, data)
                },
                criterion::BatchSize::LargeInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
