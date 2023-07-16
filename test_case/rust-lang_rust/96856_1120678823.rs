plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/validate.rs:296:66
    |
296 |                     let Some(layout) = self.tcx.generator_layout(def_id) else {
    |                                                                  ^^^^^^ expected struct `DefId`, found `&DefId`
help: consider dereferencing the borrow
    |
    |
296 |                     let Some(layout) = self.tcx.generator_layout(*def_id) else {

error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/validate.rs:306:59
    |
    |
306 |                     let Some(f_ty) = layout.field_tys.get(local) else {
    |                                                           ^^^^^ expected struct `GeneratorSavedLocal`, found `&GeneratorSavedLocal`
help: consider dereferencing the borrow
    |
    |
306 |                     let Some(f_ty) = layout.field_tys.get(*local) else {

error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/validate.rs:311:49
    |
    |
311 |                     check_equal(self, location, f_ty);
    |                                                 ^^^^ expected struct `rustc_middle::ty::Ty`, found `&rustc_middle::ty::Ty<'_>`
help: consider dereferencing the borrow
    |
    |
311 |                     check_equal(self, location, *f_ty);

    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
For more information about this error, try `rustc --explain E0308`.
