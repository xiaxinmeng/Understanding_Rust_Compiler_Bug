plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
   --> compiler/rustc_mir_transform/src/check_unsafety.rs:629:39
    |
629 |         .body(tcx.hir().body_owned_by(tcx.hir().local_def_id_to_hir_id(def_id)))
    |                         ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                         arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
    |
429 |     pub fn body_owned_by(self, id: LocalDefId) -> BodyId {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
