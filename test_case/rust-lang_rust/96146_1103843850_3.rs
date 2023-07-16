rust
#![feature(type_alias_impl_trait)]
trait Trait<'a> { type Out; }
impl<'a> Trait<'a> for () { type Out = &'a (); }

type Inner = impl Sized;
fn outer_impl() -> impl for<'a> Trait<'a, Out = Inner> {}
