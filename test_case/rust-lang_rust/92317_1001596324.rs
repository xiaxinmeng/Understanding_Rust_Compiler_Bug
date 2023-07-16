plain
[RUSTC-TIMING] rustc_typeck test:false 65.920
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
[RUSTC-TIMING] rustc_driver test:false 32.166
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
error[E0425]: cannot find value `mi_aligned_alloc` in crate `mimallocate_sys`
   |
   |
34 |             mimallocate_sys::mi_aligned_alloc;
   |                              ^^^^^^^^^^^^^^^^ not found in `mimallocate_sys`
For more information about this error, try `rustc --explain E0425`.
[RUSTC-TIMING] rustc_main test:false 0.127
error: could not compile `rustc-main` due to previous error
Build completed unsuccessfully in 0:11:54
