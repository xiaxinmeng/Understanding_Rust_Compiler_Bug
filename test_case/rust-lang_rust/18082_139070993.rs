 rust
mod private {
    pub struct Foo;
}
pub fn take_foo(_: private::Foo) { }
