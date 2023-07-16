plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused import: `Pat`
 --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:8:75
  |
8 | use rustc_hir::{Body, Expr, ExprKind, FnRetTy, HirId, Local, LocalSource, Pat};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `map::Map`
 --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:9:25
  |
  |
9 | use rustc_middle::hir::{map::Map, nested_filter};


error: unused imports: `TypeFoldable`, `TypeVisitor`
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:14:36
   |
14 | use rustc_middle::ty::{Ty, TyCtxt, TypeFoldable, TypeVisitor};

error: unused import: `rustc_span::source_map::DesugaringKind`
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:15:5
   |
---
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unused variable: `impl_candidates`
   --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:285:9
    |
285 |         impl_candidates: Vec<ty::TraitRef<'tcx>>,
    |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_impl_candidates`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: field is never read: `def_id`
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:61:5
   |
61 |     def_id: DefId,
61 |     def_id: DefId,
   |     ^^^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
error: could not compile `rustc_infer` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:28
