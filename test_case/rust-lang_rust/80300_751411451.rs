plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: use of undeclared crate or module `ast`
   --> src/librustdoc/visit_ast.rs:363:50
    |
363 |     fn check_item_attrs(&mut self, attrs: &'tcx [ast::Attribute], item: AttrReceiverKind) {
    |                                                  ^^^ use of undeclared crate or module `ast`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustdoc`
