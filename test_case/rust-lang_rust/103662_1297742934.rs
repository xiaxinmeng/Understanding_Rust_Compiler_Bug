plain
failures:

---- [ui] src/test/ui/async-await/issue-98634.rs stdout ----

error: /checkout/src/test/ui/async-await/issue-98634.rs:45: expected error not found: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-98634.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634/auxiliary"
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

thread '[ui] src/test/ui/async-await/issue-98634.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1419:13
