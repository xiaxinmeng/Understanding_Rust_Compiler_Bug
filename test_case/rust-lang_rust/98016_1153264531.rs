rust
#![crate_type = "lib"]
#![feature(repr_simd)]

#[repr(simd)]
pub struct Simd([u8; 8]);

fn to_array_inner<T>(x: Simd) -> [u8; 8] {
    x.0
}

pub fn to_array_outer(x: Simd) -> [u8; 8] {
    to_array_inner::<()>(x)
}
