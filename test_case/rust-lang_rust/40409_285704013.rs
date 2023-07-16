rust
#[bench]
fn bench_big(b: &mut Bencher) {
    b.iter(|| vec![0u8; 1024 * 1024]);
}
#[bench]
fn bench_medium(b: &mut Bencher) {
    b.iter(|| vec![0u8; 1024]);
}
#[bench]
fn bench_small(b: &mut Bencher) {
    b.iter(|| vec![0u8; 8]);
}
