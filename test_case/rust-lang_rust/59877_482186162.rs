plain
travis_time:end:0089724b:start=1554995312173335246,finish=1554995314455618225,duration=2282282979
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
[01:16:33] 
[01:16:33] running 42 tests
[01:17:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:17:07] .............FF.................F.........
[01:17:07] 
[01:17:07] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:17:07] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:17:07] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:07] Expected Line: "    _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 11 }];"
[01:17:07] Test Name: rustc.foo.Inline.after.mir
[01:17:07] ... (elided)
[01:17:07] ... (elided)
[01:17:07] bb0: {
[01:17:07] ... (elided)
[01:17:07] ... (elided)
[01:17:07]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 11 }];
[01:17:07] ... (elided)
[01:17:07]     _4 = &_3;
[01:17:07] ... (elided)
[01:17:07]     _6 = _2;
[01:17:07] ... (elided)
[01:17:07]     _7 = _2;
[01:17:07]     _5 = (move _6, move _7);
[01:17:07]     _8 = move (_5.0: i32);
[01:17:07]     _9 = move (_5.1: i32);
[01:17:07]     _0 = _8;
[01:17:07] ... (elided)
[01:17:07]     return;
[01:17:07] }
[01:17:07] ... (elided)
[01:17:07] Actual:
[01:17:07] fn  foo(_1: T, _2: i32) -> i32 {
[01:17:07]     let mut _0: i32;
[01:17:07]     scope 1 {
[01:17:07]         scope 3 {
[01:17:07]     }
[01:17:07]     scope 2 {
[01:17:07]     scope 2 {
[01:17:07]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 13 }];
[01:17:07]     }
[01:17:07]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 13 }];
[01:17:07]     let mut _5: (i32, i32);
[01:17:07]     let mut _6: i32;
[01:17:07]     let mut _7: i32;
[01:17:07]     let mut _8: i32;
[01:17:07]     let mut _9: i32;
[01:17:07]     bb0: {
[01:17:07]         StorageLive(_3);
[01:17:07]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 13 }];
[01:17:07]         StorageLive(_4);
[01:17:07]         _4 = &_3;
[01:17:07]         StorageLive(_5);
[01:17:07]         StorageLive(_6);
[01:17:07]         _6 = _2;
[01:17:07]         StorageLive(_7);
[01:17:07]         _7 = _2;
[01:17:07]         _5 = (move _6, move _7);
[01:17:07]         _8 = move (_5.0: i32);
[01:17:07]         _9 = move (_5.1: i32);
[01:17:07]         _0 = _8;
[01:17:07]         StorageDead(_5);
[01:17:07]         StorageDead(_7);
[01:17:07]         StorageDead(_6);
[01:17:07]         StorageDead(_4);
[01:17:07]         StorageDead(_3);
[01:17:07]         return;
[01:17:07] }', src/tools/compiletest/src/runtest.rs:3056:13
[01:17:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:07] 
[01:17:07] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:17:07] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:17:07] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:07] Expected Line: "    _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 27 }];"
[01:17:07] Test Name: rustc.foo.Inline.after.mir
[01:17:07] ... (elided)
[01:17:07] ... (elided)
[01:17:07] bb0: {
[01:17:07] ... (elided)
[01:17:07] ... (elided)
[01:17:07]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 27 }];
[01:17:07] ... (elided)
[01:17:07]     _4 = &_3;
[01:17:07] ... (elided)
[01:17:07]     _6 = &(*_2);
[01:17:07] ... (elided)
[01:17:07]     _7 = &(*_2);
[01:17:07]     _5 = (move _6, move _7);
[01:17:07]     _8 = move (_5.0: &i32);
[01:17:07]     _9 = move (_5.1: &i32);
[01:17:07] ... (elided)
[01:17:07]     _0 = (*_8);
[01:17:07] ... (elided)
[01:17:07]     return;
[01:17:07] }
[01:17:07] ... (elided)
[01:17:07] Actual:
[01:17:07] fn  foo(_1: T, _2: &i32) -> i32 {
[01:17:07]     let mut _0: i32;
[01:17:07]     scope 1 {
[01:17:07]         scope 3 {
[01:17:07]     }
[01:17:07]     scope 2 {
[01:17:07]     scope 2 {
[01:17:07]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 29 }];
[01:17:07]     scope 4 {
[01:17:07]     }
[01:17:07]     scope 5 {
[01:17:07]     }
[01:17:07]     }
[01:17:07]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 29 }];
[01:17:07]     let mut _5: (&i32, &i32);
[01:17:07]     let mut _6: &i32;
[01:17:07]     let mut _7: &i32;
[01:17:07]     let mut _8: &i32;
[01:17:07]     let mut _9: &i32;
[01:17:07]     bb0: {
[01:17:07]         StorageLive(_3);
[01:17:07]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 29 }];
[01:17:07]         StorageLive(_4);
[01:17:07]         _4 = &_3;
[01:17:07]         StorageLive(_5);
[01:17:07]         StorageLive(_6);
[01:17:07]         _6 = &(*_2);
[01:17:07]         StorageLive(_7);
[01:17:07]         _7 = &(*_2);
[01:17:07]         _5 = (move _6, move _7);
[01:17:07]         _8 = move (_5.0: &i32);
[01:17:07]         _9 = move (_5.1: &i32);
[01:17:07]         _0 = (*_8);
[01:17:07]         StorageDead(_5);
[01:17:07]         StorageDead(_7);
[01:17:07]         StorageDead(_6);
[01:17:07]         StorageDead(_4);
[01:17:07]         StorageDead(_3);
[01:17:07]         return;
[01:17:07] }', src/tools/compiletest/src/runtest.rs:3056:13
[01:17:07] 
[01:17:07] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:17:07] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:17:07] thread '[mir-opt] mir-opt/retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:07] Expected Line: "fn main::{{closure}}#0(_1: &[closure@HirId { owner: DefIndex(0:7), local_id: 70 }], _2: &i32) -> &i32 {"
[01:17:07] Test Name: rustc.main-{{closure}}.EraseRegions.after.mir
[01:17:07] ... (elided)
[01:17:07] ... (elided)
[01:17:07] fn main::{{closure}}#0(_1: &[closure@HirId { owner: DefIndex(0:7), local_id: 70 }], _2: &i32) -> &i32 {
[01:17:07] ... (elided)
[01:17:07]     bb0: {
[01:17:07]         Retag([fn entry] _1);
[01:17:07]         Retag([fn entry] _2);
[01:17:07]         StorageLive(_3);
[01:17:07]         _3 = _2;
[01:17:07]         Retag(_3);
[01:17:07]         _0 = _2;
[01:17:07]         Retag(_0);
[01:17:07]         StorageDead(_3);
[01:17:07]         return;
[01:17:07] }
[01:17:07] Actual:
[01:17:07] Actual:
[01:17:07] fn  main::{{closure}}#0(_1: &[closure@HirId { owner: DefIndex(0:7), local_id: 72 }], _2: &i32) -> &i32 {
[01:17:07]     let mut _0: &i32;
[01:17:07]     scope 1 {
[01:17:07]     scope 2 {
[01:17:07]         let _3: &i32;
[01:17:07]     }
[01:17:07]     bb0: {
[01:17:07]     bb0: {
[01:17:07]         Retag([fn entry] _1);
[01:17:07]         Retag([fn entry] _2);
[01:17:07]         StorageLive(_3);
[01:17:07]         _3 = _2;
[01:17:07]         Retag(_3);
[01:17:07]         _0 = _2;
[01:17:07]         Retag(_0);
[01:17:07]         StorageDead(_3);
[01:17:07]         return;
[01:17:07] }', src/tools/compiletest/src/runtest.rs:3056:13
[01:17:07] 
[01:17:07] 
[01:17:07] failures:
---
[01:17:07] test result: FAILED. 39 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:17:07] 
[01:17:07] 
[01:17:07] 
[01:17:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:07] 
[01:17:07] 
[01:17:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:07] Build completed unsuccessfully in 0:13:01
[01:17:07] Build completed unsuccessfully in 0:13:01
[01:17:07] make: *** [check] Error 1
[01:17:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b264070
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 16:25:53 UTC 2019
---
travis_time:end:2c1acdcd:start=1554999955504808559,finish=1554999955510592023,duration=5783464
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13d48d5d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:085b96b8
travis_time:start:085b96b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:305ecf26
$ dmesg | grep -i kill
