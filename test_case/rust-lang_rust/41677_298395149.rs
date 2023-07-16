Rust
#![allow(unused_variables, dead_code)]

use std::marker::PhantomData;

trait Handle {
    type Inner;
}

struct ResizingHandle<H>(PhantomData<H>);
impl<H> Handle for ResizingHandle<H> {
    type Inner = H;
}

struct Receiver<T, H: Handle<Inner=T>>(PhantomData<H>);

fn channel<T>(size: usize) -> Receiver<T, ResizingHandle<T>> {
    let rx = Receiver(PhantomData);
    rx
}

fn main() {
}
