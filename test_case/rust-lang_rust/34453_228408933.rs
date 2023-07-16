
thread 'main' panicked at 'Some tests failed
---- [compile-fail] compile-fail/issue-24424.rs stdout ----

error: /build/src/test/compile-fail/issue-24424.rs:14: unexpected "error": '14:1: 14:89: type annotations required: cannot resolve `T0: Trait0<'l0>` [E0283]'
error: /build/src/test/compile-fail/issue-24424.rs:14: expected error not found: type annotations required: cannot resolve `T0: main::Trait0<'l0>`
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail/issue-24424.rs -L x86_64-unknown-linux-gnu/test/compile-fail/ --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L x86_64-unknown-linux-gnu/test/compile-fail/issue-24424.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/issue-24424.stage2-x86_64-unknown-linux-gnu --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt
actual errors (from JSON output): [
    Error {
        line_num: 14,
        kind: Some(
            Error
        ),
        msg: "14:1: 14:89: type annotations required: cannot resolve `T0: Trait0<\'l0>` [E0283]"
    },
    Error {
        line_num: 14,
        kind: Some(
            Note
        ),
        msg: "14:1: 14:89: required by `Trait0`"
    }
]
expected errors (from test file): [
    Error {
        line_num: 14,
        kind: Some(
            Error
        ),
        msg: "type annotations required: cannot resolve `T0: main::Trait0<\'l0>`"
    }
]
