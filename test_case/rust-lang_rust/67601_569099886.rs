rust
pub struct SyncPtr(*const u32);
unsafe impl Sync for SyncPtr {}
pub const FOO:  &'static SyncPtr = &SyncPtr(&BAR as _);
pub const BAR: u32 = 0;
