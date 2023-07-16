rust
#![feature(platform_intrinsics, repr_simd)]

#[repr(simd)]
pub struct Simd<T, const LANES: usize>([T; LANES]);

impl<const LANES: usize> Simd<i8, LANES> {
    pub fn horizontal_product(self) -> i8 {
        extern "platform-intrinsic" {
            fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U;
        }

        unsafe { simd_reduce_mul_ordered(self, 1) }
    }
}

fn cb(x: [i8; 16]) {
    let a: i8 = Simd(x).horizontal_product();
    let _ = a.to_string();
}

fn main() {
    let x = [0; 16];
    let _ = std::panic::catch_unwind(|| cb(x));
}
