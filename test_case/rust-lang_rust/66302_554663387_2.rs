rust
static mut MUTABLE: u32 = 0;
const BAD: u32 = unsafe { MUTABLE };
