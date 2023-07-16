plain
 finished in 0.790 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 163 tests
.........F.F..........F.................................................................  88/163

failures:

---- [incremental] tests/incremental/const-generics/hash-tyvid-regression-1.rs stdout ----
---- [incremental] tests/incremental/const-generics/hash-tyvid-regression-1.rs stdout ----

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-1.rs:5: unexpected error: '5:18: 5:40: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]'
Build completed unsuccessfully in 0:12:53
Build completed unsuccessfully in 0:12:53
error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-1.rs:10: expected error not found: the trait bound

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-1.rs:10: expected error not found: the trait bound

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-1.rs:10: expected error not found: mismatched types

error in revision `cfail`: 1 unexpected errors found, 3 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/const-generics/hash-tyvid-regression-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1/hash-tyvid-regression-1.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:18: 5:40: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]",
]

not found errors (from test file): [
    Error {
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [incremental] tests/incremental/const-generics/hash-tyvid-regression-2.rs stdout ----

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-2.rs:5: unexpected error: '5:22: 5:45: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]'

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-2.rs:8: unexpected error: '8:19: 8:42: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]'

error in revision `cfail`: /checkout/tests/incremental/const-generics/hash-tyvid-regression-2.rs:13: expected error not found: can't compare

error in revision `cfail`: 2 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/const-generics/hash-tyvid-regression-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-2/hash-tyvid-regression-2.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-2" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-2/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:22: 5:45: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:19: 8:42: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter [E0741]",
]

not found errors (from test file): [
    Error {
---
thread '[incremental] tests/incremental/const-generics/hash-tyvid-regression-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13

---- [incremental] tests/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-77708-3.rs stdout ----

error in revision `rpass`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-77708-3.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-77708-3/issue-77708-3.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-77708-3/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/try_unify_abstract_const_regression_tests/issue-77708-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0741]: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter
   |
   |
LL | struct A<const N: NonZeroUsize>([u8; N.get()])


error[E0741]: `NonZeroUsize` must be annotated with `#[derive(ConstParamTy)]` to be used as the type of a const generic parameter
   |
   |
LL | impl<'a, const N: NonZeroUsize> TryFrom<&'a [u8]> for A<N>

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0741`.
