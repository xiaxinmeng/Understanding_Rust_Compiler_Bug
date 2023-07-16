plain
failures:

---- [ui] src/test/ui/lint/recommend-literal.rs stdout ----

error: /checkout/src/test/ui/lint/recommend-literal.rs:10: unexpected help message: '10:16: 10:23: perhaps you intended to use this type'

error: /checkout/src/test/ui/lint/recommend-literal.rs:12: unexpected help message: '12:16: 12:20: a builtin type with a similar name exists'

error: /checkout/src/test/ui/lint/recommend-literal.rs:12: unexpected help message: '12:16: 12:20: perhaps you intended to use this type'
error: 3 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/recommend-literal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/recommend-literal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/recommend-literal/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Help,
            Help,
        ),
        msg: "10:16: 10:23: perhaps you intended to use this type",
    Error {
        line_num: 12,
        kind: Some(
            Help,
            Help,
        ),
        msg: "12:16: 12:20: a builtin type with a similar name exists",
    Error {
        line_num: 12,
        kind: Some(
            Help,
            Help,
        ),
        msg: "12:16: 12:20: perhaps you intended to use this type",
]

thread '[ui] src/test/ui/lint/recommend-literal.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1392:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
