plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused import: `Const`
  --> compiler/rustc_infer/src/infer/canonical/query_response.rs:29:40
   |
29 | use rustc_middle::ty::{self, BoundVar, Const, ToPredicate, Ty, TyCtxt};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_infer` due to previous error
