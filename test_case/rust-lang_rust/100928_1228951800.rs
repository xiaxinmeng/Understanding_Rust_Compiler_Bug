plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0599]: no method named `set_code` found for struct `DiagnosticBuilder<'_, ErrorGuaranteed>` in the current scope
    |
    |
432 |         diag.set_code(error_code!(E0465));
    |              ^^^^^^^^ help: there is an associated function with a similar name: `code`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to previous error
