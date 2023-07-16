
failures:

---- [compile-fail] compile-fail/rustc-macro/append-impl.rs stdout ----

error: /build/src/test/compile-fail-fulldeps/rustc-macro/append-impl.rs:27: unexpected "error": '27:1: 29:2: the semantics of constant patterns is not yet settled (see issue #31434)'

error: /build/src/test/compile-fail-fulldeps/rustc-macro/append-impl.rs:24: expected error not found: the semantics of constant patterns is not yet settled

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail-fulldeps/rustc-macro/append-impl.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/append-impl.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/append-impl.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 27,
        kind: Some(
            Error
        ),
        msg: "27:1: 29:2: the semantics of constant patterns is not yet settled (see issue #31434)"
    }
]

not found errors (from test file): [
    Error {
        line_num: 24,
        kind: Some(
            Error
        ),
        msg: "the semantics of constant patterns is not yet settled"
    }
]

thread '[compile-fail] compile-fail/rustc-macro/append-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1084
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [compile-fail] compile-fail/rustc-macro/expand-to-unstable-2.rs stdout ----

error: /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable-2.rs:21: unexpected "error": '21:1: 21:10: unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics (see issue #29642)'

error: /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable-2.rs:19: expected error not found: reserved for internal compiler

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable-2.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable-2.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable-2.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 21,
        kind: Some(
            Error
        ),
        msg: "21:1: 21:10: unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics (see issue #29642)"
    }
]

not found errors (from test file): [
    Error {
        line_num: 19,
        kind: Some(
            Error
        ),
        msg: "reserved for internal compiler"
    }
]

thread '[compile-fail] compile-fail/rustc-macro/expand-to-unstable-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1084

---- [compile-fail] compile-fail/rustc-macro/expand-to-unstable.rs stdout ----

error: /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable.rs:21: unexpected "error": '21:1: 21:10: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)'

error: /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable.rs:19: expected error not found: use of unstable library feature

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail-fulldeps/rustc-macro/expand-to-unstable.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 21,
        kind: Some(
            Error
        ),
        msg: "21:1: 21:10: use of unstable library feature \'core_intrinsics\': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library (see issue #0)"
    }
]

not found errors (from test file): [
    Error {
        line_num: 19,
        kind: Some(
            Error
        ),
        msg: "use of unstable library feature"
    }
]

thread '[compile-fail] compile-fail/rustc-macro/expand-to-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1084


failures:
    [compile-fail] compile-fail/rustc-macro/append-impl.rs
    [compile-fail] compile-fail/rustc-macro/expand-to-unstable-2.rs
    [compile-fail] compile-fail/rustc-macro/expand-to-unstable.rs

test result: FAILED. 43 passed; 3 failed; 0 ignored; 0 measured
