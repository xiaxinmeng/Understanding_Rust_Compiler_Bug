rust
#![feature(inline_const)]

pub fn create_array() -> [Result<u16, u8>; 512] {
    const { [Ok(0); 512] }
}
