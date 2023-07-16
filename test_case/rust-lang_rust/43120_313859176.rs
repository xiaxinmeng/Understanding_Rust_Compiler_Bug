Rust
#[bench]
fn bench_rnd(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| rng.gen_range::<f64>(2.0, 100.0));
}
