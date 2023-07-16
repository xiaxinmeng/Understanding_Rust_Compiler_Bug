rust
trait Foo: Sized {
    fn new() -> Self;

    fn foo() {
        let me = Self::new();

        || loop {
            me;
        };
    }
}
