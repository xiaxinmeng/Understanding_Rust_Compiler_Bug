plain
travis_time:end:15ffd364:start=1555838795580904592,finish=1555838797724625833,duration=2143721241
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:17:56] 
[01:17:56] running 42 tests
[01:18:00] ERROR 2019-04-21T10:44:48Z: compiletest::runtest: None
[01:18:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:18:15] ..............F...........................
[01:18:15] 
[01:18:15] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:18:15] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:18:15] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:18:15] Current block: None
[01:18:15] Actual Line: "        _8 = move (_5.0: &i32);"
[01:18:15] Expected Line: "    _9 = move (_5.0: &i32);"
[01:18:15] Test Name: rustc.foo.Inline.after.mir
[01:18:15] ... (elided)
[01:18:15] ... (elided)
[01:18:15] bb0: {
[01:18:15] ... (elided)
[01:18:15] ... (elided)
[01:18:15]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:18:15] ... (elided)
[01:18:15]     _4 = &_3;
[01:18:15] ... (elided)
[01:18:15]     _6 = &(*_2);
[01:18:15] ... (elided)
[01:18:15]     _7 = &(*_2);
[01:18:15]     _5 = (move _6, move _7);
[01:18:15]     _9 = move (_5.0: &i32);
[01:18:15]     _10 = move (_5.1: &i32);
[01:18:15] ... (elided)
[01:18:15]     _0 = (*_9);
[01:18:15] ... (elided)
[01:18:15]     return;
[01:18:15] }
[01:18:15] ... (elided)
[01:18:15] Actual:
[01:18:15] fn  foo(_1: T, _2: &i32) -> i32 {
[01:18:15]     let mut _0: i32;
[01:18:15]     scope 1 {
[01:18:15]         scope 3 {
[01:18:15]     }
[01:18:15]     scope 2 {
[01:18:15]     scope 2 {
[01:18:15]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:18:15]     scope 4 {
[01:18:15]     }
[01:18:15]     scope 5 {
[01:18:15]     }
[01:18:15]     }
[01:18:15]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:18:15]     let mut _5: (&i32, &i32);
[01:18:15]     let mut _6: &i32;
[01:18:15]     let mut _7: &i32;
[01:18:15]     let mut _8: &i32;
[01:18:15]     let mut _9: &i32;
[01:18:15]     bb0: {
[01:18:15]         StorageLive(_3);
[01:18:15]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:18:15]         StorageLive(_4);
[01:18:15]         _4 = &_3;
[01:18:15]         StorageLive(_5);
[01:18:15]         StorageLive(_6);
[01:18:15]         _6 = &(*_2);
[01:18:15]         StorageLive(_7);
[01:18:15]         _7 = &(*_2);
[01:18:15]         _5 = (move _6, move _7);
[01:18:15]         _8 = move (_5.0: &i32);
[01:18:15]         _9 = move (_5.1: &i32);
[01:18:15]         _0 = (*_8);
[01:18:15]         StorageDead(_5);
[01:18:15]         StorageDead(_7);
[01:18:15]         StorageDead(_6);
[01:18:15]         StorageDead(_4);
[01:18:15]         StorageDead(_3);
[01:18:15]         return;
[01:18:15] }', src/tools/compiletest/src/runtest.rs:3060:13
[01:18:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:15] 
[01:18:15] 
[01:18:15] 
[01:18:15] failures:
[01:18:15]     [mir-opt] mir-opt/inline-closure-borrows-arg.rs
[01:18:15] 
[01:18:15] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:15] 
[01:18:15] 
[01:18:15] 
[01:18:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:15] 
[01:18:15] 
[01:18:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:15] Build completed unsuccessfully in 0:12:13
[01:18:15] Build completed unsuccessfully in 0:12:13
[01:18:15] Makefile:48: recipe for target 'check' failed
[01:18:15] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bcfbbcc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 10:45:04 UTC 2019
---
travis_time:end:0ee20c21:start=1555843506472803968,finish=1555843506478009901,duration=5205933
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:231a0c00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
