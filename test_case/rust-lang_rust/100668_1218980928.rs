plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9c20b2a8cc7588decb6de25ac6a7912dcef24d65 and e587eb76de94ffe921cc0da46207a903a84cb0ef
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

Mismatch at src/attr.rs:49:
 }
 
 /// Returns attributes that are within `outer_span`.
-pub(crate) fn filter_inline_attrs(
-    attrs: &[ast::Attribute],
-    outer_span: Span,
-) -> ast::AttrVec {
+pub(crate) fn filter_inline_attrs(attrs: &[ast::Attribute], outer_span: Span) -> ast::AttrVec {
         .iter()
         .iter()
         .filter(|a| outer_span.lo() <= a.span.lo() && a.span.hi() <= outer_span.hi())
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
