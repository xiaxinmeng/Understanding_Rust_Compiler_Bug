plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/utils.rs:239:69
    |
239 |                 print_const_expr(cx.tcx, cx.tcx.hir().body_owned_by(hir_id))
    |                                                       ------------- ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                                                       arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
    |
429 |     pub fn body_owned_by(self, id: LocalDefId) -> BodyId {

error[E0308]: mismatched types
   --> src/librustdoc/core.rs:316:55
    |
    |
316 |                 let body = hir.body(hir.body_owned_by(hir.local_def_id_to_hir_id(def_id)));
    |                                         ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                                         arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:429:12
    |
429 |     pub fn body_owned_by(self, id: LocalDefId) -> BodyId {

error[E0308]: mismatched types
   --> src/librustdoc/scrape_examples.rs:147:36
    |
    |
147 |         if hir.maybe_body_owned_by(owner).is_none() {
    |                ------------------- ^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:424:12
   --> /checkout/compiler/rustc_middle/src/hir/map/mod.rs:424:12
    |
424 |     pub fn maybe_body_owned_by(self, id: LocalDefId) -> Option<BodyId> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:02:48
