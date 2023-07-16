 rust
struct Foo { x: ~int, }

impl Drop for Foo {
    fn drop(&self) { fail!(); }
}

fn main() {
    let _x = Foo { x: ~5, };
    fail!();
}
