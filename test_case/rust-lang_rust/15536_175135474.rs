 Rust
#![crate_type="rlib"]

const f:f64 = 1.0E+300;

pub static ffull:f64 = f;
pub static ftrunc:f32 = f as f32;
pub static f2i:u64 = f as u64;
pub static i2f:f32 = 0xFFFFFFFFFFFFFFFFu64 as f32;
