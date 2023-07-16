rust
#[repr(u8)]
pub enum Foo {
    A,
    B(),
    C{},
}

pub fn a(foo: Foo) -> u8 {
    foo as u8
}
