plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1596 |         sess.err(&format!(
     |
note: the lint level is defined here
    --> compiler/rustc_session/src/lib.rs:13:9
     |
     |
13   | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
     |
     |
1596 |         sess.err(&format!(
     |
note: the lint level is defined here
    --> compiler/rustc_session/src/lib.rs:12:9
     |
     |
12   | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_session` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to 2 previous errors
