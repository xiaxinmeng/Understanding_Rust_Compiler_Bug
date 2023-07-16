plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: `#[error(...)]` is not a valid attribute
  |
  |
7 | #[error(session::incorrect_cgu_reuse_type)]
  |
  |
  = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  |
  |
7 | #[error(session::incorrect_cgu_reuse_type)]
  |
  |
  = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
 --> compiler/rustc_session/src/errors.rs:7:3
  |
  |
7 | #[error(session::incorrect_cgu_reuse_type)]

error: unused import: `crate as rustc_session`
 --> compiler/rustc_session/src/errors.rs:1:5
  |
  |
1 | use crate as rustc_session;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_session` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to 4 previous errors
Build completed unsuccessfully in 0:01:55
