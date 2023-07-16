plain
failures:

---- [ui] src/test/ui/asm/aarch64/type-check-2-2.rs stdout ----

error: /checkout/src/test/ui/asm/aarch64/type-check-2-2.rs:28: unexpected error: '28:13: 28:14: cannot borrow `v` as mutable, as it is not declared as mutable [E0596]'

error: /checkout/src/test/ui/asm/aarch64/type-check-2-2.rs:30: expected error not found: cannot borrow `v` as mutable, as it is not declared as mutable

error: /checkout/src/test/ui/asm/aarch64/type-check-2-2.rs:32: expected error not found: cannot borrow `v` as mutable, as it is not declared as mutable
error: 1 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2-2/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
    Error {
        line_num: 28,
        kind: Some(
            Error,
            Error,
        ),
        msg: "28:13: 28:14: cannot borrow `v` as mutable, as it is not declared as mutable [E0596]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 30,
        kind: Some(
            Error,
        ),
        msg: "cannot borrow `v` as mutable, as it is not declared as mutable",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "cannot borrow `v` as mutable, as it is not declared as mutable",
]

thread '[ui] src/test/ui/asm/aarch64/type-check-2-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1413:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
