rust
fn poll(mut self: Pin<&mut Self>, waker: &Waker) -> Poll<Self::Output>;
