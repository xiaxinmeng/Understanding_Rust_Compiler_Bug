rust
trait Trivial {}
impl<T: ?Sized> Trivial for T {}
