plain
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: the feature `let_chains` has been stable since 1.64.0 and no longer requires an attribute to enable
  --> compiler/rustc_passes/src/lib.rs:11:12
   |
11 | #![feature(let_chains)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:05
