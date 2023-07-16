plain
failures:

---- [incremental] src/test/incremental/const-generics/hash-tyvid-regression-1.rs stdout ----

error in revision `cfail`: /checkout/src/test/incremental/const-generics/hash-tyvid-regression-1.rs:10: expected error not found: the trait bound

error in revision `cfail`: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/const-generics/hash-tyvid-regression-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1/hash-tyvid-regression-1.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/const-generics/hash-tyvid-regression-1/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Error,
