plain
travis_time:end:065440f8:start=1547393674353503519,finish=1547393753035679132,duration=78682175613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:57] 
[01:10:57] running 122 tests
[01:11:01] i..ii...iii..iiii.....i..............i..i......F.........i.....i......ii...i..i.ii..............i... 100/122
[01:11:01] failures:
[01:11:01] 
[01:11:01] ---- [codegen] codegen/issue-45222.rs stdout ----
[01:11:01] 
[01:11:01] 
[01:11:01] error: verification with 'FileCheck' failed
[01:11:01] status: exit code: 1
[01:11:01] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
[01:11:01] ------------------------------------------
[01:11:01] 
[01:11:01] ------------------------------------------
[01:11:01] stderr:
[01:11:01] stderr:
[01:11:01] ------------------------------------------
[01:11:01] /checkout/src/test/codegen/issue-45222.rs:23:12: error: expected string not found in input
[01:11:01]  // CHECK: ret i64 500005000000000
[01:11:01]            ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:10:23: note: scanning from here
[01:11:01] define i64 @check_foo2() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:11:01]                       ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:64:23: note: possible intended match here
[01:11:01]  %exitcond.i.1 = icmp eq i64 %7, 100000
[01:11:01] /checkout/src/test/codegen/issue-45222.rs:41:12: error: expected string not found in input
[01:11:01] /checkout/src/test/codegen/issue-45222.rs:41:12: error: expected string not found in input
[01:11:01]  // CHECK: ret i64 5000050000
[01:11:01]            ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:76:31: note: scanning from here
[01:11:01] define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:11:01]                               ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:105:2: note: possible intended match here
[01:11:01]  ret i64 %count.0.i
[01:11:01] /checkout/src/test/codegen/issue-45222.rs:61:12: error: expected string not found in input
[01:11:01] /checkout/src/test/codegen/issue-45222.rs:61:12: error: expected string not found in input
[01:11:01]  // CHECK: ret i64 500050000000
[01:11:01]            ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:109:24: note: scanning from here
[01:11:01] define i64 @check_foo3r() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:11:01]                        ^
[01:11:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:163:27: note: possible intended match here
[01:11:01]  %exitcond.i.i.i.1 = icmp eq i64 %14, 10000
[01:11:01] 
[01:11:01] ------------------------------------------
[01:11:01] 
[01:11:01] thread '[codegen] codegen/issue-45222.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:11:01] 
[01:11:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:11:01] 
[01:11:01] 
[01:11:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:01] 
[01:11:01] 
[01:11:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:01] Build completed unsuccessfully in 0:11:57
[01:11:01] Build completed unsuccessfully in 0:11:57
[01:11:01] Makefile:48: recipe for target 'check' failed
[01:11:01] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f86a6d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 16:47:04 UTC 2019
