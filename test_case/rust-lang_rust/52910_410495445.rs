rust
#[bench]
fn bench_filter_third_collect(b: &mut Bencher) {
    b.iter(|| {
        let it = (black_box(0)..black_box(100)).filter(|i| i%3 == 0);
        it.collect::<Vec<u32>>()
    });
}

#[bench]
fn bench_filter_third_extend(b: &mut Bencher) {
    b.iter(|| {
        let it = (black_box(0)..black_box(100)).filter(|i| i%3 == 0);
        let mut v: Vec<u32> = Vec::new();
        v.extend(it);
    });
}
