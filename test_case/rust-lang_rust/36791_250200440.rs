 rust
#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_cloned(b: &mut Bencher) {
    let strings: Vec<_> = vec!["asfd"; 1000];
    b.iter(|| {
        for i in test::black_box(strings.iter()).cloned() {
            test::black_box(i);
        }
    })
}


#[bench]
fn bench_map(b: &mut Bencher) {
    let strings: Vec<_> = vec!["asfd"; 1000];
    b.iter(|| {
        for i in test::black_box(strings.iter()).map(|&x|x) {
            test::black_box(i);
        }
    })
}

#[bench]
fn bench_pat(b: &mut Bencher) {
    let strings: Vec<_> = vec!["asfd"; 1000];
    b.iter(|| {
        for &i in test::black_box(strings.iter()) {
            test::black_box(i);
        }
    })
}
