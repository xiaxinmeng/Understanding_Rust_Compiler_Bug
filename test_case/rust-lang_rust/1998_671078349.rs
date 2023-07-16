rust
#[cfg(foo)]
pub mod host {
    pub struct Foo(pub usize);

    pub struct Bar;
}

#[cfg(not(foo))]
pub mod host {
    pub struct Foo(pub i8);

    pub struct Qux;
}

fn foo() -> host::Foo { host::Foo(0) }
