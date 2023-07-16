
[00:48:53] error: function is never used: `set_bit`
[00:48:53]   --> libstd/../stdsimd/stdsimd/arch/detect/cache.rs:13:1
[00:48:53]    |
[00:48:53] 13 | pub const fn set_bit(x: u64, bit: u32) -> u64 {
[00:48:53]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:48:53]    |
[00:48:53] note: lint level defined here
[00:48:53]   --> libstd/lib.rs:232:31
[00:48:53]    |
[00:48:53] 232| #![cfg_attr(not(stage0), deny(warnings))]
[00:48:53]    |                               ^^^^^^^^
[00:48:53]    = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:48:53] 
[00:48:53] error: method is never used: `set`
[00:48:53]   --> libstd/../stdsimd/stdsimd/arch/detect/cache.rs:48:5
[00:48:53]    |
[00:48:53] 48 |     pub fn set(&mut self, bit: u32) {
[00:48:53]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:48:53] 
[00:48:53] error: aborting due to 2 previous errors
