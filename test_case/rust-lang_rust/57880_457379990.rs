plain
travis_time:end:09734c46:start=1548363834218751662,finish=1548363835223353250,duration=1004601588
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:23] 
[01:15:23] running 38 tests
[01:15:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:15:42] .............F..F.....................
[01:15:42] 
[01:15:42] ---- [mir-opt] mir-opt/inline-retag.rs stdout ----
[01:15:42] ---- [mir-opt] mir-opt/inline-retag.rs stdout ----
[01:15:42] thread '[mir-opt] mir-opt/inline-retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:15:42] Expected Line: "        Retag(_3);"
[01:15:42] Test Name: rustc.bar.Inline.after.mir
[01:15:42] ... (elided)
[01:15:42] ... (elided)
[01:15:42]     bb0: {
[01:15:42] ... (elided)
[01:15:42] ... (elided)
[01:15:42]         Retag(_3);
[01:15:42] ... (elided)
[01:15:42]         Retag(_3);
[01:15:42]         Retag(_6);
[01:15:42]         StorageLive(_9);
[01:15:42]         _9 = (*_3);
[01:15:42]         StorageLive(_10);
[01:15:42]         _10 = (*_6);
[01:15:42]         _0 = Eq(move _9, move _10);
[01:15:42] ... (elided)
[01:15:42]         return;
[01:15:42] ... (elided)
[01:15:42] Actual:
[01:15:42] fn bar() -> bool{
[01:15:42] fn bar() -> bool{
[01:15:42]     let mut _0: bool;
[01:15:42]     scope 1 {
[01:15:42]         scope 3 {
[01:15:42]     }
[01:15:42]     scope 2 {
[01:15:42]     scope 2 {
[01:15:42]         let _1: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:15:42]     }
[01:15:42]     let mut _2: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:15:42]     let mut _3: &i32;
[01:15:42]     let mut _4: &i32;
[01:15:42]     let _5: i32;
[01:15:42]     let mut _6: &i32;
[01:15:42]     let mut _7: &i32;
[01:15:42]     let _8: i32;
[01:15:42]     let mut _9: i32;
[01:15:42]     let mut _10: i32;
[01:15:42]     bb0: {                              
[01:15:42]         StorageLive(_1);
[01:15:42]         _1 = const foo;
[01:15:42]         StorageLive(_2);
[01:15:42]         _2 = _1;
[01:15:42]         StorageLive(_3);
[01:15:42]         StorageLive(_4);
[01:15:42]         _4 = &(promoted[1]: i32);
[01:15:42]         Retag(_4);
[01:15:42]         _3 = &(*_4);
[01:15:42]         Retag(_3);
[01:15:42]         StorageLive(_6);
[01:15:42]         StorageLive(_7);
[01:15:42]         _7 = &(promoted[0]: i32);
[01:15:42]         Retag(_7);
[01:15:42]         _6 = &(*_7);
[01:15:42]         Retag(_6);
[01:15:42]         StorageLive(_9);
[01:15:42]         _9 = (*_3);
[01:15:42]         StorageLive(_10);
[01:15:42]         _10 = (*_6);
[01:15:42]         _0 = Eq(move _9, move _10);
[01:15:42]         StorageDead(_10);
[01:15:42]         StorageDead(_9);
[01:15:42]         StorageDead(_6);
[01:15:42]         StorageDead(_3);
[01:15:42]         StorageDead(_2);
[01:15:42]         StorageDead(_1);
[01:15:42]         StorageDead(_7);
[01:15:42]         StorageDead(_4);
[01:15:42]         return;
[01:15:42] }', src/tools/compiletest/src/runtest.rs:2910:13
[01:15:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:15:42] 
[01:15:42] ---- [mir-opt] mir-opt/inline-trait-method_2.rs stdout ----
[01:15:42] ---- [mir-opt] mir-opt/inline-trait-method_2.rs stdout ----
[01:15:42] thread '[mir-opt] mir-opt/inline-trait-method_2.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:15:42] Expected Line: "    _0 = const X::y(move _2) -> bb1;"
[01:15:42] Test Name: rustc.test2.Inline.after.mir
[01:15:42] ... (elided)
[01:15:42] ... (elided)
[01:15:42] bb0: {
[01:15:42] ... (elided)
[01:15:42] ... (elided)
[01:15:42]     _0 = const X::y(move _2) -> bb1;
[01:15:42] }
[01:15:42] ... (elided)
[01:15:42] Actual:
[01:15:42] fn test2(_1: &dyn X) -> bool{
[01:15:42]     let mut _0: bool;
[01:15:42]     scope 1 {
[01:15:42]     }
[01:15:42]     let mut _2: &dyn X;
[01:15:42]     let mut _3: &dyn X;
[01:15:42]     let mut _4: &dyn X;
[01:15:42]     bb0: {                              
[01:15:42]         StorageLive(_2);
[01:15:42]         StorageLive(_3);
[01:15:42]         _3 = &(*_1);
[01:15:42]         _2 = move _3 as &dyn X (Unsize);
[01:15:42]         StorageDead(_3);
[01:15:42]         StorageLive(_4);
[01:15:42]         _4 = &(*_2);
[01:15:42]         _0 = const X::y(move _4) -> [return: bb2, unwind: bb1];
[01:15:42]     bb1: {
[01:15:42]         resume;
[01:15:42]     }
[01:15:42]     }
[01:15:42]     bb2: {                              
[01:15:42]         StorageDead(_4);
[01:15:42]         StorageDead(_2);
[01:15:42]         return;
[01:15:42] }', src/tools/compiletest/src/runtest.rs:2910:13
[01:15:42] 
[01:15:42] 
[01:15:42] failures:
[01:15:42] failures:
[01:15:42]     [mir-opt] mir-opt/inline-retag.rs
[01:15:42]     [mir-opt] mir-opt/inline-trait-method_2.rs
[01:15:42] 
[01:15:42] test result: FAILED. 36 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:15:42] 
[01:15:42] 
[01:15:42] 
[01:15:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:42] 
[01:15:42] 
[01:15:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:42] Build completed unsuccessfully in 0:11:37
[01:15:42] Build completed unsuccessfully in 0:11:37
[01:15:42] Makefile:48: recipe for target 'check' failed
[01:15:42] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14b9f83c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 24 22:19:48 UTC 2019
