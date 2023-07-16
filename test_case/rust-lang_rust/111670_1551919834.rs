plain
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0741]: `Level` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter
    |
    |
137 |     pub(crate) fn new_guaranteeing_error<M: Into<DiagnosticMessage>, const L: Level>(


error[E0741]: `Level` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter
    |
    |
123 |     pub(crate) trait IsError<const L: Level> {}

For more information about this error, try `rustc --explain E0741`.
error: could not compile `rustc_errors` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
