plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/session_diagnostic.rs:30:56
   |
30 |             DeserializeRlinkErrorSub::WrongFileType => fluent::codegen_ssa::wrong_file_type,
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found enum `DiagnosticMessage`
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/session_diagnostic.rs:32:17
   |
   |
32 |                 fluent::codegen_ssa::empty_version_number
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found enum `DiagnosticMessage`
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/session_diagnostic.rs:35:17
   |
   |
35 |                 fluent::codegen_ssa::encoding_version_mismatch
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found enum `DiagnosticMessage`
error[E0308]: mismatched types
  --> compiler/rustc_codegen_ssa/src/session_diagnostic.rs:38:17
   |
   |
38 |                 fluent::codegen_ssa::rustc_version_mismatch
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found enum `DiagnosticMessage`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_ssa` due to 4 previous errors
