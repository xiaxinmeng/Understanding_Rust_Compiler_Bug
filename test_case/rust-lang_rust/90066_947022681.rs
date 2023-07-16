plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: type does not implement `Debug`; consider adding `#[derive(Debug)]` or a manual implementation
 --> library/alloc/src/boxed/thin.rs:4:1
  |
4 | pub struct ThinBox<T>(T);
  |
  |
  = note: `-D missing-debug-implementations` implied by `-D warnings`
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:43
