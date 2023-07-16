plain
[00:48:27] .............................................................i......................................
[00:48:31] ....................................................................................................
[00:48:36] ....................................................................................................
[00:48:43] ..........................................................................................i.........
[00:48:45] ........iiiiiiiii...................................................
[00:48:45] 
[00:48:45] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:49:33] .............................................................i......................................
[00:49:37] ....................................................................................................
[00:49:42] ....................................................................................................
[00:49:48] ..........................................................................................i.........
[00:49:51] ........iiiiiiiii...................................................
[00:49:51] 
[00:49:51]  finished in 65.756
[00:49:51] travis_fold:end:test_ui_nll

---
[00:58:39] ....................................................................................................
[00:58:44] ....................................................................................................
[00:58:48] ....................................................................................................
[00:58:53] ....................................................................................................
[00:58:59] ...................................................................F..................i.............
[00:59:09] ..............................................................................i.....................
[00:59:13] .......................i............................................................................
[00:59:13] .......................i............................................................................
[00:59:18] ...........................................................F............................F.....i.....
[00:59:22] .............F......F...F...........................................................................
[00:59:27] ....................................................................................................
[00:59:33] ..F...........................................................................................F.....
[00:59:39] ....................................................................................................
[00:59:44] ...............................................................F....................................
[00:59:57] ...i................................................................................................
[01:00:03] ..i..ii.............................................................................................
[01:00:10] ....................................................................................................
[01:00:15] ....................................................................................................
[01:00:15] ....................................................................................................
[01:00:19] ..............................................................................i.....................
[01:00:24] ........................i.....................................F.....................................
[01:00:29] .................F..................................................................................
[01:00:37] ....................................................................................................
/compiletest/src/runtest.rs:1283:13
[01:00:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:38] 
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[01:00:38] 
[01:00:38] error: compiler encountered internal error
[01:00:38] status: exit code: 101
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] stderr:
[01:00:38] stderr:
[01:00:38] ------------------------------------------
[01:00:38] {"message":"librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_14227[317d]::[0]::symbol[0])","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-14227.rs","byte_start":522,"byte_end":544,"line_start":14,"line_end":14,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub static symbol: ();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: liblinux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-16538.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-16538/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-16538/auxiliary" "-A" "unused"
[01:00:38] not found errors (from test file): [
[01:00:38]     Error {
[01:00:38]         line_num: 23,
[01:00:38]         kind: Some(
[01:00:38]         ),
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to other statics by value, use the address-of operator or a constant instead"
[01:00:38] ]
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/issue-16538.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:00:38] 
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/issue-17450.rs stdout ----
[01:00:38] 
[01:00:38] error: compile-fail test compiled successfully!
[01:00:38] status: exit code: 0
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17450.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17450/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17450/auxiliary" "-A" "unused"
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] stderr:
---
[01:00:38] ---- [compile-fail] compile-fail/issue-17718-borrow-interior.rs stdout ----
[01:00:38] 
[01:00:38] error: compile-fail test compiled successfully!
[01:00:38] status: exit code: 0
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17718-borrow-interior.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-borrow-interior/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-borrow-interior/auxiliary" "-A" "unused"
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] stderr:
[01:00:38] stderr:
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/issue-17718-borrow-interior.rs' pao statics"
[01:00:38]     Error {
[01:00:38]         line_num: 27,
[01:00:38]         kind: Some(
[01:00:38]             Error
[01:00:38]             Error
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to other statics by value"
[01:00:38]     Error {
[01:00:38]         line_num: 30,
[01:00:38]         kind: Some(
[01:00:38]             Error
[01:00:38]             Error
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to statics by value"
[01:00:38]     Error {
[01:00:38]         line_num: 33,
[01:00:38]         kind: Some(
[01:00:38]             Error
[01:00:38]             Error
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to other statics by value"
[01:00:38] ]
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/issue-17718-references.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:00:38] 
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/issue-28324.rs stdout ----
[01:00:38] 
[01:00:38] error: compiler encountered internal error
[01:00:38] status: exit code: 101
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-28324.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/auxiliary" "-A" "unused"
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] stderr:
[01:00:38] stderr:
[01:00:38] ------------------------------------------
[01:00:38] {"message":"librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_28324[317d]::[0]::error_message_count[0])","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-28324.rs","byte_start":512,"byte_end":544,"line_start":14,"line_end":14,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    static error_message_count: u32;","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_28324[317d]::[0]::error_message_count[0])\n  --> /checkout/src/test/compile-fail/issue-28324.rs:14:5\n   |\nLL |     static error_message_count: u32;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:00:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:38] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:38] 
[01:00:38] note: the compiler unexpectedly panicked. this is a bug.
[01:00:38] 
[01:00:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:38] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:00:38] 
[01:00:38] 
[01:00:38] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:00:38] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/issue-34194.rs stdout ----
[01:00:38] 
[01:00:38] error: compile-fail test compiled successfully!
[01:00:38] status: exit code: 0
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-34194.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34194/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34194/auxiliary" "-A" "unused"
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] ------------------------------------------
[01:00:38] stderr:
---
[01:00:38] thread '[compile-fail] compile-fail/issue-34194.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/issue-6991.rs stdout ----
[01:00:38] 
[01:00:38] error: cstatic x: &'static usize = &1;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(non_upper_case_globals)] on by default\n\n"}
[01:00:38] {"message":"static variable `y` should have an upper case name such as `Y`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-6991.rs","byte_start":498,"byte_end":519,"line_start":12,"line_end":12,"column_start":1,"column_end":22,"is_primary":true,"text":[{"text":"static y: usize = *x;","highlight_start":1,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: static variable `y` should have an upper case name such as `Y`\n  --> /checkout/src/test/compile-fail/issue-6991.rs:12:1\n   |\nLL | static y: usize = *x;\n   | ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:00:38] ------------------------------------------
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/issue-6991.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:00:38] 
[01:00:38] 
[01:00:38] ---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----
[01:00:38] 
[01:00:38] error: compiler panicked
[01:00:38] status: exit code: 101
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/static-array-across-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/a" "-Crpath" "-O" "-Zunstable-.rs:23: expected error not found: cannot refer to statics by value
[01:00:38] 
[01:00:38] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:31: expected error not found: cannot refer to statics by value
[01:00:38] 
[01:00:38] error: 0 unexpected errors found, 3 expected errors not found
[01:00:38] status: exit code: 101
[01:00:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/thread-local-in-ctfe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/auxiliary" "-A" "unused"
[01:00:38] not found errors (from test file): [
[01:00:38]     Error {
[01:00:38]         line_num: 16,
[01:00:38]         kind: Some(
[01:00:38]         ),
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to other statics by value"
[01:00:38]     Error {
[01:00:38]         line_num: 23,
[01:00:38]         kind: Some(
[01:00:38]             Error
[01:00:38]             Error
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to statics by value"
[01:00:38]     Error {
[01:00:38]         line_num: 31,
[01:00:38]         kind: Some(
[01:00:38]             Error
[01:00:38]             Error
[01:00:38]         ),
[01:00:38]         msg: "cannot refer to statics by value"
[01:00:38] ]
[01:00:38] 
[01:00:38] thread '[compile-fail] compile-fail/thread-local-in-ctfe.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[01:00:38] 
---
[01:00:38] 
[01:00:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:00:38] 
[01:00:38] 
[01:00:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02eff1a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
