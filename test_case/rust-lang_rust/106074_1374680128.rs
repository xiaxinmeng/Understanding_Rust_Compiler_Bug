plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ee0412d1ef81efcfabe7f66cd21476ca85d618b1 and be4283c164966a759b54840455c3e40aa4520739
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::stdin_handles_mod_inner_ignore_attr ... ok
test test::verify_check_works ... ok
test test::verify_check_works_with_stdin ... ok
test test::self_tests ... ok
error: expected `(`, found keyword `trait`
  |
6 | impl
  |     - expected `(`
7 | trait Bar {}
7 | trait Bar {}
  | ^^^^^ unexpected token

test test::system_tests ... FAILED
error: expected `(`, found keyword `trait`
  |
  |
3 | pub impl trait Bar {}
  |          ^^^^^ expected `(`
test test::idempotence_tests ... FAILED

failures:


---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:189:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10
---- test::idempotence_tests stdout ----
---- test::idempotence_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity="Crate"` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
  left: `1`,
 right: `0`: 1 idempotent tests failed', src/tools/rustfmt/src/test/mod.rs:369:9
thread 'test::idempotence_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:76:10

failures:
    test::idempotence_tests
    test::system_tests
