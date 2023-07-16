rust
#[bench]
fn repeat100(b: &mut test::Bencher) {
    b.iter(|| { S.repeat(100); });
}
