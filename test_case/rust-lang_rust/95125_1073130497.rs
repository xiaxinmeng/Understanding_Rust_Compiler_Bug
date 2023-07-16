plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0308]: mismatched types
   --> compiler/rustc_const_eval/src/transform/check_consts/qualifs.rs:307:41
    |
307 |             if Q::in_adt_inherently(cx, def, substs) {
    |                                         ^^^ expected struct `AdtDef`, found `&AdtDef<'_>`
help: consider dereferencing the borrow
    |
    |
307 |             if Q::in_adt_inherently(cx, *def, substs) {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
