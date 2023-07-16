plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e6a4afc3af2d2a53f91fc8a77bdfe94bea375b29 and 48cb8d3743ef7230cdda6ab203c9dd6ba89a772b
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::configuration_snippet::configuration_snippet_tests ... ok
test test::self_tests ... ok

Mismatch at tests/source/pub-restricted.rs:24:
     WriteData(Writer<D>),
 
 
-pub(crate) enum WriteState<D> {
+crate enum WriteState<D> {
     WriteId {
         id: U64Writer,
         size: U64Writer,
test test::idempotence_tests ... ok

failures:


---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10

failures:
    test::system_tests

