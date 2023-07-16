plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking askama_shared v0.12.0
   Compiling askama_derive v0.11.0
    Checking askama v0.11.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
   --> src/librustdoc/clean/types.rs:974:42
    |
974 | ...                   sess.span_err(e.span, e.msg);
    |
note: the lint level is defined here
   --> src/librustdoc/lib.rs:24:9
    |
    |
24  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   --> src/librustdoc/clean/types.rs:974:42
    |
974 | ...                   sess.span_err(e.span, e.msg);
    |
note: the lint level is defined here
   --> src/librustdoc/lib.rs:23:9
    |
    |
23  | #![deny(rustc::untranslatable_diagnostic)]


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
738  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `rustc_errors::struct_span_err!`
     |   ---------------------------- in this expansion of `rustc_errors::struct_span_err!`
739  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
740  |           $session.struct_span_err_with_code(
     |
    ::: src/librustdoc/clean/mod.rs:2408:13
     |
2408 | /             rustc_errors::struct_span_err!(
2408 | /             rustc_errors::struct_span_err!(
2409 | |                 cx.tcx.sess,
2410 | |                 inline.span(),
2411 | |                 E0780,
2412 | |                 "anonymous imports cannot be inlined"
     | |_____________- in this macro invocation


error: diagnostics should be created using translatable messages
     |
738  |   macro_rules! struct_span_err {
     |   ---------------------------- in this expansion of `rustc_errors::struct_span_err!`
     |   ---------------------------- in this expansion of `rustc_errors::struct_span_err!`
739  |       ($session:expr, $span:expr, $code:ident, $($message:tt)*) => ({
740  |           $session.struct_span_err_with_code(
     |
    ::: src/librustdoc/clean/mod.rs:2408:13
     |
2408 | /             rustc_errors::struct_span_err!(
2408 | /             rustc_errors::struct_span_err!(
2409 | |                 cx.tcx.sess,
2410 | |                 inline.span(),
2411 | |                 E0780,
2412 | |                 "anonymous imports cannot be inlined"
     | |_____________- in this macro invocation


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
433 |                     diag.struct_err(e).emit();


error: diagnostics should be created using translatable messages
    |
    |
433 |                     diag.struct_err(e).emit();


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
570 |                     diag.struct_err(e).emit();


error: diagnostics should be created using translatable messages
    |
    |
570 |                     diag.struct_err(e).emit();


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
653 |                     diag.struct_err(&e).emit();


error: diagnostics should be created using translatable messages
    |
    |
653 |                     diag.struct_err(&e).emit();


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
819 |         self.tcx.struct_span_lint_hir(


error: diagnostics should be created using translatable messages
    |
    |
819 |         self.tcx.struct_span_lint_hir(


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
   --> src/librustdoc/html/render/context.rs:727:65
    |
727 |             self.shared.errors.iter().map(|err| self.tcx().sess.struct_err(&err).emit()).count();


error: diagnostics should be created using translatable messages
   --> src/librustdoc/html/render/context.rs:727:65
    |
727 |             self.shared.errors.iter().map(|err| self.tcx().sess.struct_err(&err).emit()).count();


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    --> src/librustdoc/html/render/mod.rs:2860:22
     |
2860 |                     .span_err(span, &format!("failed to read file {}: {}", path.display(), err));


error: diagnostics should be created using translatable messages
    --> src/librustdoc/html/render/mod.rs:2860:22
     |
2860 |                     .span_err(span, &format!("failed to read file {}: {}", path.display(), err));


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
   --> src/librustdoc/html/sources.rs:143:45
    |
143 |                     self.cx.shared.tcx.sess.span_err(


error: diagnostics should be created using translatable messages
   --> src/librustdoc/html/sources.rs:143:45
    |
143 |                     self.cx.shared.tcx.sess.span_err(


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1234 |                 diag.span_label(sp, &note);


error: diagnostics should be created using translatable messages
     |
     |
1234 |                 diag.span_label(sp, &note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1236 |                 diag.note(&note);


error: diagnostics should be created using translatable messages
     |
     |
1236 |                 diag.note(&note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1612 |     tcx.struct_span_lint_hir(lint, hir_id, sp, msg, |lint| {


error: diagnostics should be created using translatable messages
     |
     |
1612 |     tcx.struct_span_lint_hir(lint, hir_id, sp, msg, |lint| {


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1747 | ...                   diag.span_label(span, &note);


error: diagnostics should be created using translatable messages
     |
     |
1747 | ...                   diag.span_label(span, &note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1749 | ...                   diag.note(&note);


error: diagnostics should be created using translatable messages
     |
     |
1749 | ...                   diag.note(&note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1757 | ...                   diag.note(format!(


error: diagnostics should be created using translatable messages
     |
     |
1757 | ...                   diag.note(format!(


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1765 | ...                   diag.help("to escape `[` and `]` characters, \


error: diagnostics should be created using translatable messages
     |
     |
1765 | ...                   diag.help("to escape `[` and `]` characters, \


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1803 | ...                   diag.span_label(span, &note);


error: diagnostics should be created using translatable messages
     |
     |
1803 | ...                   diag.span_label(span, &note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1805 | ...                   diag.note(&note);


error: diagnostics should be created using translatable messages
     |
     |
1805 | ...                   diag.note(&note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1825 |                         diag.span_label(span, &note);


error: diagnostics should be created using translatable messages
     |
     |
1825 |                         diag.span_label(span, &note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1827 |                         diag.note(&note);


error: diagnostics should be created using translatable messages
     |
     |
1827 |                         diag.note(&note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1845 |                     diag.span_label(span, &note);


error: diagnostics should be created using translatable messages
     |
     |
1845 |                     diag.span_label(span, &note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1847 |                     diag.note(&note);


error: diagnostics should be created using translatable messages
     |
     |
1847 |                     diag.note(&note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1881 |             diag.span_label(sp, "invalid anchor");


error: diagnostics should be created using translatable messages
     |
     |
1881 |             diag.span_label(sp, "invalid anchor");


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1899 |         diag.note(&msg);


error: diagnostics should be created using translatable messages
     |
     |
1899 |         diag.note(&msg);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
1919 |                     diag.note(
     |                          ^^^^


error: diagnostics should be created using translatable messages
     |
1919 |                     diag.note(
     |                          ^^^^


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1929 |                 diag.span_label(span, note);


error: diagnostics should be created using translatable messages
     |
     |
1929 |                 diag.span_label(span, note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1931 |                 diag.note(note);


error: diagnostics should be created using translatable messages
     |
     |
1931 |                 diag.note(note);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1970 |             diag.span_label(sp, "ambiguous link");


error: diagnostics should be created using translatable messages
     |
     |
1970 |             diag.span_label(sp, "ambiguous link");


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
1972 |             diag.note("ambiguous link");


error: diagnostics should be created using translatable messages
     |
     |
1972 |             diag.note("ambiguous link");


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
2002 |         diag.help(&format!("{}: {}", help, suggestion.as_help(path_str)));


error: diagnostics should be created using translatable messages
     |
     |
2002 |         diag.help(&format!("{}: {}", help, suggestion.as_help(path_str)));


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
2021 |             diag.span_label(sp, "this item is private");


error: diagnostics should be created using translatable messages
     |
     |
2021 |             diag.span_label(sp, "this item is private");


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
     |
     |
2029 |         diag.note(note_msg);


error: diagnostics should be created using translatable messages
     |
     |
2029 |         diag.note(note_msg);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
124 |             cx.tcx.struct_span_lint_hir(


error: diagnostics should be created using translatable messages
    |
    |
124 |             cx.tcx.struct_span_lint_hir(


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
135 |         cx.tcx.struct_span_lint_hir(


error: diagnostics should be created using translatable messages
    |
    |
135 |         cx.tcx.struct_span_lint_hir(


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
   |
   |
26 |             cx.tcx.struct_span_lint_hir(crate::lint::BARE_URLS, hir_id, sp, msg, |lint| {


error: diagnostics should be created using translatable messages
   |
   |
26 |             cx.tcx.struct_span_lint_hir(crate::lint::BARE_URLS, hir_id, sp, msg, |lint| {


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
  --> src/librustdoc/passes/lint/check_code_block_syntax.rs:96:12
   |
96 |     cx.tcx.struct_span_lint_hir(crate::lint::INVALID_RUST_CODEBLOCKS, hir_id, sp, msg, |lint| {


error: diagnostics should be created using translatable messages
  --> src/librustdoc/passes/lint/check_code_block_syntax.rs:96:12
   |
96 |     cx.tcx.struct_span_lint_hir(crate::lint::INVALID_RUST_CODEBLOCKS, hir_id, sp, msg, |lint| {


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
  --> src/librustdoc/passes/lint/html_tags.rs:25:17
   |
25 |             tcx.struct_span_lint_hir(crate::lint::INVALID_HTML_TAGS, hir_id, sp, msg, |lint| {


error: diagnostics should be created using translatable messages
  --> src/librustdoc/passes/lint/html_tags.rs:25:17
   |
25 |             tcx.struct_span_lint_hir(crate::lint::INVALID_HTML_TAGS, hir_id, sp, msg, |lint| {


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
    |
338 |         tcx.sess.fatal(&e);


error: diagnostics should be created using translatable messages
    |
    |
338 |         tcx.sess.fatal(&e);


error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
    |
