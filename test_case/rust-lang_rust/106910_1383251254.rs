rust
#![feature(type_alias_impl_trait)]
type Ty<'a> = impl Sized + 'a;
fn define<'a>() -> Ty<'a> {}
fn test(_: &'static fn(Ty<'_>)) {}
