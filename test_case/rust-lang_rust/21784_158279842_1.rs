 rust
#[feature(test)]
extern crate test;

#[bench]
fn my_bench(b: &mut test::Bencher /* needed here */) {
}
