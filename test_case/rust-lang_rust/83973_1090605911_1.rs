rust
#![feature(test)]

use criterion::*;
use rand::prelude::*;

include!("../src/fast_float.rs");

fn mod360_u32(vals: &[f64]) -> () {
    #[inline(always)]
    fn mod360(x: f64) -> f64 {
        let ans = ((x as u32) % 360) as f64 + x.fract();
        core::hint::black_box(ans);
        ans
    }

    for x in vals {
        mod360(*x);
    }
}

fn mod360_i32(vals: &[f64]) -> () {
    #[inline(always)]
    fn mod360(x: f64) -> f64 {
        // let ans = (x as i32).rem_euclid(360) as f64 + x.fract();
        let ans = x.rem_i32(360);
        core::hint::black_box(ans);
        ans
    }

    for x in vals {
        mod360(*x);
    }
}

fn mod360_i64(vals: &[f64]) -> () {
    #[inline(always)]
    fn mod360(x: f64) -> f64 {
        // let ans = (x as i32).rem_euclid(360) as f64 + x.fract();
        let ans = x.rem_i64(360);
        core::hint::black_box(ans);
        ans
    }

    for x in vals {
        mod360(*x);
    }
}

fn mod360_i128(vals: &[f64]) -> () {
    #[inline(always)]
    fn mod360(x: f64) -> f64 {
        // let ans = (x as i32).rem_euclid(360) as f64 + x.fract();
        let ans = x.rem_i128(360);
        core::hint::black_box(ans);
        ans
    }

    for x in vals {
        mod360(*x);
    }
}

fn mod360_f64(vals: &[f64]) -> () {
    fn mod360(x: f64) -> f64 {
        let ans = x % 360.0;
        core::hint::black_box(ans);
        ans
    }

    for x in vals {
        mod360(*x);
    }
}

fn gen_f64(rng: &mut ThreadRng) -> f64 {
    const MIN: i64 = u32::MIN as i64 * 1000;
    const MAX: i64 = u32::MAX as i64 * 1000;
    const RANGE: u64 = (MAX - MIN) as u64;

    let rand = rng.gen_range(0, RANGE);
    // Convert back into the expected range
    let rand = MIN + (rand as i64);
    // Convert from thousandths of a degree to float
    rand as f64 / 1000.0
}

const COUNT: usize = 2048;
fn bench_mod360(c: &mut Criterion) {
    let mut group = c.benchmark_group("mod360");
    let mut rng = rand::thread_rng();
    let mut inputs = Vec::with_capacity(COUNT);
    for _ in 0..COUNT {
        inputs.push(gen_f64(&mut rng));
    }

    group.bench_with_input(
        BenchmarkId::new("mod360 via u32", "2048 rand"),
        &inputs,
        |b, vals| {
            b.iter(|| mod360_u32(&vals));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("mod360 f64", "2048 rand"),
        &inputs,
        |b, vals| {
            b.iter(|| mod360_f64(&vals));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("mod360 via i32", "2048 rand"),
        &inputs,
        |b, vals| {
            b.iter(|| mod360_i32(&vals));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("mod360 via i64", "2048 rand"),
        &inputs,
        |b, vals| {
            b.iter(|| mod360_i64(&vals));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("mod360 via i128", "2048 rand"),
        &inputs,
        |b, vals| {
            b.iter(|| mod360_i128(&vals));
        },
    );
}

criterion_group!(benches, bench_mod360);
criterion_main!(benches);
