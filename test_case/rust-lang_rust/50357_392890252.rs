rust
#![feature(test)]
extern crate test;

use std::sync::{Arc, Weak};

#[bench]
fn arc_small_new_and_drop(b: &mut test::Bencher) {
    b.iter(|| {
        let a = Arc::new(41);
        test::black_box(&a);
    });
}

#[bench]
fn arc_big_new_and_drop(b: &mut test::Bencher) {
    b.iter(|| {
        let a = Arc::new([41; 1024 * 24]);
        test::black_box(&a);
    });
}

#[bench]
fn arc_clone(b: &mut test::Bencher) {
    let a = Arc::new(41);
    b.iter(move || {
        let aa = a.clone();
        test::black_box(&aa);
    });
}

#[bench]
fn weak_small_new_and_drop(b: &mut test::Bencher) {
    b.iter(|| {
        let w = Weak::<i32>::new();
        test::black_box(&w);
    });
}

#[bench]
fn weak_big_new_and_drop(b: &mut test::Bencher) {
    b.iter(|| {
        let a = Weak::<[i32; 1024 * 24]>::new();
        test::black_box(&a);
    });
}

#[bench]
fn weak_clone(b: &mut test::Bencher) {
    let w = Weak::<i32>::new();
    b.iter(move || {
        let ww = w.clone();
        test::black_box(&ww);
    });
}
