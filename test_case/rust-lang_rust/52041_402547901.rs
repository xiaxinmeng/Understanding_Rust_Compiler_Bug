rust
#![feature(rust_2018_preview)]
#![warn(elided_lifetimes_in_paths)]

use std::cell::{RefCell, Ref};

fn main() {
    // example from `Ref` documentation:
    // https://doc.rust-lang.org/std/cell/struct.Ref.html#examples
    let c = RefCell::new((5, 'b'));
    let b1: Ref<(u32, char)> = c.borrow();
    let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
    assert_eq!(*b2, 5);
}
