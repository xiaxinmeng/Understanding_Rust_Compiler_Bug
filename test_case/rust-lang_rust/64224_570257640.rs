rust
default impl<T> Drop for Box<T> { ... }
impl Drop for Box<dyn Any + Send> {
    #[cold] fn drop(&mut self) { ... }
}
