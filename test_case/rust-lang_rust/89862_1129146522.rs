plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0599]: no method named `subst` found for struct `rustc_middle::ty::Ty` in the current scope
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:802:64
    |
802 |                     let parent_ty = tcx.type_of(parent_def_id).subst(tcx, full_substs);

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unused import: `Subst`
error: unused import: `Subst`
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:12:59
   |
12 | use rustc_middle::ty::subst::{GenericArg, GenericArgKind, Subst, SubstsRef};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_infer` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_infer` due to 2 previous errors
