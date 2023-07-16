plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `resolve_primitive` found for mutable reference `&mut LinkCollector<'a, 'tcx>` in the current scope
    |
    |
514 |         self.resolve_primitive(&path_root, TypeNS)
    |              ^^^^^^^^^^^^^^^^^ method not found in `&mut LinkCollector<'a, 'tcx>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
