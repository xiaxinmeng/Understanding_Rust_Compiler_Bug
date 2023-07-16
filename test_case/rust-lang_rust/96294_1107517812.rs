plain
........................................................................................ 12320/12952
........................................................................................ 12408/12952
........................................................................................ 12496/12952
........................................................................................ 12584/12952
...........................................................................F............ 12672/12952
............F........F.................................................................. 12760/12952
........................................................................................ 12936/12952
................
failures:


---- [ui] src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs#mir stdout ----

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:14: unexpected error: '14:5: 14:11: call to unsafe function `S::f` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:15: unexpected error: '15:5: 15:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:19: unexpected error: '19:5: 19:11: call to unsafe function `S::f` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:20: unexpected error: '20:5: 20:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:14: expected error not found: call to unsafe function is unsafe

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:15: expected error not found: call to unsafe function is unsafe

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:19: expected error not found: call to unsafe function is unsafe

error in revision `mir`: /checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs:20: expected error not found: call to unsafe function is unsafe

error in revision `mir`: 4 unexpected errors found, 4 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.mir/auxiliary"
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:5: 14:11: call to unsafe function `S::f` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:5: 15:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "19:5: 19:11: call to unsafe function `S::f` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:5: 20:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
---
        msg: "call to unsafe function is unsafe",
    },
]

thread '[ui] src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13

---- [ui] src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs:9: unexpected error: '9:17: 9:22: call to unsafe function `foo` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs:11: unexpected error: '11:5: 11:10: call to unsafe function `foo` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs:9: expected error not found: call to unsafe function is unsafe and requires unsafe function or block

error in revision `mir`: /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs:11: expected error not found: call to unsafe function is unsafe and requires unsafe function or block

error in revision `mir`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.mir/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:17: 9:22: call to unsafe function `foo` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:10: call to unsafe function `foo` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe function or block",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe function or block",
]


thread '[ui] src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/foreign-unsafe-fn-called.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/foreign-unsafe-fn-called.rs:11: unexpected error: '11:5: 11:17: call to unsafe function `test::free` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/foreign-unsafe-fn-called.rs:11: expected error not found: call to unsafe function is unsafe

error in revision `mir`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/foreign-unsafe-fn-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign-unsafe-fn-called.mir/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:17: call to unsafe function `test::free` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
---
        msg: "call to unsafe function is unsafe",
    },
]

thread '[ui] src/test/ui/foreign-unsafe-fn-called.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23: unexpected error: '23:5: 23:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:24: unexpected error: '24:5: 24:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:25: unexpected error: '25:5: 25:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:30: unexpected error: '30:5: 30:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:31: unexpected error: '31:5: 31:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:36: unexpected error: '36:5: 36:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:37: unexpected error: '37:5: 37:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:38: unexpected error: '38:5: 38:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:44: unexpected error: '44:5: 44:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:47: unexpected error: '47:18: 47:24: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:24: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:25: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:30: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:31: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:36: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:37: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:38: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:44: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:47: expected error not found: call to function with `#[target_feature]` is unsafe

