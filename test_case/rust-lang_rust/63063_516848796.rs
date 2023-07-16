rust
#![feature(existential_type)]

#[derive(Debug)]
struct Bar(usize);

existential type Foo: std::fmt::Debug;

pub fn foo() -> Foo {
    Bar(5)
}
