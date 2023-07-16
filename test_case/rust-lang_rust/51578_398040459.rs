rust
#![feature(nll)]

use std::rc::Rc;

struct Combine(u8);

fn poll(welf: Rc<Combine>) {
    drop(welf);
    welf.0 = 42;
}

fn main() {}
