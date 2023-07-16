
error[E0004]: non-exhaustive patterns: `Ty(_)`, `TraitRef(_)`, `Binding(_)` and 8 more not covered
   --> compiler/rustc_privacy/src/lib.rs:254:25
    |
254 |     let hir_vis = match tcx.hir().get(hir_id) {
    |                         ^^^^^^^^^^^^^^^^^^^^^ patterns `Ty(_)`, `TraitRef(_)`, `Binding(_)` and 8 more not covered
