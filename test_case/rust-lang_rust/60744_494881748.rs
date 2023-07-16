rust
struct Foo<const X: u8>(u32);

impl<const X: u8> Clone for Foo<{X}> {
    fn clone(&self) -> Self {
        Foo(self.0)
    }
}

#[test]
fn test_foo() {
    let foo: Foo<4> = Foo(1);
    foo.clone();
}
