plain
travis_time:end:0001dd24:start=1550141804655293538,finish=1550141806916553600,duration=2261260062
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
[01:12:02] 
[01:12:02] running 130 tests
[01:12:05] i..iii...iii..iiiiF....i.................i..i................i.....i.........ii...i..i.ii........... 100/130
[01:12:06] failures:
[01:12:06] 
[01:12:06] ---- [codegen] codegen/box-maybe-uninit.rs stdout ----
[01:12:06] 
[01:12:06] 
[01:12:06] error: verification with 'FileCheck' failed
[01:12:06] status: exit code: 1
[01:12:06] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll" "/checkout/src/test/codegen/box-maybe-uninit.rs"
[01:12:06] ------------------------------------------
[01:12:06] 
[01:12:06] ------------------------------------------
[01:12:06] stderr:
[01:12:06] stderr:
[01:12:06] ------------------------------------------
[01:12:06] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/box-maybe-uninit/box-maybe-uninit.ll:28:2: error: CHECK-NOT: string occurred!
[01:12:06]  store i64 8, i64* %.fca.0.gep.i.i, align 8
[01:12:06]  ^
[01:12:06] /checkout/src/test/codegen/box-maybe-uninit.rs:11:16: note: CHECK-NOT: pattern specified here
[01:12:06]  // CHECK-NOT: store
[01:12:06] 
[01:12:06] ------------------------------------------
[01:12:06] 
[01:12:06] thread '[codegen] codegen/box-maybe-uninit.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:12:06] 
[01:12:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:06] 
[01:12:06] 
[01:12:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:06] 
[01:12:06] 
[01:12:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:06] Build completed unsuccessfully in 0:11:28
[01:12:06] Build completed unsuccessfully in 0:11:28
[01:12:06] make: *** [check] Error 1
[01:12:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:012ebd40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 12:09:05 UTC 2019
