plain
[00:50:41] ..............................................................i.....................................
[00:50:45] ....................................................................................................
[00:50:51] ....................................................................................................
[00:50:58] ...........................................................................................i........
[00:51:00] .........iiiiiiiii...................................................
[00:51:00] 
[00:51:00] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:51:50] ..............................................................i.....................................
[00:51:54] ....................................................................................................
[00:51:59] ....................................................................................................
[00:52:05] ...........................................................................................i........
[00:52:08] .........iiiiiiiii...................................................
[00:52:08] 
[00:52:08]  finished in 67.509
[00:52:08] travis_fold:end:test_ui_nll

---
[01:01:15] running 2420 tests
[01:01:19] ....................................................................................................
[01:01:24] ....................................................................................................
[01:01:28] ....................................................................................................
[01:01:33] .................................................................F..................................
[01:01:40] ....................................................................F..................i............
[01:01:50] ...............................................................................i....................
[01:01:55] ........................i...........................................................................
[01:01:55] ........................i...........................................................................
[01:01:59] ...........................................................F..............................F....i....
[01:02:03] ..............F.........F...........................................................................
[01:02:09] ....................................................................................................
[01:02:15] ..F..........................................................................................F......
[01:02:20] ....................................................................................................
[01:02:27] ...............................................................F....................................
[01:02:39] ...i................................................................................................
[01:02:45] ..i..ii.............................................................................................
[01:02:53] ....................................................................................................
[01:02:58] ....................................................................................................
[01:02:58] ....................................................................................................
[01:03:02] ..............................................................................i.....................
[01:03:07] ........................i.....................................F.....................................
[01:03:13] ................F...................................................................................
[01:03:22] ....................
[01:03:22] failures:
[01:03:22] 
[01:03:22] ---- [compile-fail] compile-fail/check-static-recursion-foreign.rs stdout ----
[01:03:22] ---- [compile-fail] compile-fail/check-static-recursion-foreign.rs stdout ----
[01:03:22] 
[01:03:22] error: aux-build `/checkout/src/test/compile-fail/auxiliary/check_static_recursion_foreign_helper.rs` source not found
[01:03:22] thread '[compile-fail] compile-fail/check-static-recursion-foreign.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1928:9
[01:03:22] 
[01:03:22] ---- [compile-fail] compile-fail/const-fn-not-safe-for-const.rs stdout ----
[01:03:22] 
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs:30: expected error not found: cannot refer to statics by value
[01:03:22] 
[01:03:22] error: 0 unexpected errors found, 1 expected errors not found
[01:03:22] status: exit code: 101
[01:03:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/auxiliary" "-A" "unused"
[01:03:22] not found errors (from test file): [
[01:03:22]     Error {
[01:03:22]         line_num: 30,
[01:03:22]         kind: Some(
[01:03:22]         ),
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to statics by value"
[01:03:22] ]
[01:03:22] 
[01:03:22] thread '[compile-fail] compile-fail/const-fn-not-safe-for-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:03:22] 
[01:03:22] 
[01:03:22] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:20: 16:26: foreign statics cannot be accessed at compile-time [E0625]'
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-14227.rs:16: expected message not found: cannot refer to other statics by value
[01:03:22] 
[01:03:22] error: 1 unexpected errors found, 1 expected errors not found
[01:03:22] status: exit code: 101
[01:03:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[01:03:22] unexpected errors (from JSON output): [
[01:03:22]     Error {
[01:02] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-17718-references.rs:24: expected message not found: cannot refer to statics
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-17718-references.rs:27: expected error not found: cannot refer to other statics by value
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-17718-references.rs:30: expected error not found: cannot refer to statics by value
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-17718-references.rs:33: expected error not found: cannot refer to other statics by value
[01:03:22] 
[01:03:22] error: 0 unexpected errors found, 4 expected errors not found
[01:03:22] status: exit code: 101
[01:03:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17718-references.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/auxiliary" "-A" "unused"
[01:03:22] not found errors (from test file): [
[01:03:22]     Error {
[01:03:22]         line_num: 24,
[01:03:22]         kind: None,
[01:03:22]         msg: "cannot refer to statics"
[01:03:22]     Error {
[01:03:22]         line_num: 27,
[01:03:22]         kind: Some(
[01:03:22]             Error
[01:03:22]             Error
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to other statics by value"
[01:03:22]     Error {
[01:03:22]         line_num: 30,
[01:03:22]         kind: Some(
[01:03:22]             Error
[01:03:22]             Error
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to statics by value"
[01:03:22]     Error {
[01:03:22]         line_num: 33,
[01:03:22]         kind: Some(
[01:03:22]             Error
[01:03:22]             Error
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to other statics by value"
[01:03:22] ]
[01:03:22] 
[01:03:22] thread '[compile-fail] compile-fail/issue-17718-references.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:03:22] 
[01:03:22] 
[01:03:22] ---- [compile-fail] compile-fail/issue-28324.rs stdout ----
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-28324.rs:17: unexpected error: '17:24: 17:44: foreign statics cannot be accessed at compile-time [E0625]'
[01:03:22] 
[01:03:22] error: /checkout/src/test/compile-fail/issue-28324.rs:17: expected error not found: cannot refer to other statics by value
[01:03:22] 
[01:03:22] error: 1 unexpected errors found, 1 expected errors not found
[01:03:22] status: exit code: 101
[01:03:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-28324.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/ru1:03:22] ------------------------------------------
[01:03:22] ------------------------------------------
[01:03:22] stderr:
[01:03:22] ------------------------------------------
[01:03:22] 
---
[01:03:22] ---- [compile-fail] compile-fail/issue-6991.rs stdout ----
[01:03:22] 
[01:03:22] error: compile-fail test compiled successfully!
[01:03:22] status: exit code: 0
[01:03:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-6991.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-6991/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-6991/auxiliary" "-A" "unused"
[01:03:22] ------------------------------------------
[01:03:22] 
[01:03:22] ------------------------------------------
[01:03:22] stderr:
[01:03:22] stderr:
[01:03:22] ------------------------------------------
[01:03:22] {"message":"static variable `x` should have an upper case name such as `X`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-6991.rs","byte_start":467,"byte_end":497,"line_start":11,"line_end":11,"c  kind: Some(
[01:03:22]         ),
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to other statics by value"
[01:03:22]     Error {
[01:03:22]         line_num: 23,
[01:03:22]         kind: Some(
[01:03:22]             Error
[01:03:22]             Error
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to statics by value"
[01:03:22]     Error {
[01:03:22]         line_num: 31,
[01:03:22]         kind: Some(
[01:03:22]             Error
[01:03:22]             Error
[01:03:22]         ),
[01:03:22]         msg: "cannot refer to statics by value"
[01:03:22] ]
[01:03:22] 
[01:03:22] thread '[compile-fail] compile-fail/thread-local-in-ctfe.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:03:22] 
