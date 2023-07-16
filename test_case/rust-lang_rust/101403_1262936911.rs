plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: use `Session::unstable_options` instead of this field
  --> compiler/rustc_codegen_llvm/src/back/lto.rs:85:21
   |
85 |                 if !cgcx.opts.unstable_opts.unstable_options {
   |
   |
   = note: `-D rustc::bad-opt-access` implied by `-D warnings`
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_llvm` due to previous error
Build completed unsuccessfully in 0:02:31
