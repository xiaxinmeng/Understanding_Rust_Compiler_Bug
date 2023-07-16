plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
380 |                     .struct_span_err(span, "non-defining opaque type use in defining scope")
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/region_infer/mod.rs:2:9
    |
    |
2   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
380 |                     .struct_span_err(span, "non-defining opaque type use in defining scope")
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/region_infer/mod.rs:1:9
    |
    |
1   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
402 |                 .struct_span_err(span, "non-defining opaque type use in defining scope")


error: diagnostics should be created using translatable messages
    |
    |
402 |                 .struct_span_err(span, "non-defining opaque type use in defining scope")


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
424 |                 .struct_span_err(span, "non-defining opaque type use in defining scope")


error: diagnostics should be created using translatable messages
    |
    |
424 |                 .struct_span_err(span, "non-defining opaque type use in defining scope")

error: could not compile `rustc_borrowck` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 6 previous errors
