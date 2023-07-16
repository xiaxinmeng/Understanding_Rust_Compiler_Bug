plain
travis_time:end:265246dc:start=1547398870285545705,finish=1547398940346930657,duration=70061384952
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
[01:09:20] 
[01:09:20] running 122 tests
[01:09:23] i..ii...iii..iiii.....i..............i..i......F.........i.....i......ii...i..i.ii..............i... 100/122
[01:09:24] failures:
[01:09:24] 
[01:09:24] ---- [codegen] codegen/issue-45222.rs stdout ----
[01:09:24] 
[01:09:24] 
[01:09:24] error: verification with 'FileCheck' failed
[01:09:24] status: exit code: 1
[01:09:24] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
[01:09:24] ------------------------------------------
[01:09:24] 
[01:09:24] ------------------------------------------
[01:09:24] stderr:
[01:09:24] stderr:
[01:09:24] ------------------------------------------
[01:09:24] /checkout/src/test/codegen/issue-45222.rs:23:12: error: expected string not found in input
[01:09:24]  // CHECK: ret i64 500005000000000
[01:09:24]            ^
[01:09:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:10:23: note: scanning from here
[01:09:24] define i64 @check_foo2() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:09:24]                       ^
[01:09:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:64:23: note: possible intended match here
[01:09:24]  %exitcond.i.1 = icmp eq i64 %7, 100000
[01:09:24] /checkout/src/test/codegen/issue-45222.rs:41:12: error: expected string not found in input
[01:09:24] /checkout/src/test/codegen/issue-45222.rs:41:12: error: expected string not found in input
[01:09:24]  // CHECK: ret i64 5000050000
[01:09:24]            ^
[01:09:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:76:31: note: scanning from here
[01:09:24] define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:09:24]                               ^
[01:09:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:105:2: note: possible intended match here
[01:09:24]  ret i64 %count.0.i
[01:09:24] 
[01:09:24] ------------------------------------------
[01:09:24] 
[01:09:24] thread '[codegen] codegen/issue-45222.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:09:24] 
[01:09:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:09:24] 
[01:09:24] 
[01:09:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:24] 
[01:09:24] 
[01:09:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:24] Build completed unsuccessfully in 0:11:13
[01:09:24] Build completed unsuccessfully in 0:11:13
[01:09:24] Makefile:48: recipe for target 'check' failed
[01:09:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11ced63a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 18:11:53 UTC 2019
---
travis_time:end:06760162:start=1547403114594602927,finish=1547403114599052845,duration=4449918
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0234ee99
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|')
