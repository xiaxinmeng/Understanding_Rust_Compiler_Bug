rust
use std::pin::Pin;

pub trait Foo {
    fn pin_recv(self: Pin<&mut Self>);
}

impl Foo for u64 {
    fn pin_recv(self: Pin<&mut u64>) {}
}

pub struct Wrapped<T> {
    num: T,
}

impl<T: Foo> Foo for Wrapped<T> {
    fn pin_recv(self: Pin<&mut Self>) {
        self.num.pin_recv()
        // correct: `Pin::new(&mut self.num).pin_recv()`
    }
}
