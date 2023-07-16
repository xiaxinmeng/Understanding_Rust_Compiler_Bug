rust
#![feature(type_alias_impl_trait)]
type StrRef<'a> = impl AsRef<str>;
fn define(s: &str) -> StrRef<'_> { s }
