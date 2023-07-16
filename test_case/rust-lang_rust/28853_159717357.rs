 rust
trait Borrow<T: ?Sized> {}
impl<T: ?Sized> Borrow<T> for T {}
