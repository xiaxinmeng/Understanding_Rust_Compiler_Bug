plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: the feature `staged_api` is internal to the compiler or standard library
  --> library/stdarch/crates/std_detect/src/lib.rs:16:12
   |
16 | #![feature(staged_api, stdsimd, doc_cfg, allow_internal_unstable)]
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: the feature `allow_internal_unstable` is internal to the compiler or standard library
  --> library/stdarch/crates/std_detect/src/lib.rs:16:42
   |
   |
16 | #![feature(staged_api, stdsimd, doc_cfg, allow_internal_unstable)]
   |
   |
   = note: using it is strongly discouraged
error: could not compile `std_detect` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:40
cat: /tmp/toolstate/toolstates.json: No such file or directory
