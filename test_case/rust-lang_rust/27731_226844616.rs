 rust
#[simd_intrinsic(...)]
fn simd_eq<T,U>(t: T, u: T) -> U;

unsafe trait Simd {
    type EqType;
}

fn generic_eq<T:Simd>(t: T, u: T) -> T::EqType {
    simd_eq(t, t)
}

unsafe impl Simd for u32x4 { ... } // etc
