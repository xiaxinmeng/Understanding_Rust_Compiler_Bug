rust
impl<B, C> ControlFlow<B, C> {
    pub fn is_break(&self) -> bool;
    pub fn is_continue(&self) -> bool;
}
