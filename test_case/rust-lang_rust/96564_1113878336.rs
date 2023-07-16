rust
#![feature(type_alias_impl_trait)]
type Ty<'a> = impl Sized;
fn defining(s: &str) -> Ty<'_> { s }
fn execute(ty: Ty<'_>) -> &str { todo!() }
