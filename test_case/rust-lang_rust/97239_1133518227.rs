plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e57884b6e96bede12447e21edcdc92fcac59ee46 and 67306d99bbe483f3b4097befbd5ae89e88bd8ecc
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test unit_tests::test_no_panic_on_format_snippet_and_format_code_block ... ok
test unit_tests::test_format_code_block ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/utils.rs:45:
         ) => pprust::path_to_string(p) == pprust::path_to_string(q),
         (VisibilityKind::Public, VisibilityKind::Public)
         | (VisibilityKind::Inherited, VisibilityKind::Inherited)
-        | (
-            VisibilityKind::Crate,
-            VisibilityKind::Crate,
-        ) => true,
+        | (VisibilityKind::Crate, VisibilityKind::Crate) => true,
     }
 }
test test::self_tests ... FAILED


Mismatch at tests/source/pub-restricted.rs:24:
     WriteData(Writer<D>),
 
 
-pub(crate) enum WriteState<D> {
-    WriteId {
-        id: U64Writer,
-        size: U64Writer,
-        payload: Option<Writer<D>>,
-    WriteSize {
-    WriteSize {
-        size: U64Writer,
-        payload: Option<Writer<D>>,
-    },
-    WriteData(Writer<D>),
-
-
 pub(in global::path::to::some_mod) enum WriteState<D> {
     WriteId {
         id: U64Writer,
test test::idempotence_tests ... ok

failures:


---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5

---- test::system_tests stdout ----
---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10

failures:
    test::self_tests
    test::system_tests
