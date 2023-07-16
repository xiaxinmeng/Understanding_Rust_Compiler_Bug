

error: /build/src/test/compile-fail/issue-36053-2.rs:17: expected error not found: E0281

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail/issue-36053-2.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36053-2.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-36053-2.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 17,
        kind: Some(
            Error
        ),
        msg: "17:55: 17:60: no method named `count` found for type `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/build/src/test/compile-fail/issue-36053-2.rs:17:39: 17:53]>` in the current scope"
    }
]

not found errors (from test file): [
    Error {
        line_num: 17,
        kind: Some(
            Error
        ),
        msg: "E0281"
    }
]

thread '[compile-fail] compile-fail/issue-36053-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1084
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/issue-36053-2.rs

test result: FAILED. 2565 passed; 1 failed; 12 ignored; 0 measured

