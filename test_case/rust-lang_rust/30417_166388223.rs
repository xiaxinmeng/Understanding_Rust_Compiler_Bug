 rust
#![feature(test)]
#![feature(static_mutex, const_fn)]

extern crate test;

use std::sync::StaticMutex;

pub fn doit() {
    static L: StaticMutex = StaticMutex::new();
    drop(L.lock());
}

#[bench]
fn bench_a(bh: &mut test::Bencher) {
    bh.iter(|| { doit() });
}
