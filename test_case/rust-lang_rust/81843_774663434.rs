rust
struct Foo(u8);

impl Drop for Foo {
    fn drop(&mut self) {}
}

static mut FOO: Foo = Foo(3);
