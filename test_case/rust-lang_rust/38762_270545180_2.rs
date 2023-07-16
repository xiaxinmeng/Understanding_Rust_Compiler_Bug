rust
#[repr(C)]
struct Foo {
    a: i128,
    b: i8,
    c: u16,
}

fn main() {
    assert_eq!(16, ::std::mem::align_of::<Foo>()); // fails currently
}
