rust
#![feature(test)]

extern crate test;
use std::ops::Deref;
use test::Bencher;

#[bench]
fn bench_deref(b: &mut Bencher) {
    let vec: Vec<u32> = (0..1000).into_iter().collect();
    b.iter(|| {
        let mut sum: u32 = 0;
        for index in 0..vec.len() {
            let slice = vec.deref();
            sum = sum.wrapping_add(slice[index]);
        }
        sum
    });
}
