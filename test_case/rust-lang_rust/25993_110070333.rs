 rust
#![feature(associated_consts)]

struct Foo;

impl Foo {
    const bar: bool = true;
    fn bar() {}
}
