rust
// in src/libpanic_unwind/lib.rs
#![cfg_attr(not(all(windows, target_arch = "aarch64")), panic_runtime)]
