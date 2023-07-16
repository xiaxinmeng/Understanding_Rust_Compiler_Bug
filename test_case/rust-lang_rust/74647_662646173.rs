
error: unused import: `ssize_t`
  --> src/libstd/sys/unix/fd.rs:10:27
   |
10 | use libc::{c_int, c_void, ssize_t};
   |                           ^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: aborting due to previous error

[RUSTC-TIMING] std test:false 3.917
error: could not compile `std`.
