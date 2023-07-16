plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: cannot find attribute `instrument` in this scope
    --> compiler/rustc_ast_lowering/src/lib.rs:1385:7
     |
1385 |     #[instrument(level = "debug", skip(self))]
     |
     = note: consider importing this attribute macro:
             tracing::instrument

