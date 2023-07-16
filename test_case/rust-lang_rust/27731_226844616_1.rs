 rust
trait Simd { // no longer an unsafe trait
    type EqType;

    // we now include a method for the various simd operations we might want to do:
    fn eq(x: &Self, y: &Self) -> Self::EqType;
    ...
}

#[simd_intrinsic]
fn eq_u32x4(x: u32x4, y: u32x4) -> boolx4 {...}

impl Simd for u32x4 {
    #[inline(always)]
    fn eq(x: &Self, y: &Self) -> Self::EqType {
         eq_u32x4(x, y)
    }
}
