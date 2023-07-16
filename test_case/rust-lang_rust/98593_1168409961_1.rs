rust
impl GPIOA {
    pub const PTR: *const gpioa::RegisterBlock = 0x4002_0000 as *const _;
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        Self::PTR
    }
}
