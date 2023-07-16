plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0609]: no field `span` on type `Vec<NestedMetaItem>`
    |
    |
373 |                     sd.struct_span_err(k.span, r#"ignore with message should #[ignore = "message"]"#)

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_builtin_macros` due to previous error
warning: build failed, waiting for other jobs to finish...
