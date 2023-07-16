 rust
#[bench]
fn bench_deref(b: &mut Bencher) {
    let strings: Vec<_> = vec!["asfd"; 1000];
    b.iter(|| {
        for i in test::black_box(strings.iter()) {
            test::black_box(*test::black_box(i));
        }
    })
}
