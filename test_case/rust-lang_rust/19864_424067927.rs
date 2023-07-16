rust
#![feature(test)]
extern crate test;

use std::iter::repeat;

const N: usize = 100000;

#[derive(Clone)]
struct Foo;
#[derive(Clone)]
struct Bar {
    name: &'static str,
    desc: Option<String>,
    other: Option<String>,
}

#[bench]
fn b1(b: &mut test::Bencher) {
    b.iter(|| {
        let r: Result<u8, Foo> = Ok(1u8);
        repeat(r).take(N).map(|x| test::black_box(x)).count()
    });
}

#[bench]
fn b2(b: &mut test::Bencher) {
    b.iter(|| {
        let r: Result<u8, Bar> = Ok(1u8);
        repeat(r).take(N).map(|x| test::black_box(x)).count()
    });
}
