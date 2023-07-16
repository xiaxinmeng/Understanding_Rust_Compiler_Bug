rust
//! Benchmark sample floats meant to invoke certain code paths.

use std::time::Duration;
use fast_float::parse;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! core_generator {
    ($group:ident, $name:literal, $str:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                black_box($str.parse::<$t>().unwrap());
            })
        });
    };
}

macro_rules! fast_float_generator {
    ($group:ident, $name:literal, $str:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                black_box(parse::<$t, _>($str).unwrap());
            })
        });
    };
}

// FLOATS

// NOTE: Rust currently doesn't handle large, denormal floats
// with more than 25 significant digits. Use the 25 significant
// digits for both large and denormal.

// Example fast-path value.
const FAST_PATH: &str = "1.2345e22";
// Example disguised fast-path value.
const DISGUISED_FAST_PATH: &str = "1.2345e30";
// Example moderate path value: clearly not halfway `1 << 53`.
const MODERATE_PATH: &str = "9007199254740992.0";
// Example exactly-halfway value `(1<<53) + 1`.
const HALFWAY: &str = "9007199254740993.0";
// Example large, near-halfway value.
const LARGE: &str = "8.988465674311580536566680e307";
// Example denormal, near-halfway value.
const DENORMAL: &str = "8.442911973260991817129021e-309";

fn core(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("core");
    group.measurement_time(Duration::from_secs(5));
    core_generator!(group, "fast", FAST_PATH, f64);
    core_generator!(group, "disguised", DISGUISED_FAST_PATH, f64);
    core_generator!(group, "moderate", MODERATE_PATH, f64);
    core_generator!(group, "halfway", HALFWAY, f64);
    core_generator!(group, "large", LARGE, f64);
    core_generator!(group, "denormal", DENORMAL, f64);
}

fn fast_float(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("fast-float");
    group.measurement_time(Duration::from_secs(5));
    fast_float_generator!(group, "fast", FAST_PATH, f64);
    fast_float_generator!(group, "disguised", DISGUISED_FAST_PATH, f64);
    fast_float_generator!(group, "moderate", MODERATE_PATH, f64);
    fast_float_generator!(group, "halfway", HALFWAY, f64);
    fast_float_generator!(group, "large", LARGE, f64);
    fast_float_generator!(group, "denormal", DENORMAL, f64);
}

// MAIN

criterion_group!(core_benches, core);
criterion_group!(fast_float_benches, fast_float);
criterion_main!(core_benches, fast_float_benches);
