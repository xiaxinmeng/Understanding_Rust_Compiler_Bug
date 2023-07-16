rust
pub unsafe fn always_true() -> bool {
    let value: i32 = std::mem::MaybeUninit::uninit().assume_init();
    value < 0 || value == 0 || value > 0
}
