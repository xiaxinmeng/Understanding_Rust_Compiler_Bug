rust
use std::ops::Deref;

pub trait X {
    fn f(&mut self);
}

impl<T> X for [T] {
    fn f(&mut self) {}
}

impl<T: X + ?Sized> X for &T {
    fn f(&mut self) {
        self.deref().f()
    }
}

const DATA: &[u8] = &[0, 1];

fn main() {
    let mut data: &'static [u8] = DATA;
    data.f();
}
