plain
...............................iii.................................................................. 12700/12736
....................................
failures:

---- [ui] ui/rfc-2632-const-trait-impl/staged-api.rs#unstable stdout ----

error in revision `unstable`: /checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs:40: expected error not found: not yet stable as a const fn

error in revision `unstable`: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unstable" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.unstable/auxiliary"
    Error {
        line_num: 40,
        kind: Some(
            Error,
            Error,
        ),
        msg: "not yet stable as a const fn",
]


thread '[ui] ui/rfc-2632-const-trait-impl/staged-api.rs#unstable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] ui/rfc-2632-const-trait-impl/staged-api.rs#stable stdout ----


error in revision `stable`: /checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs:18: unexpected error: '18:1: 21:2: implementation has missing const stability attribute'

error in revision `stable`: /checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs:18: expected error not found: trait implementations cannot be const stable yet

error in revision `stable`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/staged-api.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stable" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.stable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/staged-api.stable/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
---
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "trait implementations cannot be const stable yet",
]


thread '[ui] ui/rfc-2632-const-trait-impl/staged-api.rs#stable' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13

failures:
    [ui] ui/rfc-2632-const-trait-impl/staged-api.rs#stable
    [ui] ui/rfc-2632-const-trait-impl/staged-api.rs#unstable
