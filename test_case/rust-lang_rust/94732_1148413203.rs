plain
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
[RUSTC-TIMING] rustc_symbol_mangling test:false 16.900
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
error: the feature `nll` has been stable since 1.63.0 and no longer requires an attribute to enable
  |
6 | #![feature(nll)]
  |            ^^^
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
[RUSTC-TIMING] rustc_query_impl test:false 7.581
error: could not compile `rustc_query_impl` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_incremental test:false 26.446
