
running 1 test
F
failures:

---- [ui] ui/issues/issue-81006-2.rs stdout ----

error: /home/omer/rust/rust/src/test/ui/issues/issue-81006-2.rs:6: unexpected error: '6:23: 6:25: 1 positional argument in format string, but no arguments were given'

error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 1
command: "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/omer/rust/rust/src/test/ui/issues/issue-81006-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81006-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/omer/rust/rust/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81006-2/auxiliary"
unexpected errors (from JSON output): [
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "6:23: 6:25: 1 positional argument in format string, but no arguments were given",
    },
]

thread '[ui] ui/issues/issue-81006-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
