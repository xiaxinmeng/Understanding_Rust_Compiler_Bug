plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 1 field
   --> src/librustdoc/clean/utils.rs:299:9
    |
299 |         ty::ConstKind::Unevaluated(def, _, promoted) => {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 1 field, found 3
   ::: /checkout/compiler/rustc_middle/src/ty/consts/kind.rs:39:5
    |
    |
39  |     Unevaluated(Unevaluated<'tcx>),
    |     ------------------------------ tuple variant defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustdoc`
