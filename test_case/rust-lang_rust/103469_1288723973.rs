plain
failures:

---- [ui] src/test/ui/borrowck/borrowck-mutate-in-guard.rs stdout ----

error: /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:10: unexpected error: '10:25: 10:43: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:12: unexpected error: '12:33: 12:39: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:10: expected error not found: cannot assign `x` in match guard

error: /checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs:12: expected error not found: cannot mutably borrow `x` in match guard
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-mutate-in-guard.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mutate-in-guard" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-mutate-in-guard/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:25: 10:43: cannot mutably borrow {$place_desc} in  [E0510]",
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:33: 12:39: cannot mutably borrow {$place_desc} in  [E0510]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 10,
        kind: Some(
            Error,
        ),
        msg: "cannot assign `x` in match guard",
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
        msg: "cannot mutably borrow `x` in match guard",
]

thread '[ui] src/test/ui/borrowck/borrowck-mutate-in-guard.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs stdout ----

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:36: unexpected error: '36:12: 36:17: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:50: unexpected error: '50:12: 50:17: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:64: unexpected error: '64:12: 64:17: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:71: unexpected error: '71:12: 71:17: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:36: expected error not found: cannot assign `x` in indexing expression

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:50: expected error not found: cannot assign `x` in indexing expression

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:64: expected error not found: cannot assign `x` in indexing expression

error: /checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs:71: expected error not found: cannot assign `x` in indexing expression
error: 4 unexpected errors found, 4 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/slice-index-bounds-check-invalidation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/slice-index-bounds-check-invalidation/auxiliary"
    Error {
        line_num: 36,
        kind: Some(
            Error,
            Error,
        ),
        msg: "36:12: 36:17: cannot mutably borrow {$place_desc} in  [E0510]",
    Error {
        line_num: 50,
        kind: Some(
            Error,
            Error,
        ),
        msg: "50:12: 50:17: cannot mutably borrow {$place_desc} in  [E0510]",
    Error {
        line_num: 64,
        kind: Some(
            Error,
            Error,
        ),
        msg: "64:12: 64:17: cannot mutably borrow {$place_desc} in  [E0510]",
    Error {
        line_num: 71,
        kind: Some(
            Error,
            Error,
        ),
        msg: "71:12: 71:17: cannot mutably borrow {$place_desc} in  [E0510]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 36,
        kind: Some(
            Error,
        ),
        msg: "cannot assign `x` in indexing expression",
    Error {
        line_num: 50,
        kind: Some(
            Error,
            Error,
        ),
        msg: "cannot assign `x` in indexing expression",
    Error {
        line_num: 64,
        kind: Some(
            Error,
            Error,
        ),
        msg: "cannot assign `x` in indexing expression",
    Error {
        line_num: 71,
        kind: Some(
            Error,
            Error,
        ),
        msg: "cannot assign `x` in indexing expression",
]

thread '[ui] src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-1.rs stdout ----

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-1.rs:21: unexpected error: '21:14: 21:16: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-1.rs:21: expected error not found: cannot mutably borrow `x` in match guard [E0510]
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-1/auxiliary"
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:14: 21:16: cannot mutably borrow {$place_desc} in  [E0510]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 21,
        kind: Some(
            Error,
        ),
        msg: "cannot mutably borrow `x` in match guard [E0510]",
]

thread '[ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-2.rs stdout ----

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-2.rs:26: unexpected error: '26:18: 26:20: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-2.rs:26: expected error not found: cannot mutably borrow `x` in match guard [E0510]
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-2/auxiliary"
    Error {
        line_num: 26,
        kind: Some(
            Error,
            Error,
        ),
        msg: "26:18: 26:20: cannot mutably borrow {$place_desc} in  [E0510]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 26,
        kind: Some(
            Error,
        ),
        msg: "cannot mutably borrow `x` in match guard [E0510]",
]

thread '[ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-3.rs stdout ----

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-3.rs:20: unexpected error: '20:14: 20:16: cannot mutably borrow {$place_desc} in  [E0510]'

error: /checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-3.rs:20: expected error not found: cannot mutably borrow `x` in match guard [E0510]
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-27282-mutate-before-diverging-arm-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27282-mutate-before-diverging-arm-3/auxiliary"
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:14: 20:16: cannot mutably borrow {$place_desc} in  [E0510]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 20,
        kind: Some(
            Error,
        ),
        msg: "cannot mutably borrow `x` in match guard [E0510]",
]

thread '[ui] src/test/ui/nll/issue-27282-mutate-before-diverging-arm-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

