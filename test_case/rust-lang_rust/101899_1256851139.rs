rust
  // --target wasm32-unknown-unknown
  // default-features = false
  // SAFETY: No preconditions.
  let cctx = unsafe { zstd_sys::ZSTD_createCCtx() };
  assert!(!ptr.is_null());
  // SAFETY: `ptr` was allocated with `ZSTD_createCCtx()`.
  unsafe { zstd_sys::ZSTD_freeCCtx(cctx) };
  // calls alloc::dealloc() with size 1 on an allocation of size 3672
  // at zstd_sys::wasm_shim::rust_zstd_wasm_shim_free()
  