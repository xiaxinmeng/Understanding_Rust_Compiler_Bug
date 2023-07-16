rust
#![feature(platform_intrinsics, repr_simd)]

extern "platform-intrinsic" {
    pub(crate) fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U;
}

#[repr(simd)]
pub struct Simd<T, const LANES: usize>([T; LANES]);

impl<T, const LANES: usize> Simd<T, LANES> {
    pub fn new(array: [T; LANES]) -> Self {
        Self(array)
    }
}

mod biteq {
    pub trait BitEq {
        fn biteq(&self, other: &Self) -> bool;
    }
    impl BitEq for i8 {
        fn biteq(&self, _: &Self) -> bool {
            false
        }
    }

    pub struct BitEqWrapper<T>(pub T);
    impl<T: BitEq> PartialEq for BitEqWrapper<T> {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }
}

fn main() {
    pub fn implementation(x: [i8; 16]) {
        use crate::biteq::BitEqWrapper;

        let a = unsafe { crate::simd_reduce_mul_ordered(crate::Simd::<i8, 16>::new(x), 1) };
        let b = 0;

        let left = BitEqWrapper(a);
        let right = BitEqWrapper(b);

        let _ = left == right;
    }

    implementation([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}
