rust
#![crate_type = "rlib"]

#[repr(u64)]
pub enum NSEventType {
    NSEventTypePressure,
}

pub const A: u64 = NSEventType::NSEventTypePressure as u64;

fn banana() -> u64 {
    A
}
