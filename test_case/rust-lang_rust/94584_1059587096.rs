plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9fcbc32053d5084d1de79bd484de82474cdae427 and 45cb580ca1d5bd0ad404e5357ab1f19329d635e8
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestWihqIJ/available-concurrency.stage-id.stderr
To only update this specific test, also pass `--test-args available-concurrency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/available-concurrency.rs" "-L" "/tmp/compiletestWihqIJ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWihqIJ/available-concurrency.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestWihqIJ/available-concurrency.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/parse/parser.rs:113:
         let result = catch_unwind(AssertUnwindSafe(|| {
             let mut parser = new_parser_from_file(sess.inner(), path, Some(span));
             match parser.parse_mod(&TokenKind::Eof) {
-                Ok((a,
+                    a,
                     i,
                     ast::ModSpans {
                         inner_span,
                         inner_span,

Mismatch at src/parse/parser.rs:120:
-                        inject_use_span: _
-                    }
+                        inject_use_span: _,
+                    },
                 )) => Some((a, i, inner_span)),
                 Err(mut e) => {
                     e.emit();
Mismatch at src/visitor.rs:916:
Mismatch at src/visitor.rs:916:
         self.push_str(&ident_str);
 
         if let ast::ModKind::Loaded(ref items, ast::Inline::Yes, ref spans) = mod_kind {
-            let ast::ModSpans{
+            let ast::ModSpans {
                 inner_span,
-                inject_use_span: _
+                inject_use_span: _,
             } = *spans;
             match self.config.brace_style() {
                 BraceStyle::AlwaysNextLine => {
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
