plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: unused import: `self`
  --> compiler/rustc_codegen_ssa/src/base.rs:22:36
   |
22 | use rustc_middle::middle::cstore::{self, EncodedMetadata};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to previous error

error: could not compile `rustc_codegen_ssa`
