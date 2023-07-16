plain
..........................................................................................iiiiii.i.. 9900/11988
iiiiii.i............................................................................................ 10000/11988
.................................................................................................... 10100/11988
.................................................................................................... 10200/11988
...................F..F............................................................................. 10300/11988
.................................................................................................... 10500/11988
.................................................................................................... 10600/11988
.............................................................................................ii..... 10700/11988
..................................i................................................................. 10800/11988
---
failures:

---- [ui] ui/error-codes/E0275.rs stdout ----

error: /checkout/src/test/ui/error-codes/E0275.rs:1: unexpected error: '1:1: 1:10: overflow evaluating the requirement `Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0275.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
            Error,
        ),
        msg: "1:1: 1:10: overflow evaluating the requirement `Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Bar<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]",
]

thread '[ui] ui/error-codes/E0275.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/issues/issue-20413.rs stdout ----

error: /checkout/src/test/ui/issues/issue-20413.rs:1: unexpected error: '1:1: 1:10: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]'

error: /checkout/src/test/ui/issues/issue-20413.rs:16: unexpected error: '16:1: 16:10: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar` [E0275]'

error: /checkout/src/test/ui/issues/issue-20413.rs:20: unexpected error: '20:1: 20:10: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz` [E0275]'

error: /checkout/src/test/ui/issues/issue-20413.rs:1: unexpected error: '1:1: 1:10: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]'

error: /checkout/src/test/ui/issues/issue-20413.rs:16: unexpected error: '16:1: 16:10: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar` [E0275]'

error: /checkout/src/test/ui/issues/issue-20413.rs:20: unexpected error: '20:1: 20:10: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz` [E0275]'
error: 6 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20413.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20413" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20413/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:10: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "16:1: 16:10: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar` [E0275]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:1: 20:10: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz` [E0275]",
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:10: overflow evaluating the requirement `NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<NoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Foo` [E0275]",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "16:1: 16:10: overflow evaluating the requirement `AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Bar` [E0275]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:1: 20:10: overflow evaluating the requirement `EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<EvenLessData<AlmostNoData<Self>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>: Baz` [E0275]",
]

thread '[ui] ui/issues/issue-20413.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13


---- [ui] ui/specialization/issue-38091-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0275]: overflow evaluating the requirement `Self: Iterate<'_>`
   |
   |
LL | trait Iterate<'a> {
   | ^^^^^^^^^^^^^^^^^ required by this bound in `Iterate`
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_38091_2`)
note: required because of the requirements on the impl of `Check` for `Self`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |             ^^^^^     ^
note: required because of the requirements on the impl of `Iterate<'_>` for `Self`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   |             ^^^^^^^^^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterate<'a>` for `Self`

error[E0275]: overflow evaluating the requirement `Self: Iterate<'_>`
   |
   |
LL |     type Ty: Valid;
   |
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_38091_2`)
note: required because of the requirements on the impl of `Check` for `Self`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |             ^^^^^     ^
note: required because of the requirements on the impl of `Iterate<'_>` for `Self`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   |             ^^^^^^^^^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterate<'a>` for `Self`

error[E0275]: overflow evaluating the requirement `Self: Iterate<'_>`
   |
   |
LL | trait Iterate<'a> {
   | ^^^^^^^^^^^^^^^^^ required by this bound in `Iterate`
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_38091_2`)
note: required because of the requirements on the impl of `Check` for `Self`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |             ^^^^^     ^
note: required because of the requirements on the impl of `Iterate<'_>` for `Self`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   |             ^^^^^^^^^^^     ^
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterate<'a>` for `Self`
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0275`.


------------------------------------------


---- [ui] ui/specialization/issue-38091.rs stdout ----

error: /checkout/src/test/ui/specialization/issue-38091.rs:4: unexpected error: '4:1: 4:18: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]'

error: /checkout/src/test/ui/specialization/issue-38091.rs:5: unexpected error: '5:5: 5:20: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]'

error: /checkout/src/test/ui/specialization/issue-38091.rs:4: unexpected error: '4:1: 4:18: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]'

error: /checkout/src/test/ui/specialization/issue-38091.rs:12: expected error not found: the trait bound `(): Valid` is not satisfied
error: 3 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:1: 4:18: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:5: 5:20: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]",
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:1: 4:18: overflow evaluating the requirement `Self: Iterate<'_>` [E0275]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 12,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `(): Valid` is not satisfied",
]

thread '[ui] ui/specialization/issue-38091.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13


---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----

error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:1: unexpected error: '1:1: 1:1: overflow evaluating the requirement `RootDatabase: SourceDatabase` [E0275]'
error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:69: expected error not found: overflow

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:1: overflow evaluating the requirement `RootDatabase: SourceDatabase` [E0275]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] ui/traits/cycle-cache-err-60010.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---- [ui] ui/traits/multidispatch-convert-ambig-dest.rs stdout ----

error: /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26: unexpected error: '26:5: 26:9: type annotations needed [E0283]'
error: /checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs:26: expected error not found: type annotations needed [E0282]

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/multidispatch-convert-ambig-dest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/multidispatch-convert-ambig-dest/auxiliary"
    Error {
        line_num: 26,
        kind: Some(
            Error,
            Error,
        ),
        msg: "26:5: 26:9: type annotations needed [E0283]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 26,
        kind: Some(
            Error,
        ),
        msg: "type annotations needed [E0282]",
]

thread '[ui] ui/traits/multidispatch-convert-ambig-dest.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---
test result: FAILED. 11881 passed; 6 failed; 101 ignored; 0 measured; 0 filtered out; finished in 127.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:51
