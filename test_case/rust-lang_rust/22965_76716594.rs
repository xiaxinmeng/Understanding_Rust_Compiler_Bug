 rust
#[unsafe_destructor]
impl<T: Send> Drop for X<T> {
    fn drop(&mut self) {
    }
}
