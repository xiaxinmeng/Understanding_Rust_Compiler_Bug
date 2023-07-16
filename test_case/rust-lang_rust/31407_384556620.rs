rust
pub static F: f32 = unsafe {
    union Helper {
        bits: u32,
        f: f32,
    }
    Helper { bits: 0x0000_0000 }.f
};
