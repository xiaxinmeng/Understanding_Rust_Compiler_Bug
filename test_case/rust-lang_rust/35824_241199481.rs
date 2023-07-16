
---- [compile-fail] compile-fail/E0455.rs stdout ----

error: /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/E0455.rs:11: expected error not found: E0455

error: 0 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/E0455.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin --error-format json -L x86_64-apple-darwin/test/compile-fail/E0455.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/E0455.stage2-x86_64-apple-darwin --cfg rtopt -C rpath -O -L x86_64-apple-darwin/rt
not found errors (from test file): [
    Error {
        line_num: 11,
        kind: Some(
            Error
        ),
        msg: "E0455"
    }
]

thread '[compile-fail] compile-fail/E0455.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/tools/compiletest/src/runtest.rs:1078
