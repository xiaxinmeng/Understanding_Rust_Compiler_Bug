rust
trait Foo { .. }
trait Bar { .. }
impl<T: Foo> Bar for T { .. }
impl<T: Foo> Foo for &T { .. }
impl<T: Bar> Bar for &T { .. }
