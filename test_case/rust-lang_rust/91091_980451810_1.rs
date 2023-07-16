rust
pub enum ControlFlow<B, C = ()> {
    Continue(C),
    Break(B),
}
