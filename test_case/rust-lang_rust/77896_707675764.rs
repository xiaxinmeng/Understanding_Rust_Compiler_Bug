rust
#![feature(test)]
#![allow(soft_unstable)]

extern crate test;

use std::rc::Rc;

pub fn boxed_rc_0(v: Vec<u32>) -> Rc<[u32]> {
    Rc::from(v)
}

pub fn boxed_rc_1(v: Vec<u32>) -> Rc<[u32]> {
    Rc::from(v.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_test_small_0(b: &mut Bencher) {
        b.iter(|| {
            let v = vec![0; 32];
            let v = boxed_rc_0(test::black_box(v));
            test::black_box(v);
        });
    }

    #[bench]
    fn bench_test_small_1(b: &mut Bencher) {
        b.iter(|| {
            let v = vec![0; 32];
            let v = boxed_rc_1(test::black_box(v));
            test::black_box(v);
        });
    }

    #[bench]
    fn bench_test_big_0(b: &mut Bencher) {
        b.iter(|| {
            let v = vec![0; 4 * 1024 * 1024];
            let v = boxed_rc_0(test::black_box(v));
            test::black_box(v);
        });
    }

    #[bench]
    fn bench_test_big_1(b: &mut Bencher) {
        b.iter(|| {
            let v = vec![0; 4 * 1024 * 1024];
            let v = boxed_rc_1(test::black_box(v));
            test::black_box(v);
        });
    }
}
