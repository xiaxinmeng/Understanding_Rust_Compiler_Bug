plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unused import: `TypeVisitable`
 --> compiler/rustc_infer/src/infer/equate.rs:7:31
  |
7 | use rustc_middle::ty::{TyVar, TypeVisitable};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_infer` due to previous error
Build completed unsuccessfully in 0:02:38
