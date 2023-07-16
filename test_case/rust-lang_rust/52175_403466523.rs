plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:07] 
[00:43:07] running 1554 tests
[00:43:12] ........................................................................F..............F..........i.
[00:43:19] ..............................................................i.FF...........F......................
[00:43:23] ........F..................F...F....................................................................
[00:43:27] ....................................................................................................
[00:43:30] ........................................................................................FF..........
[00:43:34] ....................................................................................................
[00:43:39] ...........................F......................F...F.FF.F.FFF..F...FF............................
[00:43:44] .F......................................................................FF..F.......................
[00:43:50] ....F....................F.................................................F........................
[00:43:55] ..........................................F..........F................F.........F...F.F..F.FF..FF.F.
[00:44:02] .................F................i.................................................................
[00:44:07] .......................i...............................................F...F....FF.F.......F........
[00:44:13] ....................................................................................................
[00:44:19] ...............................................F..........F...........F......................F......
[00:44:26] ....................................................................i.......F.......................
[00:44:29] ....FF.....FF......................F..................
[00:44:29] 
[00:44:29] ---- [ui] ui/catch-block-type-error.rs stdout ----
[00:44:29] 
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/catch-block-type-error.rs:22: unexpected error: '22:35: 24:6: type mismatch resolving `<std::option::Option<i32> as std::ops::Try>::Ok == ()` [E0271]'
[00:44:29] error: /checkout/src/test/ui/catch-block-type-error.rs:24: expected error not found: type mismatch
[00:44:29] 
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/catch-block-type-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-block-type-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/catch-block-type-error/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 22,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "22:35: 24:6: type mismatch resolving `<std::option::Option<i32> as std::ops::Try>::Ok == ()` [E0271]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 24,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type mismatch"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/catch-block-type-error.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:29] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:29] 
[00:44:29] ---- [ui] ui/codemap_tests/bad-format-args.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/codemap_tests/bad-format-args.rs:12: unexpected error: '12:5: 12:15: requires at least a format string argument'
[00:44:29] error: /checkout/src/test/ui/codemap_tests/bad-format-args.rs:13: unexpected error: '13:5: 13:19: expected token: `,`'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/codemap_tests/bad-format-args.rs:14: unexpected error: '14:5: 14:22: expected token: `,`'
[00:44:29] 
[00:44:29] 
[00:44:29] error: 3 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/bad-format-args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/bad-format-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/bad-format-args/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 12,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "12:5: 12:15: requires at least a format string argument"
[00:44:29]     Error {
[00:44:29]         line_num: 13,
[00:44:29]         kind: Some(
[00:44:29]             Error
---
[00:44:29] thread '[ui] ui/codemap_tests/bad-format-args.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/cross-crate-macro-backtrace/main.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/cross-crate-macro-backtrace/main.rs:18: unexpected error: '18:5: 18:22: 1 positional argument in format string, but no arguments were given'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross-crate-macro-backtrace/main.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate-macro-backtrace/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-crate-macro-backtrace/main/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 18,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "18:5: 18:22: 1 positional argument in format string, but no arguments were given"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/cross-crate-macro-backtrace/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/cross-file-errors/main.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/cross-file-errors/main.rs:15: unexpected error: '15:5: 15:19: expected expression, found reserved identifier `_`'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross-file-errors/main.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-file-errors/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross-file-errors/main/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 15,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "15:5: 15:19: expected expression, found reserved identifier `_`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/cross-file-errors/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
---
[00:44:29] error: /checkout/src/test/ui/deref-suggestion.rs:12: expected error not found: mismatched types
[00:44:29] 
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deref-suggestion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dlinux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 24,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "24:5: 24:55: recursion limit reached while expanding the macro `recurse`"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 20,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "recursion limit"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/did_you_mean/recursion_limit_macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/edition-keywords-2015-2018-expansion.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/edition-keywords-2015-2018-expansion.rs:20: unexpected error: '20:5: 20:23: expected identifier, found reserved keyword `async`'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkoutknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/edition-keywords-2018-2018-expansion/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 20,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "20:5: 20:23: expected identifier, found reserved keyword `async`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/edition-keywords-2018-2018-expansion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/feature-gate-allow-internal-unsafe-nested-macro.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/feature-gate-allow-internal-unsafe-nested-macro.rs:25: unexpected error: '25:1: 25:8: allow_internal_unsafe side-steps the unsafe_code lint [E0658]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/feature-gate-allow-internal-unsafe-nested-macro.rs:18: expected error not found: allow_internal_unsafe side-steps
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-allow-internal-unsafe-nested-macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/chec      msg: "49:5: 49:23: method `method` is not a member of trait `Tr` [E0407]"
[00:44:29]     Error {
[00:44:29]         line_num: 49,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "49:5: 49:23: not all trait items implemented, missing: `method` [E0046]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 44,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "not all trait items implemented, missing: `method`"
[00:44:29]     Error {
[00:44:29]         line_num: 45,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "method `method` is not a member of trait `Tr`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/assoc_item_ctxt.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/fields-definition.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields-definition.rs:30: unexpected error: '30:1: 30:12: field `a` is already declared [E0124]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields-definition.rs:24: expected error not found: field `a` is already declared
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields-definition.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 30,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "30:1: 30:12: field `a` is already declared [E0124]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 24,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "field `a` is already declared"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/fields-definition.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/fields-move.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields-move.rs:39: unexpected error: '39:42: 39:47: use of moved value: `foo.x` [E0382]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields-move.rs:28: expected error not found: use of moved value: `foo.x`
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields-move.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-move/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-move/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 39,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "39:42: 39:47: use of moved value: `foo.x` [E0382]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 28,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "use of moved value: `foo.x`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/fields-move.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/fields.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:38: unexpected error: '38:13: 38:26: type `foo::S` is private'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:38: unexpected error: '38:13: 38:26: type `foo::S` is private'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:38: unexpected error: '38:13: 38:26: type `foo::T` is private'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:38: unexpected error: '38:13: 38:26: type `foo::T` is private'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:25: expected error not found: type `foo::S` is private
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:26: expected error not found: type `foo::S` is private
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:28: expected error not found: type `foo::T` is private
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/fields.rs:29: expected error not found: type `foo::T` is private
[00:44:29] error: 4 unexpected errors found, 4 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 38,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "38:13: 38:26: type `foo::S` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 38,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "38:13: 38:26: type `foo::S` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 38,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "38:13: 38:26: type `foo::T` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 38,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "38:13: 38:26: type `foo::T` is private"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 25,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type `foo::S` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 26,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type `foo::S` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 28,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type `foo::T` is private"
[00:44:29]     Error {
[00:44:29]         line_num: 29,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type `foo::T` is private"
[00:44:29] ]
9,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "29:5: 33:6: cannot find function `g` in this scope [E0425]"
[00:44:29]     Error {
[00:44:29]         line_num: 36,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "36:1: 36:7: cannot find function `f` in this scope [E0425]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 25,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "cannot find function `g` in this scope"
[00:44:29]     Error {
[00:44:29]         line_num: 64,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "cannot find function `f` in this scope"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/globs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/impl_items.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/impl_items.rs:43: unexpected error: '43:5: 43:15: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/impl_items.rs:22: expected error not found: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/impl_items.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 43,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "43:5: 43:15: type `for<\'r> fn(&\'r foo::S) {foo::S::f}` is private"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 22,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "type `for<\'r> fn(&\'r foo::S) {foo::S::f}` is private"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/impl_items.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/local_inner_macros_disabled.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/local_inner_macros_disabled.rs:19: unexpected error: '19:1: 19:17: cannot find macro `helper2!` in this scope'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] statuso" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 22,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "22:16: 22:37: type `fn() -> u32 {intercrate::foo::bar::f}` is private"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/hygiene/intercrate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/hygiene/no_implicit_prelude.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:15: unexpected error: '15:14: 15:26: failed to resolve. Use of undeclared type or module `Vec` [E0433]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:15: unexpected error: '15:14: 15:26: no method named `clone` found for type `()` in the current scope [E0599]'
[00:44:29] error: /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:21: expected error not found: failed to resolve
[00:44:29] 
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/hygiene/no_implicit_prelude.rs:22: expected error not found: no method named `clone` found
[00:44:29] error: 2 unexpected errors found, 2 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkoanic` is already in scope"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 42,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "`panic` is already in scope"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/imports/shadow_builtin_macros.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/issue-25385.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-25385.rs:19: unexpected error: '19:5: 19:13: no method named `foo` found for type `i32` in the current scope [E0599]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-25385.rs:13: expected error not found: no method named `foo` found for type `i32` in the current scope
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-25385.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-25385/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-25385/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 19,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "19:5: 19:13: no method named `foo` found for type `i32` in the current scope [E0599]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 13,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "no method named `foo` found for type `i32` in the current scope"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/issue-25385.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/issue-25793.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-25793.rs:29: unexpected error: '29:20: 29:32: cannot use `self.width` because it was mutably borrowed [E0503]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-25793.rs:13: expected error not found: cannot use `self.width` because it was mutably borrowed
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-25793.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-25793/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-25793/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 29,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "29:20: 29:32: cannot use `self.width` because it was mutably borrowed [E0503]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 13,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "cannot use `self.width` because it was mutably borrowed"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/issue-25793.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/issue-26093.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-26093.rs:19: unexpected error: '19:5: 19:22: invalid left-hand side expression [E0070]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-26093.rs:13: expected error not found: invalid left-hand side expression
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-26093.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26093/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x/issue-32950/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-32950/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 13,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "13:10: 13:15: `derive` cannot be used on items with type macros"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 15,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "`derive` cannot be used on items with type macros"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/issue-32950.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/issue-42954.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-42954.rs:23: unexpected error: '23:5: 23:30: `<` is interpreted as a start of generic arguments for `u32`, not a comparison'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-42954.rs:17: expected message not found: `<` is interpreted as a start of generic arguments
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-42954.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-42954/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-42954/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 23,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "23:5: 23:30: `<` is interpreted as a start of generic arguments for `u32`, not a comparison"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 17,
[00:44:29]         kind: None,
[00:44:29]         msg: "`<` is interpreted as a start of generic arguments"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/issue-42954.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/issue-50577.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/issue-50577.rs:13: unexpected error: '13:16: 13:32: if may be missing an else clause [E0317]'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-50577.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dyet-suggestion/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 12,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "12:17: 12:24: borrowed value does not live long enough [E0597]"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/lifetimes/borrowck-let-suggestion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/lint/issue-47775-nested-macro-unnecessary-parens-arg.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/lint/issue-47775-nested-macro-unnecessary-parens-arg.rs:38: unexpected warning: '38:1: 38:31: unnecessary parentheses around function argument [unused_parens]'
[00:44:29] error: /checkout/src/test/ui/lint/issue-47775-nested-macro-unnecessary-parens-arg.rs:32: expected warning not found: unnecessary parentheses around function argument
[00:44:29] 
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 0
[00:44:29] status: exit code: 0
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-47775-nested-macro-unnecessary-parens-arg.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47775-nested-macro-unnecessary-parens-arg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/bur-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness-return-last-stmt-semi/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness-return-last-stmt-semi/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 27,
[00:44:29]         kind: Some(
[00:44:29]             Error
---
[00:44:29] thread '[ui] ui/liveness-return-last-stmt-semi.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/macro-context.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macro-context.rs:26: unexpected error: '26:5: 26:10: expected expression, found reserved keyword `typeof`'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macro-context.rs:13: expected error not found: expected expression, found reserved keyword `typeof`
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macro-context.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-context/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macro-context/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 26,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "26:5: 26:10: expected expression, found reserved keyword `typeof`"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 13,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "expected expression, found reserved keyword `typeof`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/macro-context.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/macro-shadowing.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macro-shadowing.rs:25: unexpected error: '25:1: 25:7: `macro_two` is already in scope'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macro-shadowing.rs:25: unexpected error: '25:1: 25:7: `foo` is already in scope'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macro-shadowing.rs:20: expected error not found: `foo` is already 4:29]         line_num: 28,
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         ),
[00:44:29]         msg: "28:5: 28:22: 1 positional argument in format string, but no arguments were given"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 24,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "no arguments were given"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/macros/macro-backtrace-println.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/macros/macro-backtrace-nested.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-nested.rs:28: unexpected error: '28:9: 28:28: cannot find value `fake` in this scope [E0425]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-nested.rs:29: unexpected error: '29:5: 29:29: cannot find value `fake` in this scope [E0425]'
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-nested.rs:15: expected error not found: cannot find
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-nested.rs:15: expected error not found: cannot find
[00:44:29] 
[00:44:29] 
[00:44:29] error: 2 unexpected errors found, 2 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-backtrace-nested.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-nested/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-nested/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 28,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "28:9: 28:28: cannot find value `fake` in this scope [E0425]"
[00:44:29]     Error {
[00:44:29]         line_num: 29,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "29:5: 29:29: cannot find value `fake` in this scope [E0425]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
---
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/macros/macro-backtrace-nested.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/macros/macro-backtrace-invaneg` on ambiguous numeric type `{float}` [E0689]'
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:15: expected error not found: no method
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:21: expected error not found: doesn't have fields
[00:44:29] 
---
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:39: expected error not found: doesn't have fields
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:45: expected error not found: doesn't have fields
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:51: expected error not found: can't call method `neg` on ambiguous numeric type `{float}`
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs:57: expected error not found: can't call method `neg` on ambiguous numeric type `{float}`
[00:44:29] error: 8 unexpected errors found, 8 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-backtrace-invalid-internals.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-invalid-internals/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-backtrace-invalid-internals/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 62,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "62:5: 62:25: no method named `fake` found for type `{integer}` in the current scope [E0599]"
[00:44:29]     Error {
[00:44:29]         line_num: 63,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "63:5: 63:24: `{integer}` is a primitive type and therefore doesn\'t have fields [E0610]"
[00:44:29]     Error {
[00:44:29]         line_num: 64,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "64:5: 64:29: `{integer}` is a primitive type and therefore doesn\'t have fields [E0610]"
[00:44:29]     Error {
[00:44:29]         line_num: 65,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "65:5: 65:25: can\'t call method `neg` on ambiguous numeric type `{float}` [E0689]"
[00:44:29]     Error {
[00:44:29]         line_num: 67,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]           msg: "44:5: 44:27: recursion limit reached while expanding the macro `my_recursive_macro`"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 17,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "no rules"
[00:44:29]     Error {
[00:44:29]         line_num: 32,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "recursion limit"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/macros/trace_faulty_macros.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/macros/span-covering-argument-1.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/span-covering-argument-1.rs:22: unexpected error: '22:5: 22:24: cannot borrow immutable local variable `foo` as mutable [E0596]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/macros/span-covering-argument-1.rs:15: expected error not found: cannot borrow immutable local variable `foo` as mutable [E0596]
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/span-covering-argument-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/span-covering-argument-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/span-covering-argument-1/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 22,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "22:5: 22:24: cannot borrow immutable local variable `foo` as mutable [E0596]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 15,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "cannot borrow immutable local variable `foo` as mutable [E0596]"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/macros/span-covering-argument-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/mismatched_types/issue-26480.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/mismatched_types/issue-26480.rs:37: unexpected error: '37:5: 37:19: mismatched types [E0308]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/mismatched_types/issue-26480.rs:38: unexpected error: '38:5: 38:14: non-primitive cast: `{integer}` as `()` [E0605]'
[00:44:29] error: /checkout/src/test/ui/mismatched_types/issue-26480.rs:26: expected error not found: mismatched types
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/mismatched_types/issue-26480.rs:32: expected error not found: non-primitive cast
[00:44:29] 
[00:44:29] 
[00:44:29] error: 2 unexpected errors found, 2 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-26480.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-26480/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-26480/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 37,
[00:44:29]         kind: Some(
[00:44:29]             Error
---
[00:44:29]         line_num: 38,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "38:5: 38:14: non-primitive cast: `{integer}` as `()` [E0605]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
---
[00:44:29] thread '[ui] ui/mismatched_types/issue-26480.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/reachable/expr_again.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/reachable/expr_again.rs:18: unexpected error: '18:9: 18:24: unreachable statement [unreachable_code]'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_again.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_again/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_again/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 18,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "18:9: 18:24: unreachable statement [unreachable_code]"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/reachable/expr_again.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/reachable/expr_block.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/reachable/expr_block.rs:35: unexpected error: '35:9: 35:25: unreachable statement [unreachable_code]'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_block.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_block/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_block/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 35,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "35:9: 35:25: unreachable statement [unreachable_code]"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/reachable/expr_block.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] 
[00:44:29] ---- [ui] ui/reachable/expr_if.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/reachable/expr_if.rs:37: unexpected error: '37:5: 37:27: unreachable statement [unreachable_code]'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/ch29] ---- [ui] ui/span/issue-33884.rs stdout ----
[00:44:29] 
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/span/issue-33884.rs:18: unexpected error: '18:22: 18:49: mismatched types [E0308]'
[00:44:29] 
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-33884.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-33884/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-33884/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 18,
[00:44:29]         kind: Some(
[00:44:29]             Error
---
[00:44:29] thread '[ui] ui/span/issue-33884.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/span/macro-span-replacement.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/span/macro-span-replacement.rs:22: unexpected warning: '22:5: 22:18: struct is never used: `S` [dead_code]'
[00:44:29] error: /checkout/src/test/ui/span/macro-span-replacement.rs:17: expected warning not found: struct is never used
[00:44:29] 
[00:44:29] 
[cted error: '21:5: 21:29: expected one of `)` or `,`, found `(`'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/tuple-struct-fields/test2.rs:15: expected error not found: expected one of `)` or `,`, found `(`
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] make: *** [check] Error 1
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple-struct-fields/test2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple-struct-fields/test2/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 21,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "21:5: 21:29: expected one of `)` or `,`, found `(`"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 15,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "expected one of `)` or `,`, found `(`"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/tuple-struct-fields/test2.rs' panicked at 'explicit panic', tools/compiletest/socal_or_vec_in_tuples.rs stdout ----
[00:44:29] 
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/type-check/cannot_infer_local_or_vec_in_tuples.rs:12: unexpected error: '12:18: 12:24: type annotations needed [E0282]'
[00:44:29] error: 1 unexpected errors found, 0 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-check/cannot_infer_local_or_vec_in_tuples.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/cannot_infer_local_or_vec_in_tuples/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/cannot_infer_local_or_vec_in_tuples/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 12,
[00:44:29]         kind: Some(
[00:44:29]             Error
---
[00:44:29] thread '[ui] ui/type-check/cannot_infer_local_or_vec_in_tuples.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
[00:44:29] ---- [ui] ui/union/union-derive-eq.rs stdout ----
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/union/union-derive-eq.rs:23: unexpected error: '23:10: 23:12: the trait bound `PartialEqNotEq: std::cmp::Eq` is not satisfied [E0277]'
[00:44:29] 
[00:44:29] error: /checkout/src/test/ui/union/union-derive-eq.rs:25: expected error not found: the trait bound `PartialEqNotEq: std::cmp::Eq` is not satisfied
[00:44:29] error: 1 unexpected errors found, 1 expected errors not found
[00:44:29] status: exit code: 101
[00:44:29] status: exit code: 101
[00:44:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-eq.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-eq/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-eq/auxiliary" "-A" "unused"
[00:44:29]     Error {
[00:44:29]         line_num: 23,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "23:10: 23:12: the trait bound `PartialEqNotEq: std::cmp::Eq` is not satisfied [E0277]"
[00:44:29] ]
[00:44:29] 
[00:44:29] not found errors (from test file): [
[00:44:29]     Error {
[00:44:29]     Error {
[00:44:29]         line_num: 25,
[00:44:29]         kind: Some(
[00:44:29]             Error
[00:44:29]         ),
[00:44:29]         msg: "the trait bound `PartialEqNotEq: std::cmp::Eq` is not satisfied"
[00:44:29] ]
[00:44:29] 
[00:44:29] thread '[ui] ui/union/union-derive-eq.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:44:29] 
---
[00:44:29]     [ui] ui/issue-32950.rs
[00:44:29]     [ui] ui/issue-42954.rs
[00:44:29]     [ui] ui/issue-50577.rs
[00:44:29]     [ui] ui/lifetimes/borrowck-let-suggestion.rs
[00:44:29]     [ui] ui/lint/issue-47775-nested-macro-ully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:29] 
[00:44:29] 
[00:44:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:29] Build completed unsuccessfully in 0:02:00
[00:44:29] Build completed unsuccessfully in 0:02:00
[00:44:29] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c248412
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
