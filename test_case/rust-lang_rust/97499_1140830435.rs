plain
[RUSTC-TIMING] object test:false 7.325
error: function is never used: `register_dtor_fallback`
  --> library\std\src\sys_common\thread_local_dtor.rs:19:15
   |
19 | pub unsafe fn register_dtor_fallback(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 5.853
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:47
Build completed unsuccessfully in 0:00:47
make: *** [Makefile:72: ci-subset-2] Error 1
