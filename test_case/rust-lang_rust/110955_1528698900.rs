plain
   Compiling rand_xorshift v0.3.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
error: using `.clone()` on a double reference, which returns `&i32` instead of cloning the inner type
  |
  |
5 |     let z: &i32 = (&y).clone();
  |
  |
  = note: `-D suspicious-double-ref-op` implied by `-D warnings`
error: could not compile `core` (test "coretests") due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:18:45
