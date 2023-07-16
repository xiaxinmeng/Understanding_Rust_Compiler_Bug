
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 60 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiFiiiiiiiiiiiiiiiiiiiiiiiiii.......
failures:

---- [ui] rustdoc-ui/infinite-recursive-type.rs stdout ----

error: /home/joshua/rustc/src/test/rustdoc-ui/infinite-recursive-type.rs:1: unexpected error: '1:1: 1:7: cycle detected when computing `Sized` constraints for `E` [E0391]'

error: /home/joshua/rustc/src/test/rustdoc-ui/infinite-recursive-type.rs:1: unexpected error: '1:1: 1:1: TyKind::Error constructed but no error reported'

error: /home/joshua/rustc/src/test/rustdoc-ui/infinite-recursive-type.rs:1: expected error not found: recursive type `E` has infinite size

error: 2 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/bin/rustdoc" "/home/joshua/rustc/src/test/rustdoc-ui/infinite-recursive-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/joshua/rustc/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type/auxiliary"
unexpected errors (from JSON output): [
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "1:1: 1:7: cycle detected when computing `Sized` constraints for `E` [E0391]",
    },
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "1:1: 1:1: TyKind::Error constructed but no error reported",
    },
]

not found errors (from test file): [
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "recursive type `E` has infinite size",
    },
]

thread '[ui] rustdoc-ui/infinite-recursive-type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1483:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    [ui] rustdoc-ui/infinite-recursive-type.rs

test result: FAILED. 7 passed; 1 failed; 52 ignored; 0 measured; 0 filtered out
