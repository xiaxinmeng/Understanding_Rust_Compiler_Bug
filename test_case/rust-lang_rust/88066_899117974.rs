plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling chalk-engine v0.55.0
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error: the feature `bindings_after_at` has been stable since 1.54.0 and no longer requires an attribute to enable
 --> compiler/rustc_expand/src/lib.rs:1:12
1 | #![feature(bindings_after_at)]
  |            ^^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_expand` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:07:54
