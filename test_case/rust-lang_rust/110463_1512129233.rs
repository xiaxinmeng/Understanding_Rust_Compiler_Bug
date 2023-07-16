plain
   Compiling object v0.29.0
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.020
[RUSTC-TIMING] panic_abort test:false 0.031
[RUSTC-TIMING] panic_unwind test:false 0.054
error: calls to `std::mem::drop` with a reference instead of an owned value does nothing
   --> library/stdarch/crates/std_detect/src/detect/os/linux/auxvec.rs:250:5
250 |     drop(buf);
    |     ^^^^^---^
    |          |
    |          |
    |          argument has type `&[usize; 64]`
    = note: use `let _ = ...` to ignore the expression or result
    = note: use `let _ = ...` to ignore the expression or result
    = note: `-D drop-ref` implied by `-D warnings`
[RUSTC-TIMING] std_detect test:false 0.110
error: could not compile `std_detect` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] alloc test:false 2.928
