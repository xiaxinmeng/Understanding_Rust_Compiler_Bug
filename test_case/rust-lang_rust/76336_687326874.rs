rust
#![no_std]

#[repr(C)] pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub extern "C" fn convert(color: Color) -> u16 {
    (((((color.red >> 3) & 0x1F) as u32) << 0)
    | ((((color.green >> 3) & 0x1F) as u32) << 5)
    | ((((color.blue >> 3) & 0x1F) as u32) << 10)) as u16
}
