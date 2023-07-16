rust
fn foo<R>(_: R) where R: Trait<R::Assoc> {}
trait Trait<A> { }
