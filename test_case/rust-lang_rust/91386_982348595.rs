plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused import: `Lint`
  --> compiler/rustc_mir_transform/src/lib.rs:41:26
   |
41 | use self::pass_manager::{Lint, MirPassC};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:51
