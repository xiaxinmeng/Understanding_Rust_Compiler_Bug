plain

error: function `register_dtor` is never used
 --> library/std/src/sys/wasm/../unsupported/thread_local_dtor.rs:3:15
  |
3 | pub unsafe fn register_dtor(_t: *mut u8, _dtor: unsafe extern "C" fn(*mut u8)) {
  |
  |
  = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 2.583
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:12:47
