plain
    Checking clippy_lints v0.1.66 (/checkout/src/tools/clippy/clippy_lints)
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> src/tools/clippy/clippy_lints/src/transmute/utils.rs:56:28
    |
56  |           if let Ok(check) = cast::CastCheck::new(
    |  ____________________________^^^^^^^^^^^^^^^^^^^^-
57  | |             &fn_ctxt, e, from_ty, to_ty,
58  | |             // We won't show any error to the user, so we don't care what the span is here.
59  | |             DUMMY_SP, DUMMY_SP,
60  | |         ) {
    | |_________- an argument of type `Constness` is missing
note: associated function defined here
   --> /checkout/compiler/rustc_hir_typeck/src/cast.rs:208:12
    |
208 |     pub fn new(
208 |     pub fn new(
    |            ^^^
help: provide the argument
    |
56  |         if let Ok(check) = cast::CastCheck::new(&fn_ctxt, e, from_ty, to_ty, DUMMY_SP, DUMMY_SP, /* Constness */) {

For more information about this error, try `rustc --explain E0061`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:14
