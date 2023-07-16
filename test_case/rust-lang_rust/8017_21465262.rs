 rust
struct Foo(uint);

impl Drop for Foo {
    fn drop(&self) {}
}

fn main() {
    let _a = Foo(1);
}
