plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
857 |                 err.span_note(multi_span, "the used `impl` has a `'static` requirement");
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:2:9
    |
    |
2   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
857 |                 err.span_note(multi_span, "the used `impl` has a `'static` requirement");
    |
note: the lint level is defined here
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:1:9
    |
    |
1   | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 2 previous errors
