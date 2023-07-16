plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `OpaqueType(_, _, _)` not covered
   --> src/librustdoc/clean/mod.rs:278:15
    |
278 |         match bound_predicate.skip_binder() {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `OpaqueType(_, _, _)` not covered
   ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:615:5
    |
    |
615 |     OpaqueType(Ty<'tcx>, Ty<'tcx>, bool),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `PredicateKind`

