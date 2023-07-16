plain
[RUSTC-TIMING] rustc_errors test:false 7.938
[RUSTC-TIMING] rustc_attr test:false 4.354
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
[RUSTC-TIMING] rustc_session test:false 7.623
error[E0599]: no method named `needs_normalization` found for struct `ty::Ty` in the current scope
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:213:25
    |
213 |         let ty = if !ty.needs_normalization(self.param_env.reveal()) {
    |
   ::: compiler/rustc_middle/src/ty/mod.rs:494:1
    |
    |
494 | pub struct Ty<'tcx>(Interned<'tcx, WithStableHash<TyS<'tcx>>>);
    | ------------------- method `needs_normalization` not found for this struct

error[E0599]: no method named `needs_normalization` found for struct `ty::consts::Const` in the current scope
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:226:23
    |
226 |         let c = if !c.needs_normalization(self.param_env.reveal()) {
    |                       ^^^^^^^^^^^^^^^^^^^ method not found in `ty::consts::Const<'tcx>`
   ::: compiler/rustc_middle/src/ty/consts.rs:22:1
    |
    |
22  | pub struct Const<'tcx>(pub Interned<'tcx, ConstS<'tcx>>);
    | ---------------------- method `needs_normalization` not found for this struct
error[E0599]: no method named `needs_normalization` found for struct `ty::Ty` in the current scope
error[E0599]: no method named `needs_normalization` found for struct `ty::Ty` in the current scope
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:269:25
    |
269 |         let ty = if !ty.needs_normalization(self.param_env.reveal()) {
    |
   ::: compiler/rustc_middle/src/ty/mod.rs:494:1
    |
    |
494 | pub struct Ty<'tcx>(Interned<'tcx, WithStableHash<TyS<'tcx>>>);
    | ------------------- method `needs_normalization` not found for this struct

error[E0599]: no method named `needs_normalization` found for struct `ty::consts::Const` in the current scope
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:285:23
    |
285 |         let c = if !c.needs_normalization(self.param_env.reveal()) {
    |                       ^^^^^^^^^^^^^^^^^^^ method not found in `ty::consts::Const<'tcx>`
   ::: compiler/rustc_middle/src/ty/consts.rs:22:1
    |
    |
22  | pub struct Const<'tcx>(pub Interned<'tcx, ConstS<'tcx>>);
    | ---------------------- method `needs_normalization` not found for this struct
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
[RUSTC-TIMING] rustc_ast_passes test:false 5.362
error: unused import: `super::TypeVisitable`
error: unused import: `super::TypeVisitable`
  --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:15:5
15 | use super::TypeVisitable;
   |     ^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_middle test:false 9.618
error: could not compile `rustc_middle` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
