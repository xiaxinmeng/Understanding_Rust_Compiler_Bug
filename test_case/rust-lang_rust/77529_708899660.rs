rust
// compile-flags: --crate-type=rlib -C link-dead-code
#![feature(platform_intrinsics)]
#![feature(repr_simd)]

extern "platform-intrinsic" {
    pub fn simd_reduce_all<T>(x: T) -> bool;
}

#[repr(simd)]
// #[repr(C)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vec3<bool> {
    #[inline]
    pub fn reduce_and(self) -> bool {
        unsafe { simd_reduce_all(self) }
    }
}
