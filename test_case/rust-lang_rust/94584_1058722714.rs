plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 10913c00018c76103b2fd4260d8c02ec728fd244 and 948de947b1025f0c6899787f2afb34d385e1ea00
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
error: tests/compile-fail/intrinsics/exact_div4.rs:4: expected error not found: result of dividing MIN by -1 cannot be represented

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div4.rs" "-L" "/tmp/compiletesttBqh1M" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesttBqh1M/intrinsics/exact_div4.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesttBqh1M/intrinsics/exact_div4.stage-id.aux"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/unchecked_div1.rs" "-L" "/tmp/compiletesttBqh1M" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesttBqh1M/intrinsics/unchecked_div1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesttBqh1M/intrinsics/unchecked_div1.stage-id.aux"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---
test test::verify_config_test_names ... ok
test test::verify_check_works ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/visitor.rs:916:
         self.push_str(&ident_str);
 
         if let ast::ModKind::Loaded(ref items, ast::Inline::Yes, ref spans) = mod_kind {
-            let ast::ModSpans{ inner_span, inject_use_span: _ } = *spans;
+            let ast::ModSpans {
+                inner_span,
+                inject_use_span: _,
+            } = *spans;
             match self.config.brace_style() {
                 BraceStyle::AlwaysNextLine => {
                     let indent_str = self.block_indent.to_string_with_newline(self.config);
Mismatch at src/parse/parser.rs:113:
         let result = catch_unwind(AssertUnwindSafe(|| {
         let result = catch_unwind(AssertUnwindSafe(|| {
             let mut parser = new_parser_from_file(sess.inner(), path, Some(span));
             match parser.parse_mod(&TokenKind::Eof) {
-                Ok((a, i, ast::ModSpans { inner_span, inject_use_span: _ })) => Some((a, i, inner_span)),
+                    a,
+                    i,
+                    ast::ModSpans {
+                        inner_span,
+                        inner_span,
+                        inject_use_span: _,
+                    },
+                )) => Some((a, i, inner_span)),
                 Err(mut e) => {
                     e.emit();
                     if sess.can_reset_errors() {
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:354:5


failures:
    test::self_tests
