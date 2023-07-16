rust
#[repr(u8, C)]
enum Foo {
    A = 0,
    B { a: u16 } = 1,
    C { a: u32, b: u16 } = 2,
}
