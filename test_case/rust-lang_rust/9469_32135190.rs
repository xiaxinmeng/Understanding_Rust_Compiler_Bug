 rust
extern mod extra;
use std::unstable::intrinsics::{abort, u32_mul_with_overflow};
use extra::test::BenchHarness;

#[inline(never)]
fn control(xs: &mut [u32]) {
    for x in xs.mut_iter() {
        *x *= 5;
    }
}

#[inline(never)]
fn check(xs: &mut [u32]) {
    for x in xs.mut_iter() {
        unsafe {
            let (y, o) = u32_mul_with_overflow(*x, 5);
            if o {
                abort()
            }
            *x = y;
        }
    }
}

#[inline(never)]
fn check_libstd(xs: &mut [u32]) {
    for x in xs.mut_iter() {
        *x = x.checked_mul(&5).unwrap();
    }
}

#[bench]
fn bench_control(b: &mut BenchHarness) {
    b.iter(|| {
        let mut xs = [0, ..1000];
        control(xs)
    });
}

#[bench]
fn bench_check(b: &mut BenchHarness) {
    b.iter(|| {
        let mut xs = [0, ..1000];
        check(xs)
    });
}

#[bench]
fn bench_check_libstd(b: &mut BenchHarness) {
    b.iter(|| {
        let mut xs = [0, ..1000];
        check_libstd(xs)
    });
}
