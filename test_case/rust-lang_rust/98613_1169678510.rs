plain
failures:

---- [ui] src/test/ui/associated-types/hr-associated-type-bound-2.rs stdout ----

error: /checkout/src/test/ui/associated-types/hr-associated-type-bound-2.rs:11: unexpected error: '11:1: 16:2: overflow evaluating the requirement `<u32 as X<'b>>::U` [E0275]'

error: /checkout/src/test/ui/associated-types/hr-associated-type-bound-2.rs:11: expected message not found: overflow evaluating the requirement `for<'b> u32: X<'b>`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-2/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:1: 16:2: overflow evaluating the requirement `<u32 as X<'b>>::U` [E0275]",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

not found errors (from test file): [
not found errors (from test file): [
    Error {
        line_num: 11,
        kind: None,
        msg: "overflow evaluating the requirement `for<'b> u32: X<'b>`",
]

thread '[ui] src/test/ui/associated-types/hr-associated-type-bound-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1392:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/associated-types/project-recursion-limit-non-fatal.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/project-recursion-limit-non-fatal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/project-recursion-limit-non-fatal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/project-recursion-limit-non-fatal/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<Self as FilterDsl<_>>::Output`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`project_recursion_limit_non_fatal`)
note: required because of the requirements on the impl of `HandleDelete` for `Self`
   |
   |
LL | impl<T> HandleDelete for T

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
