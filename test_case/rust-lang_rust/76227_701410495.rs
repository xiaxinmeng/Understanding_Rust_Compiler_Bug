rust
impl<T> Poll<T> {
    pub const fn is_ready(&self) -> bool;
    pub const fn is_pending(&self) -> bool;
}
