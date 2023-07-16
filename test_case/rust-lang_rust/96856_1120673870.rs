plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `this` in this scope
   --> compiler/rustc_const_eval/src/transform/validate.rs:297:25
    |
297 |                         this.fail(location, format!("No generator layout for {:?}", parent_ty));
    |                         |
    |                         not found in this scope
    |                         not found in this scope
    |                         help: you might have meant to use `self` here instead
error[E0425]: cannot find value `this` in this scope
   --> compiler/rustc_const_eval/src/transform/validate.rs:307:25
    |
    |
307 |                         this.fail(location, format!("Out of bounds local {:?} for {:?}", local, parent_ty));
    |                         |
    |                         not found in this scope
    |                         not found in this scope
    |                         help: you might have meant to use `self` here instead
error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/validate.rs:296:66
    |
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
Some errors have detailed explanations: E0308, E0425.
