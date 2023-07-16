rust
#[repr(packed)]
pub struct Foo {
    pub a: [u8; 32],
    pub b: [u8; 32],
    pub c: [u32; 4],
}

pub fn a(s: &Foo) -> &[u32] {
    &s.c
}
