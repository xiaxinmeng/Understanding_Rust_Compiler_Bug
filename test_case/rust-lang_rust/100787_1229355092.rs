plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
365 |                 let mut err = sess.struct_fatal(&format!(
    |
note: the lint level is defined here
   --> compiler/rustc_driver/src/lib.rs:13:9
    |
    |
13  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
365 |                 let mut err = sess.struct_fatal(&format!(
    |
note: the lint level is defined here
   --> compiler/rustc_driver/src/lib.rs:12:9
    |
    |
12  | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_driver` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_driver` due to 2 previous errors
