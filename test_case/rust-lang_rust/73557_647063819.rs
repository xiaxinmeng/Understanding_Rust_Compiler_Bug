rust
#[repr(align(16))]
pub struct Align16<T>(pub T);

struct Foo {
    a: Align16<i32>,
}
