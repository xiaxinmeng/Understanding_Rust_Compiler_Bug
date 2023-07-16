plain
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling core v0.0.0 (/checkout/library/core)
error: unnecessary allocation, use `&` instead
   |
   |
28 |             assert!(a == Box::new(8));
   |
   |
   = note: `-D unused-allocation` implied by `-D warnings`

error: unnecessary allocation, use `&` instead
   |
   |
34 |             assert!(a == Box::new(Test));

error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:20:42
