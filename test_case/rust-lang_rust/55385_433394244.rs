plain
travis_time:end:17548ce7:start=1540553630592160539,finish=1540553683561884755,duration=52969724216
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:52:35] .................................................................................................... 2200/4952
[00:52:39] .................................................................................................... 2300/4952
[00:52:43] .................................................................................................... 2400/4952
[00:52:47] .................................................................................................... 2500/4952
[00:52:50] .....................................................iiiiiiiii...................................... 2600/4952
[00:52:57] ...ii............................................................................................... 2800/4952
[00:53:00] .................................................................................................... 2900/4952
[00:53:04] ..............................................................................................i..... 3000/4952
[00:53:06] .................................................................................................... 3100/4952
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:13] 
[01:06:13] running 48 tests
[01:06:19] ERROR 2018-10-26T12:41:13Z: compiletest::runtest: None
[01:06:33] F....................F..........................
[01:06:33] 
[01:06:33] ---- [mir-opt] mir-opt/copy_propagation.rs stdout ----
[01:06:33] ---- [mir-opt] mir-opt/copy_propagation.rs stdout ----
[01:06:33] thread '[mir-opt] mir-opt/copy_propagation.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:06:33] Expected Line: "     _0 = _1;"
[01:06:33] Test Name: rustc.test.CopyPropagation.after.mir
[01:06:33] ... (elided)
[01:06:33]  bb0: {
[01:06:33] ... (elided)
[01:06:33]      _0 = _1;
[01:06:33]      _0 = _1;
[01:06:33] ... (elided)
[01:06:33]      return;
[01:06:33]  }
[01:06:33] Actual:
[01:06:33] fn test(_1: u32) -> u32{
[01:06:33]     let mut _0: u32;
[01:06:33]     scope 1 {
[01:06:33]     scope 2 {
[01:06:33]         let _2: u32;
[01:06:33]     }
[01:06:33]     }
[01:06:33]     bb0: {                              
[01:06:33]         StorageLive(_2);
[01:06:33]         _2 = _1;
[01:06:33]         _0 = _2;
[01:06:33]         StorageDead(_2);
[01:06:33]         return;
[01:06:33]     bb1: {
[01:06:33]         resume;
[01:06:33]     }
[01:06:33] }', tools/compiletest/src/runtest.rs:2939:13
[01:06:33] }', tools/compiletest/src/runtest.rs:2939:13
[01:06:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:33] 
[01:06:33] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:06:33] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:06:33] Current block: None
[01:06:33] Actual Line: "        _9 = move (_5.0: &i32);"
[01:06:33] Expected Line: "    _8 = move (_5.0: &i32);"
[01:06:33] Test Name: rustc.foo.Inline.after.mir
[01:06:33] ... (elided)
[01:06:33] ... (elided)
[01:06:33] bb0: {
[01:06:33] ... (elided)
[01:06:33] ... (elided)
[01:06:33]     _3 = [closure@NodeId(53)];
[01:06:33] ... (elided)
[01:06:33]     _4 = &_3;
[01:06:33] ... (elided)
[01:06:33]     _6 = &(*_2);
[01:06:33] ... (elided)
[01:06:33]     _7 = &(*_2);
[01:06:33]     _5 = (move _6, move _7);
[01:06:33]     _8 = move (_5.0: &i32);
[01:06:33]     _9 = move (_5.1: &i32);
[01:06:33] ... (elided)
[01:06:33]     _0 = (*_8);
[01:06:33] ... (elided)
[01:06:33]     return;
[01:06:33] }
[01:06:33] ... (elided)
[01:06:33] Actual:
[01:06:33] fn foo(_1: T, _2: &i32) -> i32{
[01:06:33]     let mut _0: i32;
[01:06:33]     scope 1 {
[01:06:33]         scope 3 {
[01:06:33]     }
[01:06:33]     scope 2 {
[01:06:33]     scope 2 {
[01:06:33]         let _3: [closure@NodeId(53)];
[01:06:33]     scope 4 {
[01:06:33]     }
[01:06:33]     scope 5 {
[01:06:33]         let _8: &i32;
[01:06:33]         let _8: &i32;
[01:06:33]     }
[01:06:33]     let mut _4: &[closure@NodeId(53)];
[01:06:33]     let mut _5: (&i32, &i32);
[01:06:33]     let mut _6: &i32;
[01:06:33]     let mut _7: &i32;
[01:06:33]     let mut _9: &i32;
[01:06:33]     let mut _10: &i32;
[01:06:33]     bb0: {                              
[01:06:33]         StorageLive(_3);
[01:06:33]         _3 = [closure@NodeId(53)];
[01:06:33]         StorageLive(_4);
[01:06:33]         _4 = &_3;
[01:06:33]         StorageLive(_5);
[01:06:33]         StorageLive(_6);
[01:06:33]         _6 = &(*_2);
[01:06:33]         StorageLive(_7);
[01:06:33]         _7 = &(*_2);
[01:06:33]         _5 = (move _6, move _7);
[01:06:33]         _9 = move (_5.0: &i32);
[01:06:33]         _10 = move (_5.1: &i32);
[01:06:33]         StorageLive(_8);
[01:06:33]         _8 = _9;
[01:06:33]         _0 = (*_8);
[01:06:33]         StorageDead(_8);
[01:06:33]         StorageDead(_5);
[01:06:33]         StorageDead(_7);
[01:06:33]         StorageDead(_6);
[01:06:33]         StorageDead(_4);
[01:06:33]         StorageDead(_3);
[01:06:33]         return;
[01:06:33] }', tools/compiletest/src/runtest.rs:2939:13
[01:06:33] 
[01:06:33] 
[01:06:33] failures:
---
[01:06:33] 
[01:06:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:06:33] 
[01:06:33] 
[01:06:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:33] 
[01:06:33] 
[01:06:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:33] Build completed unsuccessfully in 0:18:48
[01:06:33] Build completed unsuccessfully in 0:18:48
[01:06:33] Makefile:58: recipe for target 'check' failed
[01:06:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03da2872
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
