plain
failures:

---- [ui] src/test/ui/transmutability/issues-101739.rs stdout ----

error: /checkout/src/test/ui/transmutability/issues-101739.rs:8: unexpected error: '8:9: 8:12: cannot find type `Dst` in this scope [E0412]'
error: /checkout/src/test/ui/transmutability/issues-101739.rs:8: unexpected error: '8:50: 8:66: mismatched types [E0308]'

error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/issues-101739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/issues-101739" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/issues-101739/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:9: 8:12: cannot find type `Dst` in this scope [E0412]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
