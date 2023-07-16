rust
use std::marker::PhantomPinned;
use std::pin::Pin;

fn main() {
    let val: Pin<Pin<&PhantomPinned>> = Pin::new(Pin::new(&PhantomPinned));
}
