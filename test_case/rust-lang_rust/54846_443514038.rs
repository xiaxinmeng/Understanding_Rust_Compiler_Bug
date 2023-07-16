rust
struct Foo(u32);

impl Foo {
    const fn new(x: u32) -> Self {
        Foo(x)
    }
}

fn bar() -> &'static Foo {
    &Foo(42) // works because of rvalue promotion
}

fn baz() -> &'static Foo {
    &Foo::new(42) // error: temporary value does not live long enough
}
