rust
pub const FOO:  &'static *const u32 = &(&BAR as _);
pub const BAR: u32 = 0;
