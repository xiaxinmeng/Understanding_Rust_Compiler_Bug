plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0599]: no method named `rust_2021` found for enum `Edition` in the current scope
    --> compiler/rustc_expand/src/mbe/macro_rules.rs:1028:49
     |
1028 | ...                   && sess.edition.rust_2021()

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_expand` due to previous error
warning: build failed, waiting for other jobs to finish...
