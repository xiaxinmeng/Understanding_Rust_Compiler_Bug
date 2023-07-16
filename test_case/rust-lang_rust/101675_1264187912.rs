plain

error: unused import: `self`
  --> library/std/src/sys/unix/fs.rs:12:24
   |
12 | use crate::sys::time::{self, SystemTime};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] std test:false 3.942
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:19:25
