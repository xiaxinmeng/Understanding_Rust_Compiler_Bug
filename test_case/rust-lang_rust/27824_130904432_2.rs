 rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Rem for f32 {
    type Output = f32;

    // see notes in `core::f32::Float::floor`
    #[inline]
    #[cfg(target_env = "msvc")]
    fn rem(self, other: f32) -> f32 {
        (self as f64).rem(other as f64) as f32
    }

    #[inline]
    #[cfg(not(target_env = "msvc"))]
    fn rem(self, other: f32) -> f32 {
        extern { fn fmodf(a: f32, b: f32) -> f32; }
        unsafe { fmodf(self, other) }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Rem for f64 {
    type Output = f64;

    #[inline]
    fn rem(self, other: f64) -> f64 {
        extern { fn fmod(a: f64, b: f64) -> f64; }
        unsafe { fmod(self, other) }
    }
}
