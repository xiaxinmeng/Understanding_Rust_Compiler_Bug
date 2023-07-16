 rust
pub use self::inner::Foo;
pub type Bar = self::inner::Bar;
mod inner {
    struct Foo;
    struct Bar;
}
