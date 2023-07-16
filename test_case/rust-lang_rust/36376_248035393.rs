
error: /build/src/test/compile-fail/E0050.rs:12: unexpected "note": 'expected 2 parameters'

error: /build/src/test/compile-fail/E0050.rs:18: unexpected "note": 'found 1 parameter'

error: /build/src/test/compile-fail/E0050.rs:18: expected note not found: expected 2 parameters

error: 2 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail/E0050.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0050.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0050.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 12,
        kind: Some(
            Note
        ),
        msg: "expected 2 parameters"
    },
    Error {
        line_num: 18,
        kind: Some(
            Note
        ),
        msg: "found 1 parameter"
    }
]

not found errors (from test file): [
    Error {
        line_num: 18,
        kind: Some(
            Note
        ),
        msg: "expected 2 parameters"
    }
]

thread '[compile-fail] compile-fail/E0050.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1084
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/E0050.rs

test result: FAILED. 2571 passed; 1 failed; 12 ignored; 0 measured
