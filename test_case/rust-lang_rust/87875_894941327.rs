plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: the feature `bindings_after_at` has been stable since 1.54.0 and no longer requires an attribute to enable
  |
8 | #![feature(bindings_after_at)]
  |            ^^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:09:53
