
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_for_each(b: &mut Bencher) {
    fn last<I: Iterator>(i: I) -> Option<<I as Iterator>::Item> where I: Sized {
        let mut last = None;
        i.for_each(|x| last = Some(x));
        last
    }
    let v = vec![0usize; 1024];
    b.iter(|| { last(black_box(v.iter())) })
}

#[bench]
fn bench_fold(b: &mut Bencher) {
    fn last<I: Iterator>(i: I) -> Option<<I as Iterator>::Item> where I: Sized {
        i.fold(None, move |_, x| Some(x))
    }
    let v = vec![0usize; 1024];
    b.iter(|| { last(black_box(v.iter())) })
}
