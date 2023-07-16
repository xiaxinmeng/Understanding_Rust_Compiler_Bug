plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `references_error` found for reference `&TyS<'_>` in the current scope
     |
     |
1782 |                     if !ret_ty.references_error() {
     |                                ^^^^^^^^^^^^^^^^ method not found in `&TyS<'_>`
    ::: /checkout/compiler/rustc_middle/src/ty/fold.rs:93:8
     |
93   |     fn references_error(&self) -> bool {
93   |     fn references_error(&self) -> bool {
     |        ---------------- the method is available for `&TyS<'_>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
17   | use crate::rustc_middle::ty::TypeFoldable;
