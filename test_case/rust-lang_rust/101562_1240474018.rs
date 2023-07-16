plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4af35b8e3017cb080630942d1c2b7045c74e450a and f44767dafd99e282627a95a908aed3020b6ef26e
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::verify_check_works_with_stdin ... ok
test test::verify_check_works ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/expr.rs:1229:
     context: &RewriteContext<'_>,
     lit: &ast::Lit,
     span: Span,
-    shape: Shape
+    shape: Shape,
 ) -> Option<String> {
     let symbol = lit.symbol.as_str();
test test::self_tests ... FAILED
test test::system_tests ... ok
test test::idempotence_tests ... ok


failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
