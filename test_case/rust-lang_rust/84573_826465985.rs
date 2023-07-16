plain
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: the feature `never_type` has been stable since 1.53.0 and no longer requires an attribute to enable
  --> compiler/rustc_infer/src/lib.rs:22:12
   |
22 | #![feature(never_type)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_infer`

