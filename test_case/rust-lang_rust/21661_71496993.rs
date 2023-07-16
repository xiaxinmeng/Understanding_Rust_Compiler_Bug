 rust
#![crate_type="rlib"]
pub trait Foo where Self::Bar: Sized {
}
