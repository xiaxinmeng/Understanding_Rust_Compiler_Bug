plain
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
error: the feature `pointer_byte_offsets` has been stable since CURRENT_RUSTC_VERSION and no longer requires an attribute to enable
  --> compiler/rustc_arena/src/lib.rs:19:12
   |
19 | #![feature(pointer_byte_offsets)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
    Checking rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
error: could not compile `rustc_arena` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_arena` due to previous error
