rust
#![feature(platform_intrinsics)]
#![feature(staged_api)]
struct f32x3();
extern "platform-intrinsic" {
    fn simd_extract<T, U>(x: T, idx: u32) -> U;
}
const Y0: i8 = unsafe { simd_extract(f32x3, 0) };
