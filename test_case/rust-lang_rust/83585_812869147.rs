rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

type TElem = u8;
type Tpl = (TElem, TElem, TElem, TElem, TElem, TElem, TElem, TElem, );
struct Data {
    original: Vec<Tpl>,
    diff_random_field: Vec<Tpl>,
    diff_first_field: Vec<Tpl>,
    diff_last_field: Vec<Tpl>,
    diff_every_second: Vec<Tpl>,
}

fn generate_data(n: usize) -> Data {
    use rand::prelude::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    let mut rng = ChaCha8Rng::seed_from_u64(357u64);
    let mut data: Vec<TElem> = vec![0; n * 8];
    rng.fill(data.as_mut_slice());
    let original: Vec<Tpl> = data.windows(8).map(|x|{
        (x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7],)
    }).collect();

    let diff_random_field: Vec<Tpl> = original
        .iter()
        .map(|&x| unsafe {
            let mut arr: [TElem; 8] = std::mem::transmute(x);
            let index = rng.gen_range(0..arr.len());
            arr[index] = arr[index].wrapping_add(5);
            std::mem::transmute(arr)
        })
        .collect();
    let diff_first_field: Vec<Tpl> = original
        .iter()
        .copied()
        .map(|mut x| {
            x.0 = x.0.wrapping_add(5);
            x
        })
        .collect();
    let diff_last_field: Vec<Tpl> = original
        .iter()
        .copied()
        .map(|mut x| {
            x.7 = x.7.wrapping_add(5);
            x
        })
        .collect();

    let mut diff_every_second: Vec<Tpl> = original.clone();
    for (a, b) in diff_every_second
        .iter_mut()
        .zip(diff_random_field.iter())
        .step_by(2)
    {
        *a = b.clone();
    }

    Data {
        original,
        diff_random_field,
        diff_first_field,
        diff_last_field,
        diff_every_second,
    }
}

fn arr_cmp(a: Tpl, b: Tpl)->bool{
    unsafe{
        let a: [TElem; 8] = std::mem::transmute(a);
        let b: [TElem; 8] = std::mem::transmute(b);
        a==b
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let Data {
        original,
        diff_random_field,
        diff_first_field,
        diff_last_field,
        diff_every_second,
    } = generate_data(1000);
    let original_copy = original.clone();

    let mut group = c.benchmark_group("u8");

    let pairs = [
        ("Self", original_copy),
        ("Random field", diff_random_field),
        ("First field", diff_first_field),
        ("Last field", diff_last_field),
        ("Every Second", diff_every_second),
    ];
    for (name, to_compare) in pairs.iter().cloned() {
        group.bench_with_input(
            BenchmarkId::new("u8 tuple", name),
            &to_compare,
            |b, to_compare| {
                b.iter_batched(
                    || -> (Vec<bool>, Vec<Tpl>) {
                        let buffer = Vec::with_capacity(to_compare.len());
                        let to_compare_copy = to_compare.clone();
                        (buffer, to_compare_copy)
                    },
                    |(mut out_buff, other)| {
                        let comparison = original.iter().zip(other.iter()).map(|(a, b)| a == b);
                        out_buff.extend(comparison);
                        (out_buff, other)
                    },
                    criterion::BatchSize::LargeInput,
                );
            },
        );
    }

    for (name, to_compare) in pairs.iter().cloned() {
        group.bench_with_input(
            BenchmarkId::new("u8 arr", name),
            &to_compare,
            |b, to_compare| {
                b.iter_batched(
                    || -> (Vec<bool>, Vec<Tpl>) {
                        let buffer = Vec::with_capacity(to_compare.len());
                        let to_compare_copy = to_compare.clone();
                        (buffer, to_compare_copy)
                    },
                    |(mut out_buff, other)| {
                        let comparison = original.iter().zip(other.iter()).map(|(a, b)| arr_cmp(*a, *b));
                        out_buff.extend(comparison);
                        (out_buff, other)
                    },
                    criterion::BatchSize::LargeInput,
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
