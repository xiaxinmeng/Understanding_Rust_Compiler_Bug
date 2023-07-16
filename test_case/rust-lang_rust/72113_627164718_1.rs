text
error: any use of this value will cause an error
 --> src/main.rs:3:42
  |
3 | const OTHER_UNIT: &'static u8 = unsafe { &*(&() as *const _ as *const u8) };
  | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
  |                                          |
  |                                          memory access failed: pointer must be in-bounds at offset 1, but is outside bounds of alloc2 which has size 0
  |
  = note: `#[deny(const_err)]` on by default
