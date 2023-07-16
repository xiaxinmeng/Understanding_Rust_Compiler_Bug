plain
    Checking clippy_lints v0.1.54 (/checkout/src/tools/clippy/clippy_lints)
error[E0061]: this function takes 7 arguments but 6 arguments were supplied
   --> src/tools/clippy/clippy_lints/src/transmute/utils.rs:86:28
    |
86  |         if let Ok(check) = CastCheck::new(
    |                            ^^^^^^^^^^^^^^ expected 7 arguments
87  |             &fn_ctxt, e, from_ty, to_ty,
    |             --------  -  -------  -----
88  |             // We won't show any error to the user, so we don't care what the span is here.
89  |             DUMMY_SP, DUMMY_SP,
    |
note: associated function defined here
   --> /checkout/compiler/rustc_typeck/src/check/cast.rs:194:12
    |
