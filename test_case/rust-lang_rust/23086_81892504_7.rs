 rust
trait Foo { .. }
impl<T:Copy> Foo for T { .. }
impl<T:Copy> Foo for Range<T> { .. }
