plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0308]: mismatched types
   --> compiler/rustc_ty_utils/src/assoc.rs:54:35
    |
54  |     match tcx.hir().get_by_def_id(parent_def_id) {
    |                     ------------- ^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `OwnerId`
    |                     arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:347:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:347:12
    |
347 |     pub fn get_by_def_id(self, id: LocalDefId) -> Node<'hir> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_ty_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
