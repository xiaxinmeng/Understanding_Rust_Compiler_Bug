plain
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: the feature `let_chains` has been stable since 1.62.0 and no longer requires an attribute to enable
  --> compiler/rustc_ast_passes/src/lib.rs:11:12
   |
11 | #![feature(let_chains)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_ast_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:30
