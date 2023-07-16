plain
failures:

---- [ui] tests/ui/const-generics/nested-type.rs#min stdout ----

error in revision `min`: /checkout/tests/ui/const-generics/nested-type.rs:6: unexpected error: '6:21: 17:3: `[u8; {'

error in revision `min`: /checkout/tests/ui/const-generics/nested-type.rs:6: expected error not found: `[u8; _]` is forbidden

error in revision `min`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/nested-type.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/nested-type.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/nested-type.min/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:21: 17:3: `[u8; {",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`[u8; _]` is forbidden",
]


thread '[ui] tests/ui/const-generics/nested-type.rs#min' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1454:13

---- [ui] tests/ui/fmt/format-string-error.rs stdout ----


error: /checkout/tests/ui/fmt/format-string-error.rs:19: unexpected error: '19:23: 19:23: invalid format string: expected `'}'`, found `'\'`'

error: /checkout/tests/ui/fmt/format-string-error.rs:19: expected error not found: invalid format string: expected `'}'`, found `'\\'`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/fmt/format-string-error.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/auxiliary"
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "19:23: 19:23: invalid format string: expected `'}'`, found `'\\'`",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 19,
        kind: Some(
            Error,
        ),
        msg: "invalid format string: expected `'}'`, found `'\\\\'`",
]

thread '[ui] tests/ui/fmt/format-string-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1454:13


---- [ui] tests/ui/parser/issues/issue-62913.rs stdout ----

error: /checkout/tests/ui/parser/issues/issue-62913.rs:1: unexpected error: '1:1: 1:7: expected item, found `"\u\"`'

error: /checkout/tests/ui/parser/issues/issue-62913.rs:1: expected error not found: expected item, found `"\u\\"`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issues/issue-62913.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62913" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-62913/auxiliary"
    Error {
        line_num: 1,
        kind: Some(
            Error,
            Error,
        ),
        msg: "1:1: 1:7: expected item, found `\"\\u\\\"`",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "expected item, found `\"\\u\\\\\"`",
]

thread '[ui] tests/ui/parser/issues/issue-62913.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1454:13

