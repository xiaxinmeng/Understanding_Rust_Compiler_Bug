rust
#![feature(portable_simd)]
fn main() {
    use core::simd::Simd;
    let a = Simd::from_array([0, 4, 1, 5]);
    let b = Simd::from_array([2, 6, 3, 7]);
    let (x, y) = a.deinterleave(b);
    assert_eq!(x.to_array(), [0, 1, 2, 3]);
    assert_eq!(y.to_array(), [4, 5, 6, 7]);
}
