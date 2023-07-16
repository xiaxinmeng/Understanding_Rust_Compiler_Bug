plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:41:48] 
[00:41:48] running 2118 tests
[00:41:51] ......................................................................................F...........i.
[00:41:58] ....................................................................................................
[00:42:00] ....................................................................................................
[00:42:02] ....................................................................................................
[00:42:04] ....................................................................................................
[00:42:04] ....................................................................................................
[00:42:07] ............................F.......................................................................
[00:42:10] .........................................................................................F..........
[00:42:13] ......................................Fi............................................................
[00:42:18] ....................................................................................................
[00:42:18] ....................................................................................................
[00:42:20] ............................................................F.......................................
[00:42:23] ....................................................................................................
[00:42:26] .....................................................................................F..............
[00:42:33] ............................................................................................i.......
[00:42:36] ..................................................................................i.................
[00:42:39] ....................................................................................................
[00:42:43] ....................................................................................................
[00:42:43] ....................................................................................................
[00:42:46] ....................................................................................................
[00:42:50] .................................i..................................................................
[00:42:51] .............i....
[00:42:51] failures:
[00:42:51] 
[00:42:51] ---- [ui] ui/codemap_tests/bad-format-args.rs stdout ----
[00:42:51] 
[00:42:51] error: /checkout/src/test/ui/codemap_tests/bad-format-args.rs:12: unexpected error: '12:5: 12:15: requires at least a format string argument'
[00:42:51] error: /checkout/src/test/ui/codemap_tests/bad-format-args.rs:13: unexpected error: '13:5: 13:19: expected token: `,`'
[00:42:51] 
[00:42:51] error: 2 unexpected errors found, 0 expected errors not found
[00:42:51] status: exit code: 101
[00:42:51] status: exit code: 101
[00:42:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/bad-format-args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/bad-format-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/bad-format-args/auxiliary" "-A" "unused"
[00:42:51]     Error {
[00:42:51]         line_num: 12,
[00:42:51]         kind: Some(
[00:42:51]             Error
[00:42:51]             Error
[00:42:51]         ),
[00:42:51]         msg: "12:5: 12:15: requires at least a format string argument"
[00:42:51]     Error {
[00:42:51]         line_num: 13,
[00:42:51]         kind: Some(
[00:42:51]             Error
---
[00:42:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:51] 
[00:42:51] ---- [ui] ui/fmt/format-string-error.rs stdout ----
[00:42:51] 
[00:42:51] error: /checkout/src/test/ui/fmt/format-string-error.rs:12: unexpected error: '12:5: 12:19: invalid format string: expected `'}'` but string was terminated'
[00:42:51] 
[00:42:51] error: /checkout/src/test/ui/fmt/format-string-error.rs:12: expected error not found: invalid format string: expected `\'}\'` but string was terminated
[00:42:51] error: 1 unexpected errors found, 1 expected errors not found
[00:42:51] status: exit code: 101
[00:42:51] status: exit code: 101
[00:42:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error/auxiliary" "-A" "unused"
[00:42:51]     Error {
[00:42:51]         line_num: 12,
[00:42:51]         kind:            Error
[00:42:51]         ),
---
[00:42:51] thread '[ui] ui/issue-13446.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:42:51] 
[00:42:51] ---- [ui] ui/issue-16966.rs stdout ----
[00:42:51] 
[00:42:51] error: /checkout/src/test/ui/issue-16966.rs:13: unexpected error: '13:5: 15:7: type annotations needed [E0282]'
[00:42:51] error: 1 unexpected errors found, 0 expected errors not found
[00:42:51] status: exit code: 101
[00:42:51] status: exit code: 101
[00:42:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-16966.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-16966/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-16966/auxiliary" "-A" "unused"
[00:42:51]     Error {
[00:42:51]         line_num: 13,
[00:42:51]         kind: Some(
[00:42:51]             Error
---
[00:42:51] thread '[ui] ui/issue-16966.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:42:51] 
[00:42:51] ---- [ui] ui/issue-32829.rs stdout ----
[00:42:51] 
[00:42:51] error: /checkout/src/test/ui/issue-328rror 1
[00:42:51] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0029df84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
