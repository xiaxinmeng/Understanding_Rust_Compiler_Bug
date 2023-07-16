rust
#![feature(test)]

extern crate test;

pub fn branchless(a: i32, b: i32) -> i32 {
    let overflow_value = if (a ^ b).is_negative() {
        i32::min_value()
    } else {
        i32::max_value()
    };

    a.checked_mul(b).unwrap_or(overflow_value)
}

pub fn branch(a: i32, b: i32) -> i32 {
    a.checked_mul(b).unwrap_or_else(|| if (a ^ b).is_negative() {
        i32::min_value()
    } else {
        i32::max_value()
    })
}

use test::Bencher;

#[bench]
fn bench_branchless(b: &mut Bencher) {
    b.iter(|| (0..1000).fold(0, branchless));
}

#[bench]
fn bench_branch(b: &mut Bencher) {
    b.iter(|| (0..1000).fold(0, branch));
}
