rust
#![feature(core_intrinsics)]

pub fn add(a: bool, b: bool) -> bool {
    std::intrinsics::wrapping_add(a, b)
}
