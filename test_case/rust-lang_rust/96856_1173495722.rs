plain
    |                                 ^^^^^^^
    |
help: a local variable with a similar name exists
    |
369 |         self.super_place(place, cntxt, location);
help: consider importing this constant
    |
3   | use rustc_span::sym::context;
    |
    |

error[E0599]: no method named `subst` found for struct `rustc_middle::ty::Ty` in the current scope
   --> compiler/rustc_const_eval/src/transform/validate.rs:281:50
    |
281 |                         self.tcx.type_of(def_id).subst(self.tcx, substs).kind()

error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/validate.rs:310:21
    |
    |
286 |                 match kind {
    |                       ---- this expression has type `rustc_type_ir::TyKind<_>`
...
310 |                     &ty::Generator(def_id, substs, _) => {
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_type_ir::TyKind`, found reference
    = note:   expected enum `rustc_type_ir::TyKind<_>`
            found reference `&_`

    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: unused import: `rustc_middle::ty::subst::Subst`
  --> compiler/rustc_const_eval/src/transform/validate.rs:15:5
   |
15 | use rustc_middle::ty::subst::Subst;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
Some errors have detailed explanations: E0308, E0425, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_const_eval` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
