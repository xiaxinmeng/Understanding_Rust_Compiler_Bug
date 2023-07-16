plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0277]: can't compare `rustc_hir::PolyTraitRef<'_>` with `rustc_hir::PolyTraitRef<'_>`
     |
     |
165  |             if bounds_vec.contains(&bound) {
     |                           -------- ^^^^^^ no implementation for `rustc_hir::PolyTraitRef<'_> == rustc_hir::PolyTraitRef<'_>`
     |                           required by a bound introduced by this call
     |
     |
     = help: the trait `std::cmp::PartialEq` is not implemented for `rustc_hir::PolyTraitRef<'_>`
     = note: required because of the requirements on the impl of `std::cmp::PartialEq` for `&rustc_hir::PolyTraitRef<'_>`
note: required by a bound in `core::slice::<impl [T]>::contains`
     |
     |
2131 |         T: PartialEq,
     |            ^^^^^^^^^ required by this bound in `core::slice::<impl [T]>::contains`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
