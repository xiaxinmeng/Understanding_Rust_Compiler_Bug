plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `visit_local_macro` found for struct `RustdocVisitor<'a, 'tcx>` in the current scope
  --> src/librustdoc/visit_ast.rs:77:39
   |
35 | crate struct RustdocVisitor<'a, 'tcx> {
   | ------------------------------------- method `visit_local_macro` not found for this
...
77 |             let visit_macro = || self.visit_local_macro(def, None);
   |                                       ^^^^^^^^^^^^^^^^^ method not found in `RustdocVisitor<'a, 'tcx>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
