 rust
#![feature(test)]
extern crate test;

#[bench]
fn foo(bh: &mut test::Bencher) {
    bh.iter(|| /* thing to benchmark */);
}
