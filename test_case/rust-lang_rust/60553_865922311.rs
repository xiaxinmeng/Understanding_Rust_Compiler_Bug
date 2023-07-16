rust
#[repr(u8)]
enum Foo {
    A(u8) = 0,
    B(i8) = 1,
    C(bool) = 42,
}
