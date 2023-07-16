rust
#[repr(C)]
pub struct Foo {
    a: u8,
    // 1 byte padding
    b: u16,
    // 4 bytes padding
    c: u64,
}

pub fn foo(_: Foo) {}
