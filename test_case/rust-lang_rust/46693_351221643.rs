rust
#[bench]
fn bench_byte_find(b: &mut Bencher) {
    b.iter(|| {
        let s = test::black_box(DEMO_STRING);
        s.bytes().position(|b| b == b'\n')
    });
}
