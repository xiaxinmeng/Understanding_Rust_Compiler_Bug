rust
#![feature(test)]

use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;

extern crate test;
use test::Bencher;

#[bench]
fn hash_str(b: &mut Bencher) {
    let mut hasher = DefaultHasher::new();
    b.iter(|| {
        include_str!("x.rs").hash(&mut hasher);
    });
}

#[bench]
fn hash_bytes(b: &mut Bencher) {
    let mut hasher = DefaultHasher::new();
    b.iter(|| {
        include_bytes!("x.rs").hash(&mut hasher);
    });
}
