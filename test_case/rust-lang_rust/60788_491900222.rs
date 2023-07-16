plain
travis_time:end:0598db86:start=1557761265633418046,finish=1557761267613646217,duration=1980228171
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:47] 
[01:19:47] running 143 tests
[01:19:50] i..iii.....iii...iiii....i...............F......i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:19:52] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:52] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:52] .............i.........ii.i.....iii...F....
[01:19:52] 
[01:19:52] ---- [codegen] codegen/i686-no-macosx-deployment-target.rs stdout ----
[01:19:52] 
[01:19:52] 
[01:19:52] error: verification with 'FileCheck' failed
[01:19:52] status: exit code: 1
[01:19:52] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-no-macosx-deployment-target/i686-no-macosx-deployment-target.ll" "/checkout/src/test/codegen/i686-no-macosx-deployment-target.rs"
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] stderr:
[01:19:52] stderr:
[01:19:52] ------------------------------------------
[01:19:52] /checkout/src/test/codegen/i686-no-macosx-deployment-target.rs:22:11: error: expected string not found in input
[01:19:52] // CHECK: target triple = "i686-apple-darwin"
[01:19:52]           ^
[01:19:52] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-no-macosx-deployment-target/i686-no-macosx-deployment-target.ll:1:1: note: scanning from here
[01:19:52] ; ModuleID = 'i686_no_macosx_deployment_target.3a1fbbbh-cgu.0'
[01:19:52] ^
[01:19:52] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/i686-no-macosx-deployment-target/i686-no-macosx-deployment-target.ll:4:1: note: possible intended match here
[01:19:52] target triple = "i686-apple-macosx10.7.0"
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] 
[01:19:52] 
[01:19:52] ---- [codegen] codegen/x86_64-no-macosx-deployment-target.rs stdout ----
[01:19:52] 
[01:19:52] error: verification with 'FileCheck' failed
[01:19:52] status: exit code: 1
[01:19:52] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-no-macosx-deployment-target/x86_64-no-macosx-deployment-target.ll" "/checkout/src/test/codegen/x86_64-no-macosx-deployment-target.rs"
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] stderr:
[01:19:52] stderr:
[01:19:52] ------------------------------------------
[01:19:52] /checkout/src/test/codegen/x86_64-no-macosx-deployment-target.rs:22:11: error: expected string not found in input
[01:19:52] // CHECK: target triple = "x86_64-apple-darwin"
[01:19:52]           ^
[01:19:52] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-no-macosx-deployment-target/x86_64-no-macosx-deployment-target.ll:1:1: note: scanning from here
[01:19:52] ; ModuleID = 'x86_64_no_macosx_deployment_target.3a1fbbbh-cgu.0'
[01:19:52] ^
[01:19:52] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/x86_64-no-macosx-deployment-target/x86_64-no-macosx-deployment-target.ll:4:1: note: possible intended match here
[01:19:52] target triple = "x86_64-apple-macosx10.7.0"
[01:19:52] 
[01:19:52] ------------------------------------------
[01:19:52] 
[01:19:52] 
---
[01:19:52] test result: FAILED. 111 passed; 2 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:52] 
[01:19:52] 
[01:19:52] 
[01:19:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:52] 
[01:19:52] 
[01:19:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:52] Build completed unsuccessfully in 0:11:51
[01:19:52] Build completed unsuccessfully in 0:11:51
[01:19:52] make: *** [check] Error 1
[01:19:52] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fadbc67
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 16:47:51 UTC 2019
