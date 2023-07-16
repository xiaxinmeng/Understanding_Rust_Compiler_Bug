 rust
#![feature(test)]
#![feature(const_fn)]

extern crate test;

use std::cell::Cell;

#[bench]
fn bench_a(bh: &mut test::Bencher) {
    thread_local!(static C: Cell<usize> = Cell::new(0));
    bh.iter(|| {
        C.with(|c| {
            let r = c.get();
            c.set(r + 1);
            r
        })
    })
}
