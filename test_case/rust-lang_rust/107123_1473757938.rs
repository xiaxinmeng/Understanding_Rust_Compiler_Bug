plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 13024/14639
........................................................................................ 13112/14639
........................................................................................ 13200/14639
........................................................................................ 13288/14639
........F...........................F................................................... 13376/14639
........................................................................................ 13552/14639
........................................................................................ 13640/14639
..........................................................i............................. 13728/14639
........................................................................................ 13816/14639
---
failures:

---- [ui] tests/ui/traits/new-solver/builtin-fn-must-return-sized.rs stdout ----

error: /checkout/tests/ui/traits/new-solver/builtin-fn-must-return-sized.rs:11: unexpected error: '11:26: 11:30: type annotations needed: cannot satisfy `for<'a> extern "rust-call" fn(&'a F, T) -> <F as FnOnce<T>>::Output {<F as Fn<T>>::call}: Callable<(&F, T)>` [E0283]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/builtin-fn-must-return-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/builtin-fn-must-return-sized" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/builtin-fn-must-return-sized/auxiliary" "-Ztrait-solver=next"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:26: 11:30: type annotations needed: cannot satisfy `for<'a> extern \"rust-call\" fn(&'a F, T) -> <F as FnOnce<T>>::Output {<F as Fn<T>>::call}: Callable<(&F, T)>` [E0283]",
]

thread '[ui] tests/ui/traits/new-solver/builtin-fn-must-return-sized.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1431:13


---- [ui] tests/ui/traits/new-solver/try-example.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/try-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/try-example" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/try-example/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0283]: type annotations needed: cannot satisfy `for<'a, 'b> fn(&'a mut Formatter<'_>, Arguments<'b>) -> Result<(), std::fmt::Error> {Formatter::<'_>::write_fmt}: Callable<(&mut Formatter<'_>, Arguments<'_>)>`
  --> fake-test-src-base/traits/new-solver/try-example.rs:20:9
LL |         write!(f, "ParseError")
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: cannot satisfy `for<'a, 'b> fn(&'a mut Formatter<'_>, Arguments<'b>) -> Result<(), std::fmt::Error> {Formatter::<'_>::write_fmt}: Callable<(&mut Formatter<'_>, Arguments<'_>)>`
   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
