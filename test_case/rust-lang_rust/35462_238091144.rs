
failures:

---- [compile-fail] compile-fail/E0206.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/E0206.rs:13: unexpected "note": 'impl doesn't use types inside crate'

error: /home/logic/build/rust/src/test/compile-fail/E0206.rs:13: unexpected "note": '13:1: 13:22: the impl does not reference any types defined in this crate'

error: 2 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/E0206.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0206.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0206.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 13,
        kind: Some(
            Note
        ),
        msg: "impl doesn\'t use types inside crate"
    },
    Error {
        line_num: 13,
        kind: Some(
            Note
        ),
        msg: "13:1: 13:22: the impl does not reference any types defined in this crate"
    }
]

thread '[compile-fail] compile-fail/E0206.rs' panicked at 'Box<Any>', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [compile-fail] compile-fail/E0220.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/E0220.rs:15: unexpected "error": '15:12: 15:24: the value of the associated type `Bar` (from the trait `Trait`) must be specified [E0191]'

error: /home/logic/build/rust/src/test/compile-fail/E0220.rs:16: expected error not found: E0191

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/E0220.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0220.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/E0220.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 15,
        kind: Some(
            Error
        ),
        msg: "15:12: 15:24: the value of the associated type `Bar` (from the trait `Trait`) must be specified [E0191]"
    }
]

not found errors (from test file): [
    Error {
        line_num: 16,
        kind: Some(
            Error
        ),
        msg: "E0191"
    }
]

thread '[compile-fail] compile-fail/E0220.rs' panicked at 'Box<Any>', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082

---- [compile-fail] compile-fail/coherence-impls-copy.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:34: unexpected "note": 'impl doesn't use types inside crate'

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:34: unexpected "note": '34:1: 34:34: the impl does not reference any types defined in this crate'

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:43: unexpected "note": 'impl doesn't use types inside crate'

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:43: unexpected "note": '43:1: 43:26: the impl does not reference any types defined in this crate'

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:48: unexpected "note": 'impl doesn't use types inside crate'

error: /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs:48: unexpected "note": '48:1: 48:36: the impl does not reference any types defined in this crate'

error: 6 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/coherence-impls-copy.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-impls-copy.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-impls-copy.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 34,
        kind: Some(
            Note
        ),
        msg: "impl doesn\'t use types inside crate"
    },
    Error {
        line_num: 34,
        kind: Some(
            Note
        ),
        msg: "34:1: 34:34: the impl does not reference any types defined in this crate"
    },
    Error {
        line_num: 43,
        kind: Some(
            Note
        ),
        msg: "impl doesn\'t use types inside crate"
    },
    Error {
        line_num: 43,
        kind: Some(
            Note
        ),
        msg: "43:1: 43:26: the impl does not reference any types defined in this crate"
    },
    Error {
        line_num: 48,
        kind: Some(
            Note
        ),
        msg: "impl doesn\'t use types inside crate"
    },
    Error {
        line_num: 48,
        kind: Some(
            Note
        ),
        msg: "48:1: 48:36: the impl does not reference any types defined in this crate"
    }
]

thread '[compile-fail] compile-fail/coherence-impls-copy.rs' panicked at 'Box<Any>', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082

---- [compile-fail] compile-fail/unboxed-closure-sugar-wrong-trait.rs stdout ----

error: /home/logic/build/rust/src/test/compile-fail/unboxed-closure-sugar-wrong-trait.rs:15: unexpected "note": 'associated `Output` not found in `Trait`'

error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 101
command: /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /home/logic/build/rust/src/test/compile-fail/unboxed-closure-sugar-wrong-trait.rs -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -Z unstable-options -L /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/unboxed-closure-sugar-wrong-trait.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/test/compile-fail/unboxed-closure-sugar-wrong-trait.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/rust-test-helpers
unexpected errors (from JSON output): [
    Error {
        line_num: 15,
        kind: Some(
            Note
        ),
        msg: "associated `Output` not found in `Trait`"
    }
]

thread '[compile-fail] compile-fail/unboxed-closure-sugar-wrong-trait.rs' panicked at 'Box<Any>', /home/logic/build/rust/src/tools/compiletest/src/runtest.rs:1082


failures:
    [compile-fail] compile-fail/E0206.rs
    [compile-fail] compile-fail/E0220.rs
    [compile-fail] compile-fail/coherence-impls-copy.rs
    [compile-fail] compile-fail/unboxed-closure-sugar-wrong-trait.rs
