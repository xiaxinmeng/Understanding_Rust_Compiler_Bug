plain
[00:01:13] Successfully tagged rust-ci:latest
[00:01:13] Built container sha256:67d099d31b98882e333296cc034fe9f89798543fe598df9b5c5dcc8411a7fc6f
[00:01:13] Uploading finished image to s3://rust-lang-ci-sccache2/docker/321ed64355351899f2a84c2c6f281d7caae58e20361c9d475023745a579a75dddf37316adae505b9069a2e3ae7125d3d4ef7bc0a8d5570a9385f927cceac8560
[00:01:13] 
[00:01:13] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:01:19] xargs: docker: terminated by signal 13

[00:01:19] travis_time:end:00f39840:start=1530277591953195547,finish=1530277660521625332,duration=68568429785
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:19] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:14] 
[01:03:14] running 95 tests
[01:03:18] i...i...i.i...i...i.................F.....i.........i...i...ii.............i...ii......iii.....
[01:03:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:03:18] 
[01:03:18] ---- [codegen] codegen/issue-45466.rs stdout ----
[01:03:18] 
[01:03:18] 
[01:03:18] error: verification with 'FileCheck' failed
[01:03:18] status: exit code: 1
[01:03:18] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll" "/checkout/src/test/codegen/issue-45466.rs"
[01:03:18] ------------------------------------------
[01:03:18] 
[01:03:18] ------------------------------------------
[01:03:18] stderr:
[01:03:18] stderr:
[01:03:18] ------------------------------------------
[01:03:18] /checkout/src/test/codegen/issue-45466.rs:17:11: error: expected string not found in input
[01:03:18] // CHECK: call void @llvm.memset
[01:03:18]           ^
[01:03:18] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll:10:21: note: scanning from here
[01:03:18] define void @memzero([0 x i8]* nocapture nonnull %data.0, i64 %data.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:03:18]                     ^
[01:03:18] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45466/issue-45466.ll:97:63: note: possible intended match here
[01:03:18]  br i1 %niter.ncmp.7, label %middle.block.unr-lcssa, label %vector.body, !llvm.loop !0
[01:03:18] 
[01:03:18] ------------------------------------------
[01:03:18] 
[01:03:18] thread '[codegen] codegen/issue-45466.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
---
[01:03:18] test result: FAILED. 77 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out
[01:03:18] 
[01:03:18] 
[01:03:18] 
[01:03:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:18] 
[01:03:18] 
[01:03:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:18] Build completed unsuccessfully in 0:10:39
[01:03:18] Build completed unsuccessfully in 0:10:39
[01:03:18] Makefile:58: recipe for target 'check' failed
[01:03:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27145777
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.5
travis_time:start:0879aeba
$ dmesg | grep -i kill
[   10.381480] init: failsafe main process (1092) killed by TERM signal
[   41.635626] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.5

Done. Your build exited with 1.
