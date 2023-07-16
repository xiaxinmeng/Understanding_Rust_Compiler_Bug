 rust
#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn sum(b: &mut Bencher) {
    let numbers: Vec<i32> = (1..1000).collect();
    b.iter(|| {
        let out: i32 = test::black_box(&numbers).iter().sum();
        test::black_box(out);
    });
}

#[bench]
fn forloop(b: &mut Bencher) {
    let numbers: Vec<i32> = (1..1000).collect();
    b.iter(|| {
        let mut out = 0i32;
        for i in test::black_box(&numbers) {
            out += *i;
        }
        test::black_box(out);
    })
}
