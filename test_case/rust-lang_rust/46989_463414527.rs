rust
trait Foo {}
impl<Ret, A> Foo for fn(A) -> Ret {}
impl<Ret, A> Foo for for<'a> fn(&'a A) -> Ret {}
