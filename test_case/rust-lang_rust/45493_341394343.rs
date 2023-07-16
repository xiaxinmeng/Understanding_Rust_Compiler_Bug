rust
// -Z mir-opt-level=2 or higher

trait Array {
    type Item;
}

fn foo<A: Array>() {
    let _: *mut A::Item = std::ptr::null_mut();
}

struct Foo;
impl Array for Foo { type Item = i32; }

fn main() {
    foo::<Foo>();
}
