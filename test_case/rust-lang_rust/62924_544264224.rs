
fn bench_first_and_last(b: &mut Bencher, size: i32) {
    let map: BTreeMap<_, _> = (0..size).map(|i| (i, i)).collect();
    b.iter(|| {
        for _ in 0..10 {
            black_box(map.first_key_value());
            black_box(map.last_key_value());
        }
    });
}

#[bench]
pub fn first_and_last_0(b: &mut Bencher) {
    bench_first_and_last(b, 0);
}

#[bench]
pub fn first_and_last_100(b: &mut Bencher) {
    bench_first_and_last(b, 100);
}

#[bench]
pub fn first_and_last_10k(b: &mut Bencher) {
    bench_first_and_last(b, 10_000);
}
