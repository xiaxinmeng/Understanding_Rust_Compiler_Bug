plain
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: unused import: `VariantData`
 --> compiler/rustc_ast_passes/src/feature_gate.rs:4:36
  |
4 | use rustc_ast::{PatKind, RangeEnd, VariantData};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `feature_err`
 --> compiler/rustc_ast_passes/src/feature_gate.rs:8:28
  |
  |
8 | use rustc_session::parse::{feature_err, feature_err_issue};

error: could not compile `rustc_ast_passes` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
