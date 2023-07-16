rust
struct Foo;

impl Foo {
    fn bar() {
        struct Baz(u8, Option<Box<Self>>);
        let cons: Option<Box<Baz>> = None;
        let _ = Baz(1, cons); // Make sure that `Self` refers to the right thing
    }
}
