plain

error: function `register_dtor` is never used
  --> library/std/src/sys/unix/thread_local_dtor.rs:20:15
   |
20 | pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 4.256
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:08:32
