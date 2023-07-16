plain
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `PredicateKind::Ambiguous` not covered
   --> src/librustdoc/clean/mod.rs:305:11
    |
305 |     match bound_predicate.skip_binder() {
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `PredicateKind::Ambiguous` not covered
    |
note: `PredicateKind` defined here
    |
651 | pub enum PredicateKind<'tcx> {
    | ----------------------------
...
...
708 |     Ambiguous,
    |     ^^^^^^^^^ not covered
    = note: the matched value is of type `PredicateKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
322 ~         | ty::PredicateKind::TypeWellFormedFromEnv(..) => panic!("not user writable"),
323 ~         PredicateKind::Ambiguous => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:21
