plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused import: `fold::TypeFoldable`
 --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:3:24
  |
3 | use rustc_middle::ty::{fold::TypeFoldable, Ty};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `needs_subst` found for reference `&TyS<'_>` in the current scope
  --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:41:24
   |
41 |     !t.is_enum() && !t.needs_subst()
   |                        ^^^^^^^^^^^ method not found in `&TyS<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_lint` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
