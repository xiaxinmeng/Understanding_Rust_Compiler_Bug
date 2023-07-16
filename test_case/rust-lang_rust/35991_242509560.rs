 rust
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        panic!();
    }
}
