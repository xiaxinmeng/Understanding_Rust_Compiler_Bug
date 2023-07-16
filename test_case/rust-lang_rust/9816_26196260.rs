 rust
trait CowPtr<T> {
    fn cow<'r>(&'r mut self) -> &'r mut T;
}

impl<T> CowPtr<T> for Arc<T> {
     // ...
}
impl<T> CowPtr<T> for ~T {
    // ...
}
