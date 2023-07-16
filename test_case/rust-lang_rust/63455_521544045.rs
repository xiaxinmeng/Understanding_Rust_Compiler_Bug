rust
#![feature(core_intrinsics)]
#![no_std]

pub fn sqrt_f32(f: f32) -> f32 {
  unsafe { core::intrinsics::sqrtf32(f) }
}

pub fn sqrt_f64(f: f64) -> f64 {
  unsafe { core::intrinsics::sqrtf64(f) }
}
