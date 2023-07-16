rust
let err = match synthetic_kind {
  hir::SyntheticKind::ImplTrait => {
    struct_span_err!(
      self.tcx.sess,
      span,
      EXXX, // we'll have to add a new error code...
      "cannot provide explicit type parameters when `impl Trait` is used in argument position")
  }
};
// bonus (described below) would go here
err.emit();
