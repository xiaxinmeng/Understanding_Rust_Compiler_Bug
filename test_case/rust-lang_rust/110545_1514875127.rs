plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused import: `GenericArgKind`
  --> compiler/rustc_codegen_llvm/src/debuginfo/mod.rs:30:31
   |
30 | use rustc_middle::ty::subst::{GenericArgKind, SubstsRef};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_middle::ty::subst::GenericArgKind`
  --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:32:5
   |
32 | use rustc_middle::ty::subst::GenericArgKind;
