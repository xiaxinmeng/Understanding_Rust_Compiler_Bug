plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0603]: struct `LintDiagnosticBuilder` is private
 --> compiler/rustc_lint/src/let_underscore.rs:4:26
  |
4 | use rustc_middle::{lint::LintDiagnosticBuilder, ty};
  |                          ^^^^^^^^^^^^^^^^^^^^^ private struct
note: the struct `LintDiagnosticBuilder` is defined here
 --> /checkout/compiler/rustc_middle/src/lint.rs:5:46
  |
  |
5 | use rustc_errors::{Diagnostic, DiagnosticId, LintDiagnosticBuilder, MultiSpan};

For more information about this error, try `rustc --explain E0603`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
