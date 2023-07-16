 rust
#![allow(unstable)]
extern crate test;

use std::iter::repeat;
use std::slice::bytes::MutableByteVector;

#[bench]
fn normal(b: &mut test::Bencher) {
    b.iter(|| {
        repeat(0u8).take(64 * 1024).collect::<Vec<_>>()
    })
}

#[bench]
fn fast(b: &mut test::Bencher) {
    b.iter(|| {
        let mut v = Vec::with_capacity(64 * 1024);
        unsafe {
            v.set_len(64 * 1024);
        }
        v.set_memory(0);
        v
    })
}
