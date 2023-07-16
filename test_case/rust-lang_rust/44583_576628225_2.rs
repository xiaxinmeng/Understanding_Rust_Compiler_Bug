rust
#![feature(test)]
extern crate test;
extern crate itoa;

use test::Bencher;

#[bench]
fn stdlib(b: &mut Bencher) {
    b.iter(|| {
        let s = format!("{}", u128::max_value());
        std::hint::black_box(s);
    });
}

#[bench]
fn itoa(b: &mut Bencher) {

    b.iter(|| {
        let mut s = String::new();
        itoa::fmt(&mut s, u128::max_value()).unwrap();
        std::hint::black_box(s);
    });
}
