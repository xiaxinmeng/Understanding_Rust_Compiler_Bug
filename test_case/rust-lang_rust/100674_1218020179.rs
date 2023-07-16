plain
failures:

---- [ui] src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs stdout ----

error: /checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs:11: unexpected error: '11:1: 12:33: unused extern crate [unused_extern_crates]'
error: /checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs:12: expected error not found: unused extern crate

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--cfg" "blandiloquence" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:1: 12:33: unused extern crate [unused_extern_crates]",
]

not found errors (from test file): [
    Error {
