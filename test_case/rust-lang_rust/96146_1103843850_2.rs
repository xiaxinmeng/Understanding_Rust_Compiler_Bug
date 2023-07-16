rust
#![feature(type_alias_impl_trait)]
type Inner<'a> = impl Sized;
fn outer_impl() -> impl for<'a> Fn(&'a ()) -> Inner<'a> { |x| x }
