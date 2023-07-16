rust
#![no_std]
#![crate_type="lib"]

pub fn foo() -> bool {
    ::core::num::Float::is_nan(0f32)
}
