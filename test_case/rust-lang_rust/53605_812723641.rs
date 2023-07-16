rust
#[repr(transparent)]
pub struct MySlice(pub [u8]);

impl MySlice {
    pub const fn new(v: &[u8]) -> &MySlice {
        unsafe { std::mem::transmute(v) }
    }
}
