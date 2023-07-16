plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_session/src/cgu_reuse_tracker.rs:136:26
    |
136 |                     diag.span_fatal(error_span.0, &msg)
    |
note: the lint level is defined here
   --> compiler/rustc_session/src/lib.rs:12:9
    |
    |
12  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   --> compiler/rustc_session/src/cgu_reuse_tracker.rs:136:26
    |
136 |                     diag.span_fatal(error_span.0, &msg)
    |
note: the lint level is defined here
   --> compiler/rustc_session/src/lib.rs:11:9
    |
    |
11  | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
104 |     let mut err = sess.span_diagnostic.struct_span_err_with_code(span, explain, error_code!(E0658));


error: diagnostics should be created using translatable messages
    |
    |
104 |     let mut err = sess.span_diagnostic.struct_span_err_with_code(span, explain, error_code!(E0658));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
126 |         err.note(&format!(


error: diagnostics should be created using translatable messages
    |
    |
126 |         err.note(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
133 |         err.help(&format!("add `#![feature({feature})]` to the crate attributes to enable"));


error: diagnostics should be created using translatable messages
    |
    |
133 |         err.help(&format!("add `#![feature({feature})]` to the crate attributes to enable"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
319 |         self.span_diagnostic.struct_err(msg)


error: diagnostics should be created using translatable messages
    |
    |
319 |         self.span_diagnostic.struct_err(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
324 |         self.span_diagnostic.struct_warn(msg)


error: diagnostics should be created using translatable messages
    |
    |
324 |         self.span_diagnostic.struct_warn(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
891 |         sess.fatal(&err);


error: diagnostics should be created using translatable messages
    |
    |
891 |         sess.fatal(&err);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
232 |             let mut diag = self.struct_warn("skipping const checks");


error: diagnostics should be created using translatable messages
    |
    |
232 |             let mut diag = self.struct_warn("skipping const checks");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
247 |                 self.err(
    |                      ^^^


error: diagnostics should be created using translatable messages
    |
247 |                 self.err(
    |                      ^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
292 |         self.diagnostic().struct_span_warn(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
292 |         self.diagnostic().struct_span_warn(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
310 |         self.diagnostic().struct_span_warn_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
310 |         self.diagnostic().struct_span_warn_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
314 |         self.diagnostic().struct_warn(msg)


error: diagnostics should be created using translatable messages
    |
    |
314 |         self.diagnostic().struct_warn(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
330 |         self.diagnostic().struct_span_allow(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
330 |         self.diagnostic().struct_span_allow(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
334 |         self.diagnostic().struct_allow(msg)


error: diagnostics should be created using translatable messages
    |
    |
334 |         self.diagnostic().struct_allow(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
342 |         self.diagnostic().struct_expect(msg, id)


error: diagnostics should be created using translatable messages
    |
    |
342 |         self.diagnostic().struct_expect(msg, id)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
350 |         self.diagnostic().struct_span_err(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
350 |         self.diagnostic().struct_span_err(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
359 |         self.diagnostic().struct_span_err_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
359 |         self.diagnostic().struct_span_err_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
367 |         self.parse_sess.struct_err(msg)
    |                         ^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
367 |         self.parse_sess.struct_err(msg)
    |                         ^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
375 |         self.diagnostic().struct_err_with_code(msg, code)
    |                           ^^^^^^^^^^^^^^^^^^^^


error: diagnostics should be created using translatable messages
    |
375 |         self.diagnostic().struct_err_with_code(msg, code)
    |                           ^^^^^^^^^^^^^^^^^^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
383 |         self.diagnostic().struct_warn_with_code(msg, code)


error: diagnostics should be created using translatable messages
    |
    |
383 |         self.diagnostic().struct_warn_with_code(msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
391 |         self.diagnostic().struct_span_fatal(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
391 |         self.diagnostic().struct_span_fatal(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
400 |         self.diagnostic().struct_span_fatal_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
400 |         self.diagnostic().struct_span_fatal_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
404 |         self.diagnostic().struct_fatal(msg)


error: diagnostics should be created using translatable messages
    |
    |
404 |         self.diagnostic().struct_fatal(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
409 |         self.diagnostic().span_fatal(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
409 |         self.diagnostic().span_fatal(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
418 |         self.diagnostic().span_fatal_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
418 |         self.diagnostic().span_fatal_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
434 |             self.span_err(sp, msg);


error: diagnostics should be created using translatable messages
    |
    |
434 |             self.span_err(sp, msg);


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
443 |         self.diagnostic().span_err(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
443 |         self.diagnostic().span_err(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
452 |         self.diagnostic().span_err_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
452 |         self.diagnostic().span_err_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
520 |         self.diagnostic().span_warn(sp, msg)


error: diagnostics should be created using translatable messages
    |
    |
520 |         self.diagnostic().span_warn(sp, msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
528 |         self.diagnostic().span_warn_with_code(sp, msg, code)


error: diagnostics should be created using translatable messages
    |
    |
528 |         self.diagnostic().span_warn_with_code(sp, msg, code)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
    |
    |
574 |         self.diagnostic().struct_note_without_error(msg)


error: diagnostics should be created using translatable messages
    |
    |
574 |         self.diagnostic().struct_note_without_error(msg)


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1453 |         sess.err(
     |              ^^^


error: diagnostics should be created using translatable messages
     |
1453 |         sess.err(
     |              ^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1463 |             sess.err(&format!(


error: diagnostics should be created using translatable messages
     |
     |
1463 |             sess.err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1473 |             sess.err(&format!(


error: diagnostics should be created using translatable messages
     |
     |
1473 |             sess.err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1483 |             sess.err(
     |                  ^^^


error: diagnostics should be created using translatable messages
     |
1483 |             sess.err(
     |                  ^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1496 |             sess.err(&format!(


error: diagnostics should be created using translatable messages
     |
     |
1496 |             sess.err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1502 |             sess.err(&format!(


error: diagnostics should be created using translatable messages
     |
     |
1502 |             sess.err(&format!(


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1511 |         sess.err(&format!("`-Zsanitizer={first}` is incompatible with `-Zsanitizer={second}`"));


error: diagnostics should be created using translatable messages
     |
     |
1511 |         sess.err(&format!("`-Zsanitizer={first}` is incompatible with `-Zsanitizer={second}`"));


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
1516 |         sess.err(
     |              ^^^


error: diagnostics should be created using translatable messages
     |
1516 |         sess.err(
     |              ^^^


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1525 |             sess.err("`-Zsanitizer=cfi` requires `-Clto`");


error: diagnostics should be created using translatable messages
     |
     |
1525 |             sess.err("`-Zsanitizer=cfi` requires `-Clto`");


error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
     |
     |
1528 |             sess.err("`-Zvirtual-function-elimination` requires `-Clto`");
