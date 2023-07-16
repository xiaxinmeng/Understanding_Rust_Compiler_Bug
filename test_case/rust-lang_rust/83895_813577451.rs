plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `register_plugins` found for reference `&rustc_interface::interface::Compiler` in the current scope
    |
    |
656 |             let (_, lint_store) = &*compiler.register_plugins()?.peek();
    |                                              ^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `register_lints`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
