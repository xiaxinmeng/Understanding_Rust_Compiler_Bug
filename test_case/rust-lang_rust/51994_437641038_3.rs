rust
enum Foo { Variant(u8), }

impl Foo {
    fn bar() {
        let x = Self::Variant(1);
    }
}
