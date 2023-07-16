plain
travis_time:end:15191826:start=1555808958141650945,finish=1555808960299598101,duration=2157947156
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:20] 
[01:14:20] running 42 tests
[01:14:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:14:39] .............FF...........................
[01:14:39] 
[01:14:39] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:14:39] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:14:39] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:14:39] Expected Line: "    _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 13 }];"
[01:14:39] Test Name: rustc.foo.Inline.after.mir
[01:14:39] ... (elided)
[01:14:39] ... (elided)
[01:14:39] bb0: {
[01:14:39] ... (elided)
[01:14:39] ... (elided)
[01:14:39]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 13 }];
[01:14:39] ... (elided)
[01:14:39]     _4 = &_3;
[01:14:39] ... (elided)
[01:14:39]     _6 = _2;
[01:14:39] ... (elided)
[01:14:39]     _7 = _2;
[01:14:39]     _5 = (move _6, move _7);
[01:14:39]     _8 = move (_5.0: i32);
[01:14:39]     _9 = move (_5.1: i32);
[01:14:39]     _0 = _8;
[01:14:39] ... (elided)
[01:14:39]     return;
[01:14:39] }
[01:14:39] ... (elided)
[01:14:39] Actual:
[01:14:39] fn  foo(_1: T, _2: i32) -> i32 {
[01:14:39]     let mut _0: i32;
[01:14:39]     scope 1 {
[01:14:39]         scope 3 {
[01:14:39]     }
[01:14:39]     scope 2 {
[01:14:39]     scope 2 {
[01:14:39]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:14:39]     }
[01:14:39]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:14:39]     let mut _5: (i32, i32);
[01:14:39]     let mut _6: i32;
[01:14:39]     let mut _7: i32;
[01:14:39]     let mut _8: i32;
[01:14:39]     let mut _9: i32;
[01:14:39]     bb0: {
[01:14:39]         StorageLive(_3);
[01:14:39]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:14:39]         StorageLive(_4);
[01:14:39]         _4 = &_3;
[01:14:39]         StorageLive(_5);
[01:14:39]         StorageLive(_6);
[01:14:39]         _6 = _2;
[01:14:39]         StorageLive(_7);
[01:14:39]         _7 = _2;
[01:14:39]         _5 = (move _6, move _7);
[01:14:39]         _8 = move (_5.0: i32);
[01:14:39]         _9 = move (_5.1: i32);
[01:14:39]         _0 = _8;
[01:14:39]         StorageDead(_5);
[01:14:39]         StorageDead(_7);
[01:14:39]         StorageDead(_6);
[01:14:39]         StorageDead(_4);
[01:14:39]         StorageDead(_3);
[01:14:39]         return;
[01:14:39] }', src/tools/compiletest/src/runtest.rs:3060:13
[01:14:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:39] 
[01:14:39] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:14:39] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:14:39] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:14:39] Expected Line: "    _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 29 }];"
[01:14:39] Test Name: rustc.foo.Inline.after.mir
[01:14:39] ... (elided)
[01:14:39] ... (elided)
[01:14:39] bb0: {
[01:14:39] ... (elided)
[01:14:39] ... (elided)
[01:14:39]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 29 }];
[01:14:39] ... (elided)
[01:14:39]     _4 = &_3;
[01:14:39] ... (elided)
[01:14:39]     _6 = &(*_2);
[01:14:39] ... (elided)
[01:14:39]     _7 = &(*_2);
[01:14:39]     _5 = (move _6, move _7);
[01:14:39]     _8 = move (_5.0: &i32);
[01:14:39]     _9 = move (_5.1: &i32);
[01:14:39] ... (elided)
[01:14:39]     _0 = (*_8);
[01:14:39] ... (elided)
[01:14:39]     return;
[01:14:39] }
[01:14:39] ... (elided)
[01:14:39] Actual:
[01:14:39] fn  foo(_1: T, _2: &i32) -> i32 {
[01:14:39]     let mut _0: i32;
[01:14:39]     scope 1 {
[01:14:39]         scope 3 {
[01:14:39]     }
[01:14:39]     scope 2 {
[01:14:39]     scope 2 {
[01:14:39]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:14:39]     scope 4 {
[01:14:39]     }
[01:14:39]     scope 5 {
[01:14:39]     }
[01:14:39]     }
[01:14:39]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:14:39]     let mut _5: (&i32, &i32);
[01:14:39]     let mut _6: &i32;
[01:14:39]     let mut _7: &i32;
[01:14:39]     let mut _8: &i32;
[01:14:39]     let mut _9: &i32;
[01:14:39]     bb0: {
[01:14:39]         StorageLive(_3);
[01:14:39]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:14:39]         StorageLive(_4);
[01:14:39]         _4 = &_3;
[01:14:39]         StorageLive(_5);
[01:14:39]         StorageLive(_6);
[01:14:39]         _6 = &(*_2);
[01:14:39]         StorageLive(_7);
[01:14:39]         _7 = &(*_2);
[01:14:39]         _5 = (move _6, move _7);
[01:14:39]         _8 = move (_5.0: &i32);
[01:14:39]         _9 = move (_5.1: &i32);
[01:14:39]         _0 = (*_8);
[01:14:39]         StorageDead(_5);
[01:14:39]         StorageDead(_7);
[01:14:39]         StorageDead(_6);
[01:14:39]         StorageDead(_4);
[01:14:39]         StorageDead(_3);
[01:14:39]         return;
[01:14:39] }', src/tools/compiletest/src/runtest.rs:3060:13
[01:14:39] 
[01:14:39] 
[01:14:39] failures:
[01:14:39] failures:
[01:14:39]     [mir-opt] mir-opt/inline-closure-borrows-arg.rs
[01:14:39]     [mir-opt] mir-opt/inline-closure.rs
[01:14:39] 
[01:14:39] test result: FAILED. 40 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:39] 
[01:14:39] 
[01:14:39] 
[01:14:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:39] 
[01:14:39] 
[01:14:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:39] Build completed unsuccessfully in 0:11:40
[01:14:39] Build completed unsuccessfully in 0:11:40
[01:14:39] make: *** [check] Error 1
[01:14:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d6c7404
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 02:24:09 UTC 2019
