
[01:32:52] error: expected identifier, found keyword `return`
[01:32:52]  --> bogofile:1:13
[01:32:52]   |
[01:32:52] 1 | ::abc::def::return
[01:32:52]   |             ^^^^^^ expected identifier, found keyword
[01:32:52] 
[01:32:52] ..........................F...........................................................
[01:32:52] failures:
[01:32:52] 
[01:32:52] ---- parse::tests::out_of_line_mod stdout ----
[01:32:52] thread 'parse::tests::out_of_line_mod' panicked at 'called `Result::unwrap()` on an `Err` value: Diagnostic { level: Error, message: [("file not found for module `this_does_not_exist`", NoStyle)], code: Some(Error("E0583")), span: MultiSpan { primary_spans: [Span { lo: BytePos(24), hi: BytePos(43), ctxt: #0 }], span_labels: [] }, children: [SubDiagnostic { level: Help, message: [("name the file either this_does_not_exist.rs or this_does_not_exist/mod.rs inside the directory \"foo\"", NoStyle)], span: MultiSpan { primary_spans: [], span_labels: [] }, render_span: None }], suggestions: [] }', libcore/result.rs:1009:5
[01:32:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:32:52] 
[01:32:52] 
[01:32:52] failures:
[01:32:52]     parse::tests::out_of_line_mod
[01:32:52] 
[01:32:52] test result: FAILED. 85 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
