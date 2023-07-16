plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error: unused import: `CodeModel`
 --> compiler/rustc_target/src/spec/x86_64_unknown_uefi.rs:8:19
  |
8 | use crate::spec::{CodeModel, Target};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
error: aborting due to previous error
