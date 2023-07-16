
failures:

---- [compile-fail] compile-fail/issue-15260.rs stdout ----

error: /build/src/test/compile-fail/issue-15260.rs:18: unexpected "note": 'multiple uses of `a` in pattern'

error: /build/src/test/compile-fail/issue-15260.rs:17: unexpected "note": 'first use of `a`'

error: /build/src/test/compile-fail/issue-15260.rs:23: unexpected "note": 'multiple uses of `a` in pattern'

error: /build/src/test/compile-fail/issue-15260.rs:22: unexpected "note": 'first use of `a`'

error: /build/src/test/compile-fail/issue-15260.rs:29: unexpected "note": 'multiple uses of `a` in pattern'

error: /build/src/test/compile-fail/issue-15260.rs:27: unexpected "note": 'first use of `a`'

error: /build/src/test/compile-fail/issue-15260.rs:30: unexpected "note": 'multiple uses of `a` in pattern'

error: /build/src/test/compile-fail/issue-15260.rs:27: unexpected "note": 'first use of `a`'

error: /build/src/test/compile-fail/issue-15260.rs:17: expected note not found: field `a` previously bound here

error: /build/src/test/compile-fail/issue-15260.rs:22: expected note not found: field `a` previously bound here

error: /build/src/test/compile-fail/issue-15260.rs:27: expected note not found: field `a` previously bound here

error: /build/src/test/compile-fail/issue-15260.rs:27: expected note not found: field `a` previously bound here

error: 8 unexpected errors found, 4 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail/issue-15260.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-15260.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-15260.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 18,
        kind: Some(
            Note
        ),
        msg: "multiple uses of `a` in pattern"
    },
    Error {
        line_num: 17,
        kind: Some(
            Note
        ),
        msg: "first use of `a`"
    },
    Error {
        line_num: 23,
        kind: Some(
            Note
        ),
        msg: "multiple uses of `a` in pattern"
    },
    Error {
        line_num: 22,
        kind: Some(
            Note
        ),
        msg: "first use of `a`"
    },
    Error {
        line_num: 29,
        kind: Some(
            Note
        ),
        msg: "multiple uses of `a` in pattern"
    },
    Error {
        line_num: 27,
        kind: Some(
            Note
        ),
        msg: "first use of `a`"
    },
    Error {
        line_num: 30,
        kind: Some(
            Note
        ),
        msg: "multiple uses of `a` in pattern"
    },
    Error {
        line_num: 27,
        kind: Some(
            Note
        ),
        msg: "first use of `a`"
    }
]

not found errors (from test file): [
    Error {
        line_num: 17,
        kind: Some(
            Note
        ),
        msg: "field `a` previously bound here"
    },
    Error {
        line_num: 22,
        kind: Some(
            Note
        ),
        msg: "field `a` previously bound here"
    },
    Error {
        line_num: 27,
        kind: Some(
            Note
        ),
        msg: "field `a` previously bound here"
    },
    Error {
        line_num: 27,
        kind: Some(
            Note
        ),
        msg: "field `a` previously bound here"
    }
]

thread '[compile-fail] compile-fail/issue-15260.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1097
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/issue-15260.rs

test result: FAILED. 2575 passed; 1 failed; 12 ignored; 0 measured
