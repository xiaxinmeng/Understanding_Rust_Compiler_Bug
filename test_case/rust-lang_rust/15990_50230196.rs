
extern crate test;
use test::Bencher;

#[bench]
fn bench_skip(b: &mut Bencher) {
    let xs = [0u, 1, 2, 3, 5, 13, 15, 16, 17, 19, 20, 30];
    b.iter(|| xs.iter().skip(5).next());
}

#[bench]
fn bench_skip_zero(b: &mut Bencher) {
    let xs = [0u, 1, 2, 3, 5, 13, 15, 16, 17, 19, 20, 30];
    b.iter(|| xs.iter().skip(0).next());
}

#[bench]
fn bench_skip_reach_end(b: &mut Bencher) {
    let xs = [0u, 1, 2, 3, 5, 13, 15, 16, 17, 19, 20, 30];
    b.iter(|| xs.iter().skip(20).next());
}
