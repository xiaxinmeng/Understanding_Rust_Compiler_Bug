rust
#![feature(existential_type)]

pub trait Test {
    type Foo;
    fn foo() -> Self::Foo;
}

struct FooImpl;

impl Test for () {
    existential type Foo: ;
    fn foo() -> Self::Foo {
        FooImpl
    }
}
