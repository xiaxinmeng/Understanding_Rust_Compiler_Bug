plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error[E0061]: this method takes 4 arguments but 2 arguments were supplied
   --> compiler/rustc_hir_typeck/src/method/suggest.rs:380:24
    |
380 |             err = self.missing_writer(rcvr_ty, args).unwrap();
    |                                      ||
    |                                      ||
    |                                      |an argument of type `rustc_span::Span` is missing
    |                                      an argument of type `rustc_span::symbol::Ident` is missing
note: method defined here
   --> compiler/rustc_hir_typeck/src/method/suggest.rs:248:8
    |
248 |     fn missing_writer(
248 |     fn missing_writer(
    |        ^^^^^^^^^^^^^^
249 |         &self,
250 |         _sugg_span: Span,
    |         ----------------
251 |         rcvr_ty: Ty<'tcx>,
252 |         _item_name: Ident,
    |         -----------------
    |         -----------------
253 |         args: Option<(&'tcx hir::Expr<'tcx>, &'tcx [hir::Expr<'tcx>])>,
help: provide the arguments
    |
    |
380 |             err = self.missing_writer(/* rustc_span::Span */, rcvr_ty, /* rustc_span::symbol::Ident */, args).unwrap();

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
