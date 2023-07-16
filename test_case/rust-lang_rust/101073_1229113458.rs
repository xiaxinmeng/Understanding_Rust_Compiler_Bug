plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
578 |             session.warn(&format!(
    |
note: the lint level is defined here
   --> compiler/rustc_interface/src/lib.rs:9:9
    |
    |
9   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
    |
    |
578 |             session.warn(&format!(
    |
note: the lint level is defined here
   --> compiler/rustc_interface/src/lib.rs:8:9
    |
    |
8   | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
627 |                 sess.warn(
    |                      ^^^^


error: diagnostics should be created using translatable messages
    |
627 |                 sess.warn(
    |                      ^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
634 |                     sess.warn("ignoring -C extra-filename flag due to -o flag");


error: diagnostics should be created using translatable messages
    |
    |
634 |                     sess.warn("ignoring -C extra-filename flag due to -o flag");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
639 |                 sess.warn("ignoring --out-dir flag due to -o flag");


error: diagnostics should be created using translatable messages
    |
    |
639 |                 sess.warn("ignoring --out-dir flag due to -o flag");

error: could not compile `rustc_interface` due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_interface` due to 8 previous errors
