
failures:

---- [compile-fail] compile-fail/issue-26886.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/issue-26886.rs:13: unexpected "note": 'already imported'

error: /home/logic/build/rust/src/test/compile-fail/issue-26886.rs:14: unexpected "note": 'already imported'

error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/issue-26886.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26886.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-26886.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 13,
        kind: Some(
            Note
        ),
        msg: "already imported"
    },
    Error {
        line_num: 14,
        kind: Some(
            Note
        ),
        msg: "already imported"
    }
]

thread '[compile-fail] compile-fail/issue-26886.rs' panicked at 'explicit panic', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [compile-fail] compile-fail/use-mod.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/use-mod.rs:16: unexpected "note": 'already imported'

error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/use-mod.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/use-mod.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/use-mod.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 16,
        kind: Some(
            Note
        ),
        msg: "already imported"
    }
]

thread '[compile-fail] compile-fail/use-mod.rs' panicked at 'explicit panic', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082


failures:
    [compile-fail] compile-fail/issue-26886.rs
    [compile-fail] compile-fail/use-mod.rs
