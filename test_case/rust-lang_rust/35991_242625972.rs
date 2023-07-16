 rust
#![crate_type = "lib"]
pub struct Foo;
impl Foo {
    pub fn foo() {}
}
pub fn bar() {
    Foo::foo();
}
