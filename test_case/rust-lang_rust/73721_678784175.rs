rust
pub struct Foo;
pub trait Bar {
    const BAZ: u8;
}
impl Bar for Foo {
    #[doc(alias = "thing")]
    const BAZ: u8 = 0;
}
