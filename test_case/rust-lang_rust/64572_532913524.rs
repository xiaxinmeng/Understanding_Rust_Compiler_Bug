rust
// bench.rs
// - Compile with `rustc --test -O bench.rs`
// - Run with `./bench --bench`
#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn long_all(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100000);
        let k = test::black_box(1000000);
        (0..2*n).all(|x| x < k)
    })
}

#[bench]
fn long_all_chained(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100000);
        let k = test::black_box(1000000);
        (0..n).chain(0..n).all(|x| x < k)
    })
}
