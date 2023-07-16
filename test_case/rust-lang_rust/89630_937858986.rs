plain
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: the feature `never_type` has been stable since 1.57.0 and no longer requires an attribute to enable
  --> compiler/rustc_data_structures/src/lib.rs:22:12
   |
22 | #![feature(never_type)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_data_structures` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:05:28
