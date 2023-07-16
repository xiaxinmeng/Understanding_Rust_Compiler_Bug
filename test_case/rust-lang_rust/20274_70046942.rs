 rust
#[repr(u8)]
enum c_void_inner {
    __variant1,
    __variant2
}

#[repr(u8)]
pub struct c_void(c_void_inner);
