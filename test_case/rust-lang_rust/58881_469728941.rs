rust
# Today's nightly, with LLVM assertions
$ rustup-toolchain-install-master a9da8fc9c267c08cfdb8cf5b39da14f154d12939 --alt
$ rustc +a9da8fc9c267c08cfdb8cf5b39da14f154d12939-alt foo.rs
warning: `extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout
 --> foo.rs:2:38
  |
2 |     pub static g_cubeb_log_callback: unsafe extern "C" fn(*const u8, ...);
  |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(improper_ctypes)] on by default
  = help: consider using a struct instead

Wrong types for attribute: byval inalloca nest noalias nocapture nonnull readnone readonly signext sret zeroext dereferenceable(1) dereferenceable_or_null(1)
  call void (i8*, ...) %4(i8* %5, { i64, i64 } %10, { i64, i64 } %14, { i64, i64 } byval noalias nocapture dereferenceable(16) %18)
in function _ZN3foo4main17h0e6b3bfd37376208E
LLVM ERROR: Broken function found, compilation aborted!
