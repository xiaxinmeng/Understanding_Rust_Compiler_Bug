rust
use std::marker::PhantomData;

struct Buffer<T: ?Sized> {
    buf: [u8; Self::LEN],
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Buffer<T> {
    const LEN: usize = 64;
}

fn main() {
    println!("Hello, world!");
}
