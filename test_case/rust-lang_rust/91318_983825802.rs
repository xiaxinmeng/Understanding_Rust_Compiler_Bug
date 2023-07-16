plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0053]: method `fold_ty` has an incompatible type for trait
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:207:44
    |
207 |     fn fold_ty(&mut self, ty: Ty<'tcx>) -> Result<Ty<'tcx>, Self::Error> {
    |                                            |
    |                                            |
    |                                            expected `&TyS<'tcx>`, found enum `Result`
    |                                            help: change the output type to match the trait: `&'tcx TyS<'tcx>`
   ::: compiler/rustc_middle/src/ty/fold.rs:236:43
    |
    |
236 |     fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx>
    |                                           -------- type in trait
    |
    = note: expected fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, &'tcx TyS<'tcx>) -> &'tcx TyS<'tcx>`
               found fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, &'tcx TyS<'tcx>) -> Result<&'tcx TyS<'tcx>, !>`
error[E0053]: method `fold_const` has an incompatible type for trait
error[E0053]: method `fold_const` has an incompatible type for trait
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:217:10
    |
217 |     ) -> Result<&'tcx ty::Const<'tcx>, Self::Error> {
    |          |
    |          |
    |          expected `&ty::consts::Const<'tcx>`, found enum `Result`
    |          help: change the output type to match the trait: `&'tcx ty::consts::Const<'tcx>`
   ::: compiler/rustc_middle/src/ty/fold.rs:250:59
    |
    |
250 |     fn fold_const(&mut self, c: &'tcx ty::Const<'tcx>) -> &'tcx ty::Const<'tcx>
    |                                                           --------------------- type in trait
    |
    = note: expected fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, &'tcx ty::consts::Const<'tcx>) -> &'tcx ty::consts::Const<'tcx>`
               found fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, &'tcx ty::consts::Const<'tcx>) -> Result<&'tcx ty::consts::Const<'tcx>, !>`
error[E0053]: method `fold_mir_const` has an incompatible type for trait
error[E0053]: method `fold_mir_const` has an incompatible type for trait
   --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:227:10
    |
227 |     ) -> Result<mir::ConstantKind<'tcx>, Self::Error> {
    |          |
    |          |
    |          expected enum `mir::ConstantKind`, found enum `Result`
    |          help: change the output type to match the trait: `mir::ConstantKind<'tcx>`
   ::: compiler/rustc_middle/src/ty/fold.rs:264:65
    |
    |
264 |     fn fold_mir_const(&mut self, c: mir::ConstantKind<'tcx>) -> mir::ConstantKind<'tcx>
    |                                                                 ----------------------- type in trait
    |
    = note: expected fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, mir::ConstantKind<'_>) -> mir::ConstantKind<'_>`
               found fn pointer `fn(&mut TryNormalizeAfterErasingRegionsFolder<'tcx>, mir::ConstantKind<'_>) -> Result<mir::ConstantKind<'_>, !>`

error[E0271]: type mismatch resolving `<TryNormalizeAfterErasingRegionsFolder<'_> as TypeFolder<'tcx>>::Error == !`
  --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:91:19
   |
91 |             value.fold_with(&mut folder)
   |                   ^^^^^^^^^ expected enum `NormalizationError`, found `!`
   |
   = note: expected enum `NormalizationError<'_>`
              found type `!`
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:91:13
   |
67 |     pub fn try_normalize_erasing_regions<T>(
   |                                          - this type parameter
71 |     ) -> Result<T, NormalizationError<'tcx>>
   |          ----------------------------------- expected `Result<T, NormalizationError<'tcx>>` because of return type
...
...
91 |             value.fold_with(&mut folder)
   |             |
   |             expected enum `Result`, found type parameter `T`
   |             expected enum `Result`, found type parameter `T`
   |             help: try using a variant of the expected enum: `Ok(value.fold_with(&mut folder))`
   = note:        expected enum `Result<T, NormalizationError<'tcx>>`
           found type parameter `T`

Some errors have detailed explanations: E0053, E0271, E0308.
