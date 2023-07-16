plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error[E0308]: mismatched types
   --> compiler/rustc_hir_typeck/src/callee.rs:472:59
    |
472 |             let body_def_id = self.tcx.hir().local_def_id(self.body_id);
    |                                              ------------ ^^^^^^^^^^^^ expected struct `HirId`, found struct `LocalDefId`
    |                                              arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:173:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:173:12
    |
173 |     pub fn local_def_id(self, hir_id: HirId) -> LocalDefId {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
