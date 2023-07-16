plain
    Checking chalk-engine v0.80.0
error: the feature `derive_default_enum` has been stable since 1.62.0 and no longer requires an attribute to enable
  --> compiler/rustc_feature/src/lib.rs:14:32
   |
14 | #![cfg_attr(bootstrap, feature(derive_default_enum))]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: could not compile `rustc_feature` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_feature` due to previous error
Build completed unsuccessfully in 0:02:00
