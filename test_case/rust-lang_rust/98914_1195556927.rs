plain
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0433]: failed to resolve: use of undeclared crate or module `sym`
   --> src/tools/clippy/clippy_lints/src/inherent_to_string.rs:100:40
    |
100 |             if impl_item.ident.name == sym::to_string;
    |                                        ^^^ use of undeclared crate or module `sym`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:07
