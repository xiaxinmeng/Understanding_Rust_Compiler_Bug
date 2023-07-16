rust
#![feature(test)]
#![feature(dec2flt)]

extern crate test;
extern crate core;

use test::{Bencher, black_box};
use core::num::dec2flt::rawfp::RawFloat;
use std::f32;

fn fmax(a: f32, b: f32) -> f32 {
    if b.is_nan() || b <= a { a } else { b }
}
fn fmin(a: f32, b: f32) -> f32 {
    if b.is_nan() || b >= a { a } else { b }
}

const NUMBERS: &[f32] = &[
    f32::INFINITY,
    f32::MAX,
    10.0,
    1.0,
    0.1,
    f32::MIN_POSITIVE,
    0.0,
    -0.0,
    -f32::MIN_POSITIVE,
    -0.1,
    -1.0
    -10.0,
    -f32::MAX,
    -f32::INFINITY,
    f32::NAN,
];

#[test]
fn test_equivalent() {
    for a in NUMBERS {
        for b in NUMBERS {
            let c = fmax(*a, *b);
            let d = a.max(*b);
            assert_eq!(c.integer_decode(), d.integer_decode(), "max({:?}, {:?}) => wrong:{:?}, correct:{:?}", a, b, c, d);

            let e = fmin(*a, *b);
            let f = a.min(*b);
            assert_eq!(e.integer_decode(), f.integer_decode(), "min({:?}, {:?}) => wrong:{:?}, correct:{:?}", a, b, e, f);
        }
    }
}

#[bench]
fn test_max_std(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a in NUMBERS {
            for b in NUMBERS {
                black_box(a.max(*b));
            }
        }
    })
}

#[bench]
fn test_max_new(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a in NUMBERS {
            for b in NUMBERS {
                black_box(fmax(*a, *b));
            }
        }
    })
}

#[bench]
fn test_min_std(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a in NUMBERS {
            for b in NUMBERS {
                black_box(a.min(*b));
            }
        }
    })
}

#[bench]
fn test_min_new(bencher: &mut Bencher) {
    bencher.iter(|| {
        for a in NUMBERS {
            for b in NUMBERS {
                black_box(fmin(*a, *b));
            }
        }
    })
}
