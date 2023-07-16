plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `suggestion` in module `crate::fluent_generated`
  --> compiler/rustc_lint/src/lints.rs:64:24
   |
64 | #[multipart_suggestion(suggestion, applicability = "machine-applicable")]
   |                        ^^^^^^^^^^ not found in `crate::fluent_generated`
help: consider importing one of these items
   |
3  | use crate::fluent_generated::_subdiag::suggestion;
   |
