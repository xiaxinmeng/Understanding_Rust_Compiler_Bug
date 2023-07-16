
#[bench]
fn bench1(bh: & mut std::test::BenchHarness) {
    let mut x = 1;
    do bh.iter {
        x += 1;
        x ^= 2 * x;
    }
}
