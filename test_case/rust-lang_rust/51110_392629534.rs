plain
[00:48:08] ..............................................................i.....................................
[00:48:12] ....................................................................................................
[00:48:18] ....................................................................................................
[00:48:24] ...........................................................................................i........
[00:48:27] .........iiiiiiiii...................................................
[00:48:27] 
[00:48:27] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:49:15] ..............................................................i.....................................
[00:49:19] ....................................................................................................
[00:49:24] ....................................................................................................
[00:49:30] ...........................................................................................i........
[00:49:33] .........iiiiiiiii...................................................
[00:49:33] 
[00:49:33]  finished in 65.597
[00:49:33] travis_fold:end:test_ui_nll

---
[00:58:19] ....................................................................................................
[00:58:24] ....................................................................................................
[00:58:28] ....................................................................................................
[00:58:33] ....................................................................................................
[00:58:39] ...................................................................F..................i.............
[00:58:49] ..............................................................................i.....................
[00:58:54] .......................i............................................................................
[00:58:54] .......................i............................................................................
[00:58:58] ...........................................................F..................................i.....
[00:59:02] .............F.........F............................................................................
[00:59:08] ....................................................................................................
[00:59:14] .F..........................................................................................F.......
[00:59:20] ....................................................................................................
[00:59:25] ..............................................................F.....................................
[00:59:38] ..i.................................................................................................
[00:59:44] .i..ii..............................................................................................
[00:59:52] ....................................................................................................
[00:59:57] ....................................................................................................
[00:59:57] ....................................................................................................
[01:00:01] .............................................................................i......................
[01:00:06] .......................i.....................................F......................................
[01:00:11] ...............F....................................................................................
[01:00:19] ....................................................................................................
[01:00:20] ...................
[01:00:20] failures:
[01:00:20] 
[01:00:20] 
[01:00:20] ---- [compile-fail] compile-fail/const-fn-not-safe-for-const.rs stdout ----
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs:30: expected error not found: cannot refer to statics by value
[01:00:20] 
[01:00:20] error: 0 unexpected errors found, 1 expected errors not found
[01:00:20] status: exit code: 101
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/auxiliary" "-A" "unused"
[01:00:20] not found errors (from test file): [
[01:00:20]     Error {
[01:00:20]         line_num: 30,
[01:00:20]         kind: Some(
[01:00:20]         ),
[01:00:20]         ),
[01:00:20]         msg: "cannot refer to statics by value"
[01:00:20] ]
[01:00:20] 
[01:00:20] thread '[compile-fail] compile-fail/const-fn-not-safe-for-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:00:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:20] 
[01:00:20] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/issue-14227.rs:16: unexpected error: '16:20: 16:26: constant evaluation error [E0080]'
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/issue-14227.rs:16: expected message not found: cannot refer to other statics by value
[01:00:20] 
[01:00:20] error: 1 unexpected errors found, 1 expected errors not found
[01:00:20] status: exit code: 101
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[01:00:20] unexpected errors (from JSON output): [
[01:00:20]     Error {
[01:00:20]         line_num: 16,
[01:00:20]         kind: Some(
[01:00:20]         ),
[01:00:20]         ),
[01:00:20]         msg: "16:20: 16:26: constant evaluation error [E0080]"
[01:00:20] ]
[01:00:20] 
[01:00:20] not found errors (from test file): [
[01:00:20]     Error {
[01:00:20]     Error {
[01:00:20]         line_num: 16,
[01:00:20]         kind: None,
[01:00:20]         msg: "cannot refer to other statics by value"
-17718-references.rs:27: expected error not found: cannot refer to other statics by value
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/issue-17718-references.rs:30: expected error not found: cannot refer to statics by value
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/issue-17718-references.rs:33: expected error not found: cannot refer to other statics by value
[01:00:20] 
[01:00:20] error: 0 unexpected errors found, 4 expected errors not found
[01:00:20] status: exit code: 101
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17718-references.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/auxiliary" "-A" "unused"
[01:00:20] not found errors (from test file): [
[01:00:20]     Error {
[01:00:20]         line_num: 24,
[01:00:20]         kind: None,
[01:00:20]         msg: "cannot refer to statics"
[01:00:20]     Error {
[01:00:20]         line_num: 27,
[01:00:20]         kind: Some(
[01:00:20]             Error
[01:00:20]             Error
[01:00:20]         ),
[01:00:20]         msg: "cannot refer to other statics by value"
[01:00:20]     Error {
[01:00:20]         line_num: 30,
[01:00:20]         kind: Some(
[01:00:20]             Error
[01:00:20]             Error
#[warn(non_upper_case_globals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: static variable `x` should have an upper case name such as `X`\n  --> /checkout/src/test/compile-fail/issue-6991.rs:11:1\n   |\nLL | static x: &'static usize = &1;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(non_upper_case_globals)] on by default\n\n"}
[01:00:20] {"message":"static variable `y` should have an upper case name such as `Y`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-6991.rs","byte_start":498,"byte_end":519,"line_start":12,"line_end":12,"column_start":1,"column_end":22,"is_primary":true,"text":[{"text":"static y: usize = *x;","highlight_start":1,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: static variable `y` should have an upper case name such as `Y`\n  --> /checkout/src/test/compile-fail/issue-6991.rs:12:1\n   |\nLL | static y: usize = *x;\n   | ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:00:20] ------------------------------------------
[01:00:20] 
[01:00:20] thread '[compile-fail] compile-fail/issue-6991.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:00:20] 
[01:00:20] 
[01:00:20] ---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----
[01:00:20] 
[01:00:20] error: compiler panicked
[01:00:20] status: exit code: 101
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src-local-in-ctfe.rs stdout ----
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:16: expected error not found: cannot refer to other statics by value
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:23: expected error not found: cannot refer to statics by value
[01:00:20] 
[01:00:20] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:31: expected error not found: cannot refer to statics by value
[01:00:20] 
[01:00:20] error: 0 unexpected errors found, 3 expected errors not found
[01:00:20] status: exit code: 101
[01:00:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/thread-local-in-ctfe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/auxiliary" "-A" "unused"
[01:00:20] not found errors (from test file): [
[01:00:20]     Error {
[01:00:20]         line_num: 16,
[01:00:20]         kind: Some(
[01:00:20]         ),
[01:00:20]         ),
[01:00:20]         msg: "cannot refer to other statics by value"
[01:00:20]     Error {
[01:00:20]         line_num: 23,
[01:00:20]         kind: Some(
[01:00:20]             Error
[01:00:20]             Error
[01:00:20]         ),
[01:00:20]         msg: "cannot refer to statics by valu[32;1mThe command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
travis_fold:start:after_failure.1
travis_time:start:0f3f54e5
