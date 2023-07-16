
struct Foo { x: ~int, }

impl Drop for Foo {
    fn finalize(&self) { fail!(); }
}

fn main() {
    let _x = Foo { x: ~5, };
    fail!();
}
