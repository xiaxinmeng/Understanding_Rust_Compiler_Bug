rust
// core::num

/// NonZero<i8> = NonZeroI8
/// NonZero<i16> = NonZeroI16
/// etc
pub type NonZero<N> = <N as private::Primitive>::NonZero;
