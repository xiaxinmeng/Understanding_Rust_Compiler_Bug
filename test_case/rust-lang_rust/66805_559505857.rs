rust
#![feature(proc_macro_hygiene)]

use std::marker::PhantomData;
use pm::crash;

mod a;

struct Sender<T> {
    _p: PhantomData<T>,
}

impl<T> Sender<T> {
    fn new() -> Self { unimplemented!() }
    fn send(self, _: T) {}
}

fn func() {}

fn main() {
    stream! {
        ()?;
        yield func();
    }   
}
