plain
[00:45:31] ..............................................................i.....................................
[00:45:34] ....................................................................................................
[00:45:38] ....................................................................................................
[00:45:43] ...........................................................................................i........
[00:45:45] .........iiiiiiiii...................................................
[00:45:45] 
[00:45:45] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:25] ..............................................................i.....................................
[00:46:29] ....................................................................................................
[00:46:34] ....................................................................................................
[00:46:40] ...........................................................................................i........
[00:46:42] .........iiiiiiiii...................................................
[00:46:42] 
[00:46:42]  finished in 57.551
[00:46:42] travis_fold:end:test_ui_nll

---
[00:55:20] ....................................................................................................
[00:55:25] ....................................................................................................
[00:55:29] ....................................................................................................
[00:55:34] ....................................................................................................
[00:55:40] ...................................................................F..................i.............
[00:55:49] ..............................................................................i.....................
[00:55:54] .......................i............................................................................
[00:55:54] .......................i............................................................................
[00:55:58] ...........................................................F..................................i.....
[00:56:02] .............F.........F............................................................................
[00:56:08] ....................................................................................................
[00:56:13] .F...........................................................................................F......
[00:56:19] ....................................................................................................
[00:56:25] ..............................................................F.....................................
[00:56:37] ..i.................................................................................................
[00:56:43] .i..ii..............................................................................................
[00:56:50] ....................................................................................................
[00:56:55] ....................................................................................................
[00:56:55] ....................................................................................................
[00:56:58] .............................................................................i......................
[00:57:03] .......................i.....................................F......................................
[00:57:09] ................F...................................................................................
[00:57:16] ....................................................................................................
[00:57:17] ...................
[00:57:17] failures:
[00:57:17] 
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/const-fn-not-safe-for-const.rs stdout ----
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs:30: expected error not found: cannot refer to statics by value
[00:57:17] 
[00:57:17] error: 0 unexpected errors found, 1 expected errors not found
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-fn-not-safe-for-const.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-not-safe-for-const/auxiliary" "-A" "unused"
[00:57:17] not found errors (from test file): [
[00:57:17]     Error {
[00:57:17]         line_num: 30,
[00:57:17]         kind: Some(
[00:57:17]         ),
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to statics by value"
[00:57:17] ]
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/const-fn-not-safe-for-const.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/issue-14227.rs stdout ----
[00:57:17] 
[00:57:17] error: compiler encountered internal error
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-14227.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-14227/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
[00:57:17] stderr:
[00:57:17] ------------------------------------------
[00:57:17] {"message":"librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_14227[317d]::[0]::symbol[0])","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-14227.rs","byte_start":522,"byte_end":544,"line_start":14,"line_end":14,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub static symbol: ();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_14227[317d]::[0]::symbol[0])\n  --> /checkout/src/test/compile-fail/issue-14227.rs:14:5\n   |\nLL |     pub static symbol: ();\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:17] 
[00:57:17] note: the compiler unexpectedly panicked. this is a bug.
[00:57:17] 
[00:57:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:17] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:57:17] 
[00:57:17] 
[00:57:17] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/issue-14227.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] thread '[compile-fail] compile-fail/issue-14227.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/issue-17450.rs stdout ----
[00:57:17] 
[00:57:17] error: compile-fail test compiled successfully!
[00:57:17] status: exit code: 0
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17450.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17450/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17450/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
---
[00:57:17] thread '[compile-fail] compile-fail/issue-17450.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/issue-17718-references.rs stdout ----
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/issue-17718-references.rs:24: expected message not found: cannot refer to statics
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/issue-17718-references.rs:27: expected error not found: cannot refer to other statics by value
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/issue-17718-references.rs:30: expected error not found: cannot refer to statics by value
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/issue-17718-references.rs:33: expected error not found: cannot refer to other statics by value
[00:57:17] 
[00:57:17] error: 0 unexpected errors found, 4 expected errors not found
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-17718-references.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-17718-references/auxiliary" "-A" "unused"
[00:57:17] not found errors (from test file): [
[00:57:17]     Error {
[00:57:17]         line_num: 24,
[00:57:17]         kind: None,
[00:57:17]         msg: "cannot refer to statics"
[00:57:17]     Error {
[00:57:17]         line_num: 27,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to other statics by value"
[00:57:17]     Error {
[00:57:17]         line_num: 30,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to statics by value"
[00:57:17]     Error {
[00:57:17]         line_num: 33,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to other statics by value"
[00:57:17] ]
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/issue-17718-references.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:57:17] 
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/issue-28324.rs stdout ----
[00:57:17] 
[00:57:17] error: compiler encountered internal error
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-28324.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-28324/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
[00:57:17] stderr:
[00:57:17] ------------------------------------------
[00:57:17] {"message":"librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_28324[317d]::[0]::error_message_count[0])","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-28324.rs","byte_start":512,"byte_end":544,"line_start":14,"line_end":14,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    static error_message_count: u32;","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc_typeck/check/mod.rs:829: can't type-check body of DefId(0/1:9 ~ issue_28324[317d]::[0]::error_message_count[0])\n  --> /checkout/src/test/compile-fail/issue-28324.rs:14:5\n   |\nLL |     static error_message_count: u32;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:17] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:57:17] 
[00:57:17] note: the compiler unexpectedly panicked. this is a bug.
[00:57:17] 
[00:57:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:17] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:57:17] 
[00:57:17] 
[00:57:17] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] thread '[compile-fail] compile-fail/issue-28324.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/issue-34194.rs stdout ----
[00:57:17] 
[00:57:17] error: compile-fail test compiled successfully!
[00:57:17] status: exit code: 0
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-34194.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34194/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-34194/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
---
[00:57:17] ---- [compile-fail] compile-fail/issue-6991.rs stdout ----
[00:57:17] 
[00:57:17] error: compile-fail test compiled successfully!
[00:57:17] status: exit code: 0
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-6991.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-6991/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-6991/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
[00:57:17] stderr:
[00:57:17] ------------------------------------------
[00:57:17] {"message":"static variable `x` should have an upper case name such as `X`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-6991.rs","byte_start":467,"byte_end":497,"line_start":11,"line_end":11,"column_start":1,"column_end":31,"is_primary":true,"text":[{"text":"static x: &'static usize = &1;","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_upper_case_globals)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: static variable `x` should have an upper case name such as `X`\n  --> /checkout/src/test/compile-fail/issue-6991.rs:11:1\n   |\nLL | static x: &'static usize = &1;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(non_upper_case_globals)] on by default\n\n"}
[00:57:17] {"message":"static variable `y` should have an upper case name such as `Y`","code":{"code":"non_upper_case_globals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/compile-fail/issue-6991.rs","byte_start":498,"byte_end":519,"line_start":12,"line_end":12,"column_start":1,"column_end":22,"is_primary":true,"text":[{"text":"static y: usize = *x;","highlight_start":1,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: static variable `y` should have an upper case name such as `Y`\n  --> /checkout/src/test/compile-fail/issue-6991.rs:12:1\n   |\nLL | static y: usize = *x;\n   | ^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/issue-6991.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] 
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----
[00:57:17] 
[00:57:17] error: compiler panicked
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/static-array-across-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/auxiliary" "-A" "unused"
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] stderr:
[00:57:17] stderr:
[00:57:17] ------------------------------------------
[00:57:17] thread 'main' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:57:17] 
[00:57:17] error: internal compiler error: unexpected panic
[00:57:17] 
[00:57:17] 
[00:57:17] note: the compiler unexpectedly panicked. this is a bug.
[00:57:17] 
[00:57:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:57:17] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:57:17] 
[00:57:17] 
[00:57:17] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:57:17] 
[00:57:17] ------------------------------------------
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/static-array-across-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] thread '[compile-fail] compile-fail/static-array-across-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[00:57:17] 
[00:57:17] ---- [compile-fail] compile-fail/thread-local-in-ctfe.rs stdout ----
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:16: expected error not found: cannot refer to other statics by value
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:23: expected error not found: cannot refer to statics by value
[00:57:17] 
[00:57:17] error: /checkout/src/test/compile-fail/thread-local-in-ctfe.rs:31: expected error not found: cannot refer to statics by value
[00:57:17] 
[00:57:17] error: 0 unexpected errors found, 3 expected errors not found
[00:57:17] status: exit code: 101
[00:57:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/thread-local-in-ctfe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/thread-local-in-ctfe/auxiliary" "-A" "unused"
[00:57:17] not found errors (from test file): [
[00:57:17]     Error {
[00:57:17]         line_num: 16,
[00:57:17]         kind: Some(
[00:57:17]         ),
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to other statics by value"
[00:57:17]     Error {
[00:57:17]         line_num: 23,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to statics by value"
[00:57:17]     Error {
[00:57:17]         line_num: 31,
[00:57:17]         kind: Some(
[00:57:17]             Error
[00:57:17]             Error
[00:57:17]         ),
[00:57:17]         msg: "cannot refer to statics by value"
[00:57:17] ]
[00:57:17] 
[00:57:17] thread '[compile-fail] compile-fail/thread-local-in-ctfe.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1283:13
[00:57:17] 
---
[00:57:17] 
[00:57:17] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:17] 
[00:57:17] 
[00:57:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:17] 
[00:57:17] 
[00:57:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:17] Build completed unsuccessfully in 0:13:57
[00:57:17] Build completed unsuccessfully in 0:13:57
[00:57:17] make: *** [check] Error 1
[00:57:17] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b9edc3b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
