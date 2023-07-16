plain
failures:

---- [ui] src/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait.rs stdout ----

error: /checkout/src/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait.rs:14: expected error not found: `impl Trait` only allowed in function and inherent method return types, not in trait method return
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`impl Trait` only allowed in function and inherent method return types, not in trait method return",
]

thread '[ui] src/test/ui/feature-gates/feature-gate-return_position_impl_trait_in_trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
