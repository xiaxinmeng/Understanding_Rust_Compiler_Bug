plain
travis_time:end:07f6d26e:start=1548518452449537595,finish=1548518454789505962,duration=2339968367
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:05] 
[01:14:05] running 38 tests
[01:14:30] ..........F..F..F.....................
[01:14:30] failures:
[01:14:30] 
[01:14:30] ---- [mir-opt] mir-opt/inline-any-operand.rs stdout ----
[01:14:30] ---- [mir-opt] mir-opt/inline-any-operand.rs stdout ----
[01:14:30] thread '[mir-opt] mir-opt/inline-any-operand.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:14:30] Expected Line: "    _0 = Eq(move _3, move _4);"
[01:14:30] Test Name: rustc.bar.Inline.after.mir
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30] bb0: {
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30]     _0 = Eq(move _3, move _4);
[01:14:30] ... (elided)
[01:14:30]     return;
[01:14:30] }
[01:14:30] ... (elided)
[01:14:30] Actual:
[01:14:30] fn bar() -> bool{
[01:14:30]     let mut _0: bool;
[01:14:30]     scope 1 {
[01:14:30]     scope 2 {
[01:14:30]     scope 2 {
[01:14:30]         let _1: fn(i32, i32) -> bool {foo};
[01:14:30]     }
[01:14:30]     let mut _2: fn(i32, i32) -> bool {foo};
[01:14:30]     bb0: {                              
[01:14:30]         StorageLive(_1);
[01:14:30]         _1 = const foo;
[01:14:30]         StorageLive(_2);
[01:14:30]         _2 = _1;
[01:14:30]         _0 = move _2(const 1i32, const -1i32) -> bb1;
[01:14:30]     }
[01:14:30]     bb1: {                              
[01:14:30]         StorageDead(_2);
[01:14:30]         StorageDead(_1);
[01:14:30]         return;
[01:14:30]     bb2: {
[01:14:30]         resume;
[01:14:30]     }
[01:14:30] }', src/tools/compiletest/src/runtest.rs:2952:13
[01:14:30] }', src/tools/compiletest/src/runtest.rs:2952:13
[01:14:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:14:30] 
[01:14:30] ---- [mir-opt] mir-opt/inline-retag.rs stdout ----
[01:14:30] thread '[mir-opt] mir-opt/inline-retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:14:30] Expected Line: "        Retag(_3);"
[01:14:30] Test Name: rustc.bar.Inline.after.mir
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30]     bb0: {
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30]         Retag(_3);
[01:14:30] ... (elided)
[01:14:30]         Retag(_3);
[01:14:30]         Retag(_6);
[01:14:30]         StorageLive(_9);
[01:14:30]         _9 = (*_3);
[01:14:30]         StorageLive(_10);
[01:14:30]         _10 = (*_6);
[01:14:30]         _0 = Eq(move _9, move _10);
[01:14:30] ... (elided)
[01:14:30]         return;
[01:14:30] ... (elided)
[01:14:30] Actual:
[01:14:30] fn bar() -> bool{
[01:14:30] fn bar() -> bool{
[01:14:30]     let mut _0: bool;
[01:14:30]     scope 1 {
[01:14:30]     scope 2 {
[01:14:30]     scope 2 {
[01:14:30]         let _1: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:14:30]     }
[01:14:30]     let mut _2: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:14:30]     let mut _3: &i32;
[01:14:30]     let mut _4: &i32;
[01:14:30]     let _5: i32;
[01:14:30]     let mut _6: &i32;
[01:14:30]     let mut _7: &i32;
[01:14:30]     let _8: i32;
[01:14:30]     bb0: {                              
[01:14:30]         StorageLive(_1);
[01:14:30]         _1 = const foo;
[01:14:30]         StorageLive(_2);
[01:14:30]         _2 = _1;
[01:14:30]         StorageLive(_3);
[01:14:30]         StorageLive(_4);
[01:14:30]         _4 = &(promoted[1]: i32);
[01:14:30]         Retag(_4);
[01:14:30]         _3 = &(*_4);
[01:14:30]         Retag(_3);
[01:14:30]         StorageLive(_6);
[01:14:30]         StorageLive(_7);
[01:14:30]         _7 = &(promoted[0]: i32);
[01:14:30]         Retag(_7);
[01:14:30]         _6 = &(*_7);
[01:14:30]         Retag(_6);
[01:14:30]         _0 = move _2(move _3, move _6) -> bb1;
[01:14:30]     }
[01:14:30]     bb1: {                              
[01:14:30]         StorageDead(_6);
[01:14:30]         StorageDead(_3);
[01:14:30]         StorageDead(_2);
[01:14:30]         StorageDead(_1);
[01:14:30]         StorageDead(_7);
[01:14:30]         StorageDead(_4);
[01:14:30]         return;
[01:14:30]     bb2: {
[01:14:30]         resume;
[01:14:30]     }
[01:14:30] }', src/tools/compiletest/src/runtest.rs:2952:13
[01:14:30] }', src/tools/compiletest/src/runtest.rs:2952:13
[01:14:30] 
[01:14:30] ---- [mir-opt] mir-opt/inline-trait-method_2.rs stdout ----
[01:14:30] thread '[mir-opt] mir-opt/inline-trait-method_2.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:14:30] Expected Line: "    _0 = const X::y(move _2) -> bb1;"
[01:14:30] Test Name: rustc.test2.Inline.after.mir
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30] bb0: {
[01:14:30] ... (elided)
[01:14:30] ... (elided)
[01:14:30]     _0 = const X::y(move _2) -> bb1;
[01:14:30] }
[01:14:30] ... (elided)
[01:14:30] Actual:
[01:14:30] fn test2(_1: &dyn X) -> bool{
[01:14:30]     let mut _0: bool;
[01:14:30]     let mut _2: &dyn X;
[01:14:30]     let mut _3: &dyn X;
[01:14:30]     bb0: {                              
[01:14:30]         StorageLive(_2);
[01:14:30]         StorageLive(_3);
[01:14:30]         _3 = &(*_1);
[01:14:30]         _2 = move _3 as &dyn X (Unsize);
[01:14:30]         StorageDead(_3);
[01:14:30]         _0 = const test(move _2) -> bb1;
[01:14:30]     }
[01:14:30]     bb1: {                              
[01:14:30]         StorageDead(_2);
[01:14:30]         return;
[01:14:30]     bb2: {
[01:14:30]         resume;
[01:14:30]     }
[01:14:30] }', src/tools/compiletest/src/runtest.rs:2952:13
---
[01:14:30] test result: FAILED. 35 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:30] 
[01:14:30] 
[01:14:30] 
[01:14:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:30] 
[01:14:30] 
[01:14:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:30] Build completed unsuccessfully in 0:11:59
[01:14:30] Build completed unsuccessfully in 0:11:59
[01:14:30] Makefile:48: recipe for target 'check' failed
[01:14:30] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aac967d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 17:15:36 UTC 2019
---
travis_time:end:07bc7418:start=1548522938540120707,finish=1548522938545873180,duration=5752473
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15853b70
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2449eb73
travis_time:start:2449eb73
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:327aacd3
$ dmesg | grep -i kill
