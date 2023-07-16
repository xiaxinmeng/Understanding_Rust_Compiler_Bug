 rust
extern crate test;

#[bench]
fn x(b: &mut test::Bencher) {
    let mut v = Vec::with_capacity(100);
    b.iter(|| { v.extend(0..100); v.truncate(0); });
}
