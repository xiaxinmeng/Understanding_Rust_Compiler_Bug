rust
#[repr(C)]
pub struct Big([u64; 16]);

#[no_mangle]
pub extern fn test_Big(_: Big) -> Big { loop {} }
