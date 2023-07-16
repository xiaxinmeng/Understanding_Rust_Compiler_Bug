rust
#![feature(test)]

extern crate test;

use test::Bencher;

#[derive(Clone)]
struct Clonable<T>(T);

#[bench]
fn bench_data(b: &mut Bencher) {
    b.iter(|| vec![0u8; 1024 * 1024]);
}

#[bench]
fn bench_clone(b: &mut Bencher) {
    b.iter(|| vec![Clonable(0u8); 1024 * 1024]);
}

#[bench]
fn bench_copy(b: &mut Bencher) {
    b.iter(|| vec![(0u8,); 1024 * 1024]);
}

#[bench]
fn bench_nullptr(b: &mut Bencher) {
    b.iter(|| vec![std::ptr::null::<u8>(); 1024 * 1024]);
}

#[bench]
fn bench_none_ref(b: &mut Bencher) {
    b.iter(|| vec![None::<&u8>; 1024 * 1024]);
}

#[bench]
fn bench_padded(b: &mut Bencher) {
    b.iter(|| vec![(0u8,0u16); 1024 * 1024]);
}
