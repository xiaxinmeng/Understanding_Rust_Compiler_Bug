rust
#![feature(repr_simd)]

#[repr(simd)]
pub struct Vec3 { // error[E0077]: SIMD vector element type should be machine type
    pub x: bool,
    pub y: bool,
    pub z: bool,
}
