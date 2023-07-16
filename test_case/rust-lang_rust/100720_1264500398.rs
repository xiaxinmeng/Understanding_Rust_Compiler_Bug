plain
failures:

---- [incremental] src/test/incremental/issue-61323.rs stdout ----

error in revision `cfail`: /checkout/src/test/incremental/issue-61323.rs:12: unexpected error: '12:1: 12:9: recursive type `C` has infinite size [E0072]'

error in revision `cfail`: /checkout/src/test/incremental/issue-61323.rs:12: unexpected error: '12:1: 12:9: recursive type `C` has infinite size [E0072]'

error in revision `cfail`: /checkout/src/test/incremental/issue-61323.rs:12: unexpected error: '12:1: 12:9: recursive type `C` has infinite size [E0072]'

error in revision `cfail`: /checkout/src/test/incremental/issue-61323.rs:3: expected error not found: 3:1: 3:7: recursive types `A` and `C` have infinite size [E0072]

error in revision `cfail`: 3 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-61323.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-61323/issue-61323.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-61323" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-61323/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
---
        line_num: 3,
        kind: Some(
            Error,
        ),
        msg: "3:1: 3:7: recursive types `A` and `C` have infinite size [E0072]",
]

thread '[incremental] src/test/incremental/issue-61323.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
