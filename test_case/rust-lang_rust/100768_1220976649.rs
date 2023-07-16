plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: `#[fatal(...)]` is not a valid attribute
  |
  |
7 | #[fatal(plugin_impl::load_plugin_error)]
  |
  |
  = help: only `error`, `warning`, `help`, `note` and `warn_` are valid attributes

error: diagnostic kind not specified
  |
  |
7 | #[fatal(plugin_impl::load_plugin_error)]
  |
  |
  = help: use the `#[error(...)]` attribute to create an error

error: cannot find attribute `fatal` in this scope
  |
  |
7 | #[fatal(plugin_impl::load_plugin_error)]

error[E0599]: no method named `emit_fatal` found for reference `&Session` in the current scope
  --> compiler/rustc_plugin_impl/src/load.rs:57:14
   |
   |
57 |         sess.emit_fatal(LoadPluginError { span: ident.span, msg: err.to_string() });
   |              ^^^^^^^^^^ help: there is an associated function with a similar name: `fatal`
error[E0308]: mismatched types
  --> compiler/rustc_plugin_impl/src/load.rs:54:58
   |
   |
54 |       let fun = dylink_registrar(lib).unwrap_or_else(|err| {
   |  __________________________________________________________^
55 | |         // This is fatal: there are almost certainly macros we need inside this crate, so
56 | |         // continuing would spew "macro undefined" errors.
57 | |         sess.emit_fatal(LoadPluginError { span: ident.span, msg: err.to_string() });
   | |_____^ expected fn pointer, found `()`
   |
   |
   = note: expected fn pointer `for<'r, 's> fn(&'r mut Registry<'s>)`

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_plugin_impl` due to 5 previous errors
