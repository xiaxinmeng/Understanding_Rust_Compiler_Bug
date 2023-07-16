plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........i...............................................................................  1144/14691
........................................................................................  1232/14691
........................................................................................  1320/14691
........................................................................................  1408/14691
.......................F................................................................  1496/14691
..................i...F.................................................................  1584/14691
........................................................................................  1760/14691
........................................................................................  1848/14691
........................................................................................  1936/14691
...................................i....................i...........ii..................  2024/14691
---
..................................i.....................................................  7304/14691
......ii.ii.........i...i...............................................................  7392/14691
..........................i.............................................................  7480/14691
........................................................................................  7568/14691
.....................i..F.i..........................................i..................  7656/14691
........................................................................................  7832/14691
....................i...................................................................  7920/14691
........................................................................................  8008/14691
........................................................................................  8096/14691
---
failures:

---- [ui] tests/ui/borrowck/borrowck-vec-pattern-nesting.rs stdout ----

error: /checkout/tests/ui/borrowck/borrowck-vec-pattern-nesting.rs:48: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/borrowck/borrowck-vec-pattern-nesting.rs:68: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/borrowck/borrowck-vec-pattern-nesting.rs:89: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 3 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/borrowck-vec-pattern-nesting.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/auxiliary"
    Error {
        line_num: 48,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 68,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 89,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/borrowck/borrowck-vec-pattern-nesting.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] tests/ui/borrowck/issue-87456-point-to-closure.rs stdout ----

error: /checkout/tests/ui/borrowck/issue-87456-point-to-closure.rs:10: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/borrowck/issue-87456-point-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-87456-point-to-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-87456-point-to-closure/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/borrowck/issue-87456-point-to-closure.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13


---- [ui] tests/ui/fully-qualified-type/fully-qualified-type-name1.rs stdout ----

error: /checkout/tests/ui/fully-qualified-type/fully-qualified-type-name1.rs:6: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/fully-qualified-type/fully-qualified-type-name1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fully-qualified-type/fully-qualified-type-name1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fully-qualified-type/fully-qualified-type-name1/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/fully-qualified-type/fully-qualified-type-name1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13


---- [ui] tests/ui/lifetimes/missing-lifetime-in-alias.rs stdout ----

error: /checkout/tests/ui/lifetimes/missing-lifetime-in-alias.rs:24: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lifetimes/missing-lifetime-in-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/missing-lifetime-in-alias" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/missing-lifetime-in-alias/auxiliary"
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/lifetimes/missing-lifetime-in-alias.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13


---- [ui] tests/ui/match/match-type-err-first-arm.rs stdout ----

error: /checkout/tests/ui/match/match-type-err-first-arm.rs:8: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/match/match-type-err-first-arm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-type-err-first-arm" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-type-err-first-arm/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/match/match-type-err-first-arm.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13


---- [ui] tests/ui/numeric/numeric-suffix/numeric-suffix.rs stdout ----

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:335: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:340: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:345: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:358: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:363: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:368: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:391: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:396: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:413: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:418: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:429: expected note not found: in this expansion of desugaring of a resized `Span`

error: /checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs:434: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 12 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/numeric/numeric-suffix/numeric-suffix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-suffix/numeric-suffix" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-suffix/numeric-suffix/auxiliary"
    Error {
        line_num: 335,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 340,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 345,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 358,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 363,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 368,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 391,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 396,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 413,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 418,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 429,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
    Error {
        line_num: 434,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/numeric/numeric-suffix/numeric-suffix.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13


---- [ui] tests/ui/type/type-check/point-at-inference-4.rs stdout ----

error: /checkout/tests/ui/type/type-check/point-at-inference-4.rs:12: expected note not found: in this expansion of desugaring of a resized `Span`
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/type-check/point-at-inference-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/point-at-inference-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/point-at-inference-4/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of a resized `Span`",
]

thread '[ui] tests/ui/type/type-check/point-at-inference-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1448:13

