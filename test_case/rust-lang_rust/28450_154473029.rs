 rust
// these do the same thing, and compile today
pub use self::internal::Foo;
pub type Bar = self::internal::Bar;
mod internal {
    pub struct Foo;
    pub struct Bar;
}
