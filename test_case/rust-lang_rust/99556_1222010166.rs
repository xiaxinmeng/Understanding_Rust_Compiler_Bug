plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: `#[error(...)]` is not a valid attribute
    |
    |
654 | #[error(passes::collapse_debuginfo)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
    |
    |
654 | #[error(passes::collapse_debuginfo)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
   --> compiler/rustc_passes/src/errors.rs:654:3
    |
    |
654 | #[error(passes::collapse_debuginfo)]
    |
note: `error` is imported here, but it is a function-like macro
   --> compiler/rustc_passes/src/lib.rs:18:1
    |
    |
18  | #[macro_use]
    | ^^^^^^^^^^^^

error[E0277]: the trait bound `CollapseDebuginfo: SessionDiagnostic<'_>` is not satisfied
    |
    |
421 |                     .emit_err(errors::CollapseDebuginfo { attr_span: attr.span, defn_span: span });
    |                      -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `CollapseDebuginfo`
    |                      required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] rustc_passes test:false 0.812
error: could not compile `rustc_passes` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
