plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: hidden lifetime parameters in types are deprecated
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:2095:25
     |
2095 | fn find_param_in_ty(ty: Ty, param_to_point_at: ty::GenericArg) -> bool {
     |                         ^^ expected lifetime parameter
     |
     = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
help: indicate the anonymous lifetime
     |
2095 | fn find_param_in_ty(ty: Ty<'_>, param_to_point_at: ty::GenericArg) -> bool {

error: hidden lifetime parameters in types are deprecated
error: hidden lifetime parameters in types are deprecated
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:2095:52
     |
2095 | fn find_param_in_ty(ty: Ty, param_to_point_at: ty::GenericArg) -> bool {
     |                                                |
     |                                                expected lifetime parameter
     |
help: indicate the anonymous lifetime
help: indicate the anonymous lifetime
     |
2095 | fn find_param_in_ty(ty: Ty, param_to_point_at: ty::GenericArg<'_>) -> bool {

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1842:48
     |
1842 |             .filter(|(_, ty)| find_param_in_ty(ty, param_to_point_at))
     |                               ---------------- ^^ expected struct `rustc_middle::ty::Ty`, found `&&rustc_middle::ty::Ty<'_>`
     |                               arguments to this function are incorrect
     |
note: function defined here
note: function defined here
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:2095:4
     |
2095 | fn find_param_in_ty(ty: Ty, param_to_point_at: ty::GenericArg) -> bool {
help: consider dereferencing the borrow
     |
     |
1842 |             .filter(|(_, ty)| find_param_in_ty(**ty, param_to_point_at))

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1884:21
     |
1883 |                 match find_param_in_ty(field_ty, param_to_point_at) {
1884 |                     Ok(value) => value,
     |                     ^^^^^^^^^ expected `bool`, found enum `Result`
     |
     = note: expected type `bool`
     = note: expected type `bool`
                found enum `Result<_, _>`

error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:1885:21
     |
1883 |                 match find_param_in_ty(field_ty, param_to_point_at) {
1884 |                     Ok(value) => value,
1885 |                     Err(value) => return value,
     |                     ^^^^^^^^^^ expected `bool`, found enum `Result`
     |
