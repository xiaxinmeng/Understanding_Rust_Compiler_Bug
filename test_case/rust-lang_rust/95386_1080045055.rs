plain
failures:

---- [ui] ui/did_you_mean/compatible-variants.rs stdout ----

error: /checkout/src/test/ui/did_you_mean/compatible-variants.rs:87: unexpected help message: '87:17: 87:17: try wrapping the expression in `A2`'
error: /checkout/src/test/ui/did_you_mean/compatible-variants.rs:87: expected error not found: try wrapping

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/compatible-variants.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/compatible-variants" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/compatible-variants/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
    Error {
        line_num: 87,
        kind: Some(
            Help,
            Help,
        ),
        msg: "87:17: 87:17: try wrapping the expression in `A2`",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 87,
        kind: Some(
            Error,
        ),
        msg: "try wrapping",
]

thread '[ui] ui/did_you_mean/compatible-variants.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