error in revision `mir`: 10 unexpected errors found, 10 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.mir/auxiliary"
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:5: 23:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:5: 24:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "25:5: 25:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:5: 30:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "31:5: 31:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 36,
        kind: Some(
            Error,
            Error,
        ),
        msg: "36:5: 36:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 37,
        kind: Some(
            Error,
            Error,
        ),
        msg: "37:5: 37:15: call to function `avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 38,
        kind: Some(
            Error,
            Error,
        ),
        msg: "38:5: 38:20: call to function `Quux::avx_bmi2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 44,
        kind: Some(
            Error,
            Error,
        ),
        msg: "44:5: 44:11: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 47,
        kind: Some(
            Error,
            Error,
        ),
        msg: "47:18: 47:24: call to function `sse2` with `#[target_feature]` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 23,
        kind: Some(
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 36,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 37,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 38,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 44,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
    Error {
        line_num: 47,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to function with `#[target_feature]` is unsafe",
]


thread '[ui] src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/threads-sendsync/issue-43733.rs:19: unexpected error: '19:5: 19:32: call to unsafe function `std::thread::__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/threads-sendsync/issue-43733.rs:22: unexpected error: '22:42: 22:77: call to unsafe function `std::thread::LocalKey::<T>::new` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/threads-sendsync/issue-43733.rs:19: expected error not found: call to unsafe function is unsafe

error in revision `mir`: /checkout/src/test/ui/threads-sendsync/issue-43733.rs:22: expected error not found: call to unsafe function is unsafe

error in revision `mir`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/auxiliary"
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "19:5: 19:32: call to unsafe function `std::thread::__FastLocalKeyInner::<T>::get` is unsafe and requires unsafe function or block [E0133]",
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "22:42: 22:77: call to unsafe function `std::thread::LocalKey::<T>::new` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
---
        msg: "call to unsafe function is unsafe",
    },
]

thread '[ui] src/test/ui/threads-sendsync/issue-43733.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:12: unexpected error: '12:5: 12:11: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133) [unsafe_op_in_unsafe_fn]'

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:27: unexpected error: '27:5: 27:11: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133) [unsafe_op_in_unsafe_fn]'

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:76: unexpected error: '76:5: 76:11: call to unsafe function `unsf` is unsafe and requires unsafe block [E0133]'

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:80: unexpected error: '80:9: 80:15: call to unsafe function `unsf` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:12: expected error not found: call to unsafe function is unsafe and requires unsafe block

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:27: expected error not found: call to unsafe function is unsafe and requires unsafe block

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:76: expected error not found: call to unsafe function is unsafe and requires unsafe block

error in revision `mir`: /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:80: expected error not found: call to unsafe function is unsafe and requires unsafe function or block

error in revision `mir`: 4 unexpected errors found, 4 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.mir/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:5: 12:11: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133) [unsafe_op_in_unsafe_fn]",
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "27:5: 27:11: call to unsafe function `unsf` is unsafe and requires unsafe block (error E0133) [unsafe_op_in_unsafe_fn]",
    Error {
        line_num: 76,
        kind: Some(
            Error,
            Error,
        ),
        msg: "76:5: 76:11: call to unsafe function `unsf` is unsafe and requires unsafe block [E0133]",
    Error {
        line_num: 80,
        kind: Some(
            Error,
            Error,
        ),
        msg: "80:9: 80:15: call to unsafe function `unsf` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 12,
        kind: Some(
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe block",
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe block",
    Error {
        line_num: 76,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe block",
    Error {
        line_num: 80,
        kind: Some(
            Error,
            Error,
        ),
        msg: "call to unsafe function is unsafe and requires unsafe function or block",
]


thread '[ui] src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/unsafe/unsafe-fn-called-from-safe.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/unsafe/unsafe-fn-called-from-safe.rs:7: unexpected error: '7:5: 7:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/unsafe/unsafe-fn-called-from-safe.rs:7: expected error not found: call to unsafe function is unsafe

error in revision `mir`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-called-from-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-called-from-safe.mir/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:5: 7:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
---
        msg: "call to unsafe function is unsafe",
    },
]

thread '[ui] src/test/ui/unsafe/unsafe-fn-called-from-safe.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
---- [ui] src/test/ui/unsafe/unsafe-fn-used-as-value.rs#mir stdout ----


error in revision `mir`: /checkout/src/test/ui/unsafe/unsafe-fn-used-as-value.rs:8: unexpected error: '8:5: 8:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]'

error in revision `mir`: /checkout/src/test/ui/unsafe/unsafe-fn-used-as-value.rs:8: expected error not found: call to unsafe function is unsafe

error in revision `mir`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-fn-used-as-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-fn-used-as-value.mir/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:8: call to unsafe function `f` is unsafe and requires unsafe function or block [E0133]",
]

not found errors (from test file): [
    Error {
---
        msg: "call to unsafe function is unsafe",
    },
]

thread '[ui] src/test/ui/unsafe/unsafe-fn-used-as-value.rs#mir' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13

failures:
    [ui] src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs#mir
    [ui] src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs#mir
