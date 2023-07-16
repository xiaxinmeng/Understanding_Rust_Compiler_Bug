plain
[RUSTC-TIMING] rustc_llvm test:false 0.224
error: unused import: `TypeVisitable`
 --> compiler/rustc_infer/src/infer/equate.rs:7:31
  |
7 | use rustc_middle::ty::{TyVar, TypeVisitable};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
[RUSTC-TIMING] rustc_infer test:false 3.908
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
