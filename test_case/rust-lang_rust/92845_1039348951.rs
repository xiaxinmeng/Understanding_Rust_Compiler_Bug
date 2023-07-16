plain
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.023
error: unused import: `c_int`
  --> library/panic_unwind/src/emcc.rs:15:18
   |
15 | use libc::{self, c_int};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] panic_abort test:false 0.036
[RUSTC-TIMING] panic_unwind test:false 0.041
error: could not compile `panic_unwind` due to previous error
warning: build failed, waiting for other jobs to finish...
