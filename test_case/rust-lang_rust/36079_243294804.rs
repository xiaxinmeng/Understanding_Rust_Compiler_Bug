 bash
---- [compile-fail] compile-fail\typeck-default-trait-impl-outside-crate.rs stdout ----

error: C:/bot/slave/auto-win-gnu-32-opt-rustbuild/build/src/test/compile-fail/typeck-default-trait-impl-outside-crate.rs:13: expected error not found: `Copy` trait not defined in this crate

error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: PATH="C:\bot\slave\..."
not found errors (from test file): [
    Error {
        line_num: 13,
        kind: Some(
            Error
        ),
        msg: "`Copy` trait not defined in this crate"
    }
]

thread '[compile-fail] compile-fail\typeck-default-trait-impl-outside-crate.rs' panicked at 'explicit panic', C:\bot\slave\auto-win-gnu-32-opt-rustbuild\build\src\tools\compiletest\src\runtest.rs:1084


failures:
    [compile-fail] compile-fail\typeck-default-trait-impl-outside-crate.rs

test result: FAILED. 2524 passed; 1 failed; 12 ignored; 0 measured
