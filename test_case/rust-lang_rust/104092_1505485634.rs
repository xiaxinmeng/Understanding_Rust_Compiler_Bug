rust
#![feature(test)]
extern crate test;

#[test]
fn foo() {}

#[bench]
fn short_name(b: &mut test::Bencher) {
    b.iter(|| 1);
}

#[bench]
fn this_is_a_really_long_name(b: &mut test::Bencher) {
    b.iter(|| 1);
}
