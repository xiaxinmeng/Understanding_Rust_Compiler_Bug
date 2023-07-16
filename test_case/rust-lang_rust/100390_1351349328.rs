rust
/// Converts `bool` straight to `f32`.
///
/// Ideally Rust should support `f32::from(bool)`, but in the meantime this is a nicer short hand than
/// having to do `f32::from(i16::from(bool))`, or a more error prone lossy cast with `as i32 as f32` that we used to do
///
/// Rust support issue: <https://github.com/rust-lang/rust/issues/74015>
pub fn f32_bool(b: bool) -> f32 {
    // in Rust CPU code we would do this, but for rust-gpu that would use 16-bit
    // integer operations and fail with:
    // error: u16 without OpCapability Int16
    //
    // f32::from(i16::from(b))
    //
    // so do "lossy" cast that is 32-bit
    #[allow(clippy::cast_lossless)]
    {
        b as u32 as f32
    }
}
