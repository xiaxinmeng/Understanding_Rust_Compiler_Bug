 rust
#![crate_type="lib"]

#[derive(Copy)]
pub struct Foo {
    pub objs: [u8; 512 * 128],
}
impl ::std::default::Default for Foo {
    fn default() -> Foo {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}
