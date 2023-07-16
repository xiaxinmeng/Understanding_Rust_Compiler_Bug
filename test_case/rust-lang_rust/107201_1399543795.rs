plain
failures:

---- [ui] tests/ui/async-await/issue-61076.rs stdout ----

error: /checkout/tests/ui/async-await/issue-61076.rs:56: expected note not found: checked the `Output` of this `async fn`, expected opaque type

error: /checkout/tests/ui/async-await/issue-61076.rs:56: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/async-await/issue-61076.rs:56: expected note not found: in this expansion of desugaring of `async` block or function
error: 0 unexpected errors found, 3 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary" "--edition=2018"
    Error {
        line_num: 56,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, expected opaque type",
    Error {
        line_num: 56,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 56,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
]

thread '[ui] tests/ui/async-await/issue-61076.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] tests/ui/suggestions/if-then-neeing-semi.rs stdout ----

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: checked the `Output` of this `async fn`, expected opaque type

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:18: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/if-then-neeing-semi.rs:24: expected note not found: in this expansion of desugaring of `async` block or function
error: 0 unexpected errors found, 12 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/if-then-neeing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-then-neeing-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/if-then-neeing-semi/auxiliary" "--edition=2018"
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, expected opaque type",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 18,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 24,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
]

thread '[ui] tests/ui/suggestions/if-then-neeing-semi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13


---- [ui] tests/ui/suggestions/match-prev-arm-needing-semi.rs stdout ----

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: checked the `Output` of this `async fn`, expected opaque type

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: checked the `Output` of this `async fn`, found opaque type

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: in this expansion of desugaring of `async` block or function

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: while checking the return type of the `async fn`

error: /checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs:22: expected note not found: in this expansion of desugaring of `async` block or function
error: 0 unexpected errors found, 12 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary" "--edition=2018"
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, expected opaque type",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the `Output` of this `async fn`, found opaque type",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "while checking the return type of the `async fn`",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of desugaring of `async` block or function",
]

thread '[ui] tests/ui/suggestions/match-prev-arm-needing-semi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

