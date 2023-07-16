 Rust
#[bench]
fn bench_vec_repeat(b: &mut extra::test::BenchHarness) {
    do b.iter {
        let v: ~[u8] = ~[0u8, ..1024];
    }
}
