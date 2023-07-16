 rust
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        *self = Foo;
    }
}

fn main() {
    let _a = Foo;
}
