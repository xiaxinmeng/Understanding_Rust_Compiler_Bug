`

   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
error[E0532]: expected tuple struct or tuple variant, found unit variant `ty::Opaque`
   --> compiler/rustc_hir_analysis/src/check/compare_method.rs:339:13
    |
339 |             ty::Opaque(..) => {
    |             ^^^^^^^^^^^^^^
    |
   ::: /checkout/compiler/rustc_type_ir/src/sty.rs:42:5
    |
42  |     Opaque,
    |     ------ `ty::Opaque` defined here
    |
help: use this syntax instead
    |
339 |             ty::Opaque => {
    |             ~~~~~~~~~~
help: consider importing one of these items instead
    |
1   | use crate::check::compare_method::infer::outlives::components::Component::Opaque;
    |
1   | use crate::check::compare_method::infer::region_constraints::GenericKind::Opaque;
    |
1   | use rustc_infer::infer::outlives::components::Component::Opaque;
    |
1   | use rustc_infer::infer::region_constraints::GenericKind::Opaque;
    |
      and 2 other candidates
help: if you import `Opaque`, refer to it directly
    |
339 -             ty::Opaque(..) => {
339 +             Opaque(..) => {
    |

For more information about this erro
