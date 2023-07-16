plain
   Compiling rustc-demangle v0.1.21
error: non-binding let on a type that implements `Drop`
   --> library/alloc/src/vec/into_iter.rs:324:21
    |
324 |                     let _ = RawVec::from_raw_parts_in(self.0.buf.as_ptr(), self.0.cap, alloc);
    |
    |
    = note: `-D let-underscore-drop` implied by `-D warnings`
help: consider binding to an unused variable to avoid immediately dropping the value
    |
324 |                     let _unused = RawVec::from_raw_parts_in(self.0.buf.as_ptr(), self.0.cap, alloc);
help: consider immediately dropping the value
    |
    |
324 |                     drop(RawVec::from_raw_parts_in(self.0.buf.as_ptr(), self.0.cap, alloc));

error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:59
