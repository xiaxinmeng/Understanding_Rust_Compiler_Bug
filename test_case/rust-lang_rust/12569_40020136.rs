 rust
struct Foo;

impl Foo {
    pub fn foo() -> uint { 1 }
}

mod Foo {
    pub fn foo() -> uint { 2 }
}
