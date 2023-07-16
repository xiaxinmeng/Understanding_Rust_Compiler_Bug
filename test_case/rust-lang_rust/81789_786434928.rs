plain
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/transmute.rs:744:26
    |
744 |             &fn_ctxt, e, from_ty, to_ty,
    |                          ^^^^^^^ expected struct `rustc_hir::Expr`, found struct `TyS`
    |
    = note: expected reference `&rustc_hir::Expr<'_>`
               found reference `&'tcx TyS<'tcx>`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/transmute.rs:746:13
    |
    |
746 |             DUMMY_SP, DUMMY_SP,
    |             ^^^^^^^^ expected `&TyS<'_>`, found struct `rustc_span::Span`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints`
