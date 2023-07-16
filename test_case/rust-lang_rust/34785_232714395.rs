 rust
pub fn eq_discriminant<T>(x: &T, y: &T) -> bool {
    unsafe {
        intrinsics::discriminant_value(x) == intrinsics::discriminant_value(y)
    }
}
