plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between dfd6306d26af1a163aaaa1456b4594244ddd182f and 77a350ef8115064692e5a20e0e37cd752147cde4
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::configuration_snippet::configuration_snippet_tests ... ok
error: expected identifier, found keyword `const`
   --> tests/source/type.rs:145:14
    |
145 | trait T: ?   const  Super {}


error: expected one of `(`, `+`, `::`, `<`, `=`, `where`, or `{`, found `Super`
   --> tests/source/type.rs:145:21
    |
145 | trait T: ?   const  Super {}
    |                     ^^^^^ expected one of 7 possible tokens
test test::self_tests ... ok
test test::self_tests ... ok
error: expected one of `!`, `(`, `::`, `;`, `<`, `where`, or `{`, found `~`
   --> tests/target/type.rs:160:35
    |
160 | fn trait_object() -> &'static dyn ~const T {
    |                                   ^ expected one of 7 possible tokens
test test::system_tests ... FAILED
test test::idempotence_tests ... FAILED

failures:
failures:

---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:187:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:74:10
---- test::idempotence_tests stdout ----
---- test::idempotence_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
  left: `1`,
 right: `0`: 1 idempotent tests failed', src/tools/rustfmt/src/test/mod.rs:324:9
thread 'test::idempotence_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:74:10

failures:
    test::idempotence_tests
    test::system_tests
