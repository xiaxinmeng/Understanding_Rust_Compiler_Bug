plain
travis_time:end:1c50ad22:start=1555993175460226313,finish=1555993272897439054,duration=97437212741
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:48] 
[01:12:48] running 42 tests
[01:12:50] ERROR 2019-04-23T05:34:11Z: compiletest::runtest: None
[01:13:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:13:06] ........F.................................
[01:13:06] 
[01:13:06] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:13:06] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:13:06] thread '[mir-opt] mir-opt/generator-drop-cleanup.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:13:06] Current block: None
[01:13:06] Actual Line: "        _5 = discriminant((*_1));"
[01:13:06] Expected Line: "    switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];"
[01:13:06] Test Name: rustc.main-{{closure}}.generator_drop.0.mir
[01:13:06] ... (elided)
[01:13:06] bb0: {
[01:13:06] bb0: {
[01:13:06]     switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:13:06] }
[01:13:06] bb1: {
[01:13:06]     goto -> bb5;
[01:13:06] }
[01:13:06] bb2: {
[01:13:06]     return;
[01:13:06] }
[01:13:06] bb3: {
[01:13:06]     return;
[01:13:06] }
[01:13:06] bb4: {
[01:13:06]     goto -> bb6;
[01:13:06] }
[01:13:06] bb5: {
[01:13:06]     goto -> bb2;
[01:13:06] }
[01:13:06] bb6: {
[01:13:06]     goto -> bb3;
[01:13:06] }
[01:13:06] bb7: {
[01:13:06]     StorageLive(_3);
[01:13:06]     goto -> bb1;
[01:13:06] }
[01:13:06] bb8: {
[01:13:06]     return;
[01:13:06] }
[01:13:06] Actual:
[01:13:06] fn  main::{{closure}}#0(_1: *mut [generator@/checkout/src/test/mir-opt/generator-drop-cleanup.rs:7:15: 9:6 {()}]) -> () {
[01:13:06]     let mut _0: ();
[01:13:06]     let mut _2: ();
[01:13:06]     let mut _3: ();
[01:13:06]     let mut _4: ();
[01:13:06]     let mut _5: isize;
[01:13:06]     bb0: {
[01:13:06]         _5 = discriminant((*_1));
[01:13:06]         switchInt(move _5) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:13:06]     bb1: {
[01:13:06]         goto -> bb5;
[01:13:06]     }
[01:13:06]     bb2: {
---
[01:13:06]     }
[01:13:06]     bb6: {
[01:13:06]         goto -> bb3;
[01:13:06]     }
[01:13:06]     bb7: {
[01:13:06]         StorageLive(_3);
[01:13:06]         goto -> bb1;
[01:13:06]     bb8: {
[01:13:06]         return;
[01:13:06]     }
[01:13:06] }', src/tools/compiletest/src/runtest.rs:3061:13
---
[01:13:06] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:13:06] 
[01:13:06] 
[01:13:06] 
[01:13:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:06] 
[01:13:06] 
[01:13:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:06] Build completed unsuccessfully in 0:11:22
[01:13:06] Build completed unsuccessfully in 0:11:22
[01:13:06] make: *** [check] Error 1
[01:13:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e82f6aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 05:34:29 UTC 2019
