rust
fn poll(mut self: Pin<&mut Self>, task: &mut Task) -> Poll<Self::Output>;
