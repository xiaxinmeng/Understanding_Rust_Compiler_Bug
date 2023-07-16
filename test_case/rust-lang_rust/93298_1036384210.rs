plain
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0433]: failed to resolve: use of undeclared type `StripReferences`
   --> compiler/rustc_middle/src/ty/trait_def.rs:154:75
    |
154 |             fast_reject::simplify_type(self, self_ty, SimplifyParams::No, StripReferences::No)
    |                                                                           ^^^^^^^^^^^^^^^ use of undeclared type `StripReferences`
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
   --> compiler/rustc_middle/src/ty/trait_def.rs:154:13
    |
    |
154 |             fast_reject::simplify_type(self, self_ty, SimplifyParams::No, StripReferences::No)
    |             |
    |             expected 3 arguments
    |
note: function defined here
note: function defined here
   --> compiler/rustc_middle/src/ty/fast_reject.rs:78:8
    |
78  | pub fn simplify_type(
    |        ^^^^^^^^^^^^^
79  |     tcx: TyCtxt<'_>,
80  |     ty: Ty<'_>,
    |     ----------
81  |     can_simplify_params: SimplifyParams,
    |     -----------------------------------
