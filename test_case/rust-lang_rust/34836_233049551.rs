
#![feature(test)]
#![allow(warnings)]
extern crate test;

use std::panic::catch_unwind;

#[bench]
fn test_panic(bh: &mut test::Bencher) {
    bh.iter(|| {
        catch_unwind(|| {
          println!("Hello World!");
        });
    });
}
