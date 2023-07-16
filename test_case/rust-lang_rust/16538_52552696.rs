 Rust
pub static DefaultRngType : RngType = ffi::FFI::wrap(ffi::gsl_rng_default as *mut ffi::gsl_rng_type);
