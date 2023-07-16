plain
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0609]: no field `extra_ub_checks` on type `rustc_session::config::Options`
   |
   |
17 |         sess.opts.extra_ub_checks.unwrap_or(sess.opts.debug_assertions)
   |
   = note: available fields are: `crate_types`, `optimize`, `debug_assertions`, `debuginfo`, `lint_opts` ... and 31 others
help: one of the expressions' fields has a field of the same name
   |
   |
17 |         sess.opts.cg.extra_ub_checks.unwrap_or(sess.opts.debug_assertions)

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_mir_transform` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
