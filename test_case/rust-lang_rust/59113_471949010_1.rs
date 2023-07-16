rust
fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output>;
