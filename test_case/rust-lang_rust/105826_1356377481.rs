rust
#![feature(type_alias_impl_trait)]
type Opaque<'a> = impl Sized;
fn define<'a>() -> Opaque<'a> {}
fn test<'a>() {
    None::<&'static Opaque<'a>>;
}
