 rust
#![allow(unused_must_use)]
#![feature(test)]

extern crate test;

use std::io::Write;
use std::vec::Vec;

use test::Bencher;

#[bench]
fn bench_write_value(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = Vec::new();
        for _ in 0..1000 {
            mem.write("abc".as_bytes());
        }
    });
}

#[bench]
fn bench_write_ref(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = Vec::new();
        let wr = &mut mem as &mut Write;
        for _ in 0..1000 {
            wr.write("abc".as_bytes());
        }
    });
}

#[bench]
fn bench_write_macro1(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = Vec::new();
        let wr = &mut mem as &mut Write;
        for _ in 0..1000 {
            write!(wr, "abc");
        }
    });
}

#[bench]
fn bench_write_macro2(bh: &mut Bencher) {
    bh.iter(|| {
        let mut mem = Vec::new();
        let wr = &mut mem as &mut Write;
        for _ in 0..1000 {
            write!(wr, "{}", "abc");
        }
    });
}
