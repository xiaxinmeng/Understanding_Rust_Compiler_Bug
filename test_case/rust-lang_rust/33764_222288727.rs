 rust
#![feature(repr_simd)]

#[repr(simd)]
#[allow(non_camel_case_types)]
pub struct u64x2(pub u64, pub u64);

fn main() {
    const T0: u64x2 = u64x2(0, 0);
    println!("{}", T0.0.rotate_right(8));
}
