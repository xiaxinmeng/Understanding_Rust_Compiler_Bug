plain
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0433]: failed to resolve: use of undeclared crate or module `iter`
   --> src/tools/clippy/clippy_lints/src/default_numeric_fallback.rs:111:42
    |
111 |                     for (expr, bound) in iter::zip(args, fn_sig.skip_binder().inputs()) {
    |                                          ^^^^ use of undeclared crate or module `iter`
error[E0433]: failed to resolve: use of undeclared crate or module `iter`
   --> src/tools/clippy/clippy_lints/src/default_numeric_fallback.rs:124:42
    |
    |
124 |                     for (expr, bound) in iter::zip(args, fn_sig.inputs()) {
    |                                          ^^^^ use of undeclared crate or module `iter`
error[E0433]: failed to resolve: use of undeclared crate or module `iter`
   --> src/tools/clippy/clippy_lints/src/literal_representation.rs:351:13
    |
    |
351 |             iter::zip(&UUID_GROUP_LENS, &group_sizes).all(|(&a, &b)| a == b)
    |             ^^^^ use of undeclared crate or module `iter`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `clippy_lints`
