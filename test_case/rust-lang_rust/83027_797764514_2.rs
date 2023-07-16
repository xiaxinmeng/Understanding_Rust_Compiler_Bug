rust
pub(crate) trait m256iExt: Sized {
    // ...
    // #[target_feature(default)]
    fn as_i32x8(self) -> i32x8 {
        unsafe { transmute(self.as_m256i()) }
    }
}
