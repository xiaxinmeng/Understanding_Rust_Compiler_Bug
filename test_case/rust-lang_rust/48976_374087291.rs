rust
#![feature(nll,rustc_attrs)]
use std::cell::RefCell;

#[derive(Default)]
struct VContainer {
    buffer: Vec<usize>,
    add_idx: usize,
}

impl VContainer {
    fn new() -> Self {
        Self::default()
    }

    fn append(&mut self, us: usize) {
        self.buffer[self.add_idx] = us;
    }
}

fn producer(ss: RefCell<VContainer>) {
    let mut ss = ss.borrow_mut();

    for i in 0 .. 21 {
        ss.append(i);
    }
}

fn main() {
    let v = VContainer::new();
    producer(RefCell::new(v))
}
