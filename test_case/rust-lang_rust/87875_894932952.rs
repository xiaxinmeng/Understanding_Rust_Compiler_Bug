plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
error[E0718]: `box_free` language item must be applied to a function with at least 1 generic argument
    |
    |
322 | #[cfg_attr(not(test), lang = "box_free")]
...
...
329 | pub(crate) unsafe fn box_free<T: ?Sized, A: Allocator>(ptr: Unique<T>, alloc: A) {
    |                              ------------------------- this function has 2 generic arguments
For more information about this error, try `rustc --explain E0718`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
