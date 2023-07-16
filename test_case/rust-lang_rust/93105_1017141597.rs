plain
    Checking core v0.0.0 (/checkout/library/core)
error: function is never used: `box_free`
   --> library/alloc/src/alloc.rs:328:17
    |
328 | const unsafe fn box_free<T: ?Sized, A: ~const Allocator + ~const Drop>(
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:44
