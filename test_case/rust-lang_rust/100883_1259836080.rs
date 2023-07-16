plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: the feature `doc_cfg` has been stable since 1.65.0 and no longer requires an attribute to enable
  --> library/stdarch/crates/std_detect/src/lib.rs:16:33
   |
16 | #![feature(staged_api, stdsimd, doc_cfg, allow_internal_unstable)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: could not compile `std_detect` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:19
