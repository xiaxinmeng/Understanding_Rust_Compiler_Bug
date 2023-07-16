plain
failures:

---- [ui] src/test/ui/generic-associated-types/issue-91883.rs stdout ----

error: /checkout/src/test/ui/generic-associated-types/issue-91883.rs:32: unexpected error: '32:24: 32:39: lifetime bound not satisfied [E0478]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-91883.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91883/auxiliary"
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "32:24: 32:39: lifetime bound not satisfied [E0478]",
]

thread '[ui] src/test/ui/generic-associated-types/issue-91883.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1392:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
