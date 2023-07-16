
#[bench]
fn my_bench(b: &mut Bencher) {
    b.mode = BenchMode::Single;
    b.iter(|| run());
}
