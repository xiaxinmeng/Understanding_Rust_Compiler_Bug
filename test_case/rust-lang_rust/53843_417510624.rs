rust
impl<P> Deref for Pin<P> where {
    type Target = P;
    fn deref(&self) -> &P {
        &self.0
    }
}
