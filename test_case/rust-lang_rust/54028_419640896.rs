rust
#[cfg(target_arch = "x86")]
use std::arch::x86::__m128d as f64x2;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m128d as f64x2;
pub struct Complex {
    _dummy: [f64x2; 0],
    real: f64,
    imag: f64,
}
