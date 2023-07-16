plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 12e4fd0755d7d976d4ee0f2004dc938290752ff7 and 36dc0e963aabe19cc8dbf1964e29eb4683cf760a
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test utils::test::test_remove_trailing_white_spaces ... ok
test test::verify_check_works ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/patterns.rs:1:
-use rustc_ast::ast::{self, BindingAnnotation, ByRef, Pat, PatField, PatKind, RangeEnd, RangeSyntax};
+use rustc_ast::ast::{
+    self, BindingAnnotation, ByRef, Pat, PatField, PatKind, RangeEnd, RangeSyntax,
 use rustc_ast::ptr;
 use rustc_ast::ptr;
 use rustc_span::{BytePos, Span};
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
