plain
travis_time:end:23bf3ddb:start=1548606586968328481,finish=1548606666831744968,duration=79863416487
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:10:02] running 38 tests
[01:10:06] ERROR 2019-01-27T17:41:21Z: compiletest::runtest: None
[01:10:06] ERROR 2019-01-27T17:41:21Z: compiletest::runtest: None
[01:10:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:10:20] ...........FF.........................
[01:10:20] 
[01:10:20] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:10:20] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:10:20] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:10:20] Current block: None
[01:10:20] Actual Line: "        _0 = const std::ops::Fn::call(move _4, move _5) -> bb1;"
[01:10:20] Expected Line: "    _8 = move (_5.0: &i32);"
[01:10:20] Test Name: rustc.foo.Inline.after.mir
[01:10:20] ... (elided)
[01:10:20] ... (elided)
[01:10:20] bb0: {
[01:10:20] ... (elided)
[01:10:20] ... (elided)
[01:10:20]     _3 = [closure@NodeId(53)];
[01:10:20] ... (elided)
[01:10:20]     _4 = &_3;
[01:10:20] ... (elided)
[01:10:20]     _6 = &(*_2);
[01:10:20] ... (elided)
[01:10:20]     _7 = &(*_2);
[01:10:20]     _5 = (move _6, move _7);
[01:10:20]     _8 = move (_5.0: &i32);
[01:10:20]     _9 = move (_5.1: &i32);
[01:10:20] ... (elided)
[01:10:20]     _0 = (*_8);
[01:10:20] ... (elided)
[01:10:20]     return;
[01:10:20] }
[01:10:20] ... (elided)
[01:10:20] Actual:
[01:10:20] fn foo(_1: T, _2: &i32) -> i32{
[01:10:20]     let mut _0: i32;
[01:10:20]     scope 1 {
[01:10:20]     scope 2 {
[01:10:20]     scope 2 {
[01:10:20]         let _3: [closure@NodeId(53)];
[01:10:20]     }
[01:10:20]     let mut _4: &[closure@NodeId(53)];
[01:10:20]     let mut _5: (&i32, &i32);
[01:10:20]     let mut _6: &i32;
[01:10:20]     let mut _7: &i32;
[01:10:20]     bb0: {                              
[01:10:20]         StorageLive(_3);
[01:10:20]         _3 = [closure@NodeId(53)];
[01:10:20]         StorageLive(_4);
[01:10:20]         _4 = &_3;
[01:10:20]         StorageLive(_5);
[01:10:20]         StorageLive(_6);
[01:10:20]         _6 = &(*_2);
[01:10:20]         StorageLive(_7);
[01:10:20]         _7 = &(*_2);
[01:10:20]         _5 = (move _6, move _7);
[01:10:20]         _0 = const std::ops::Fn::call(move _4, move _5) -> bb1;
[01:10:20]     }
[01:10:20]     bb1: {                              
[01:10:20]         StorageDead(_5);
[01:10:20]         StorageDead(_7);
[01:10:20]         StorageDead(_6);
[01:10:20]         StorageDead(_4);
[01:10:20]         StorageDead(_3);
[01:10:20]         return;
[01:10:20]     bb2: {
[01:10:20]         resume;
[01:10:20]     }
[01:10:20] }', src/tools/compiletest/src/runtest.rs:2953:13
[01:10:20] }', src/tools/compiletest/src/runtest.rs:2953:13
[01:10:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:10:20] 
[01:10:20] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:10:20] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:10:20] Current block: None
[01:10:20] Actual Line: "        _0 = const std::ops::Fn::call(move _4, move _5) -> bb1;"
[01:10:20] Expected Line: "    _8 = move (_5.0: i32);"
[01:10:20] Test Name: rustc.foo.Inline.after.mir
[01:10:20] ... (elided)
[01:10:20] ... (elided)
[01:10:20] bb0: {
[01:10:20] ... (elided)
[01:10:20] ... (elided)
[01:10:20]     _3 = [closure@NodeId(39)];
[01:10:20] ... (elided)
[01:10:20]     _4 = &_3;
[01:10:20] ... (elided)
[01:10:20]     _6 = _2;
[01:10:20] ... (elided)
[01:10:20]     _7 = _2;
[01:10:20]     _5 = (move _6, move _7);
[01:10:20]     _8 = move (_5.0: i32);
[01:10:20]     _9 = move (_5.1: i32);
[01:10:20]     _0 = _8;
[01:10:20] ... (elided)
[01:10:20]     return;
[01:10:20] }
[01:10:20] ... (elided)
[01:10:20] Actual:
[01:10:20] fn foo(_1: T, _2: i32) -> i32{
[01:10:20]     let mut _0: i32;
[01:10:20]     scope 1 {
[01:10:20]     scope 2 {
[01:10:20]     scope 2 {
[01:10:20]         let _3: [closure@NodeId(39)];
[01:10:20]     }
[01:10:20]     let mut _4: &[closure@NodeId(39)];
[01:10:20]     let mut _5: (i32, i32);
[01:10:20]     let mut _6: i32;
[01:10:20]     let mut _7: i32;
[01:10:20]     bb0: {                              
[01:10:20]         StorageLive(_3);
[01:10:20]         _3 = [closure@NodeId(39)];
[01:10:20]         StorageLive(_4);
[01:10:20]         _4 = &_3;
[01:10:20]         StorageLive(_5);
[01:10:20]         StorageLive(_6);
[01:10:20]         _6 = _2;
[01:10:20]         StorageLive(_7);
[01:10:20]         _7 = _2;
[01:10:20]         _5 = (move _6, move _7);
[01:10:20]         _0 = const std::ops::Fn::call(move _4, move _5) -> bb1;
[01:10:20]     }
[01:10:20]     bb1: {                              
[01:10:20]         StorageDead(_5);
[01:10:20]         StorageDead(_7);
[01:10:20]         StorageDead(_6);
[01:10:20]         StorageDead(_4);
[01:10:20]         StorageDead(_3);
[01:10:20]         return;
[01:10:20]     bb2: {
[01:10:20]         resume;
[01:10:20]     }
[01:10:20] }', src/tools/compiletest/src/runtest.rs:2953:13
---
[01:10:20] test result: FAILED. 36 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:20] 
[01:10:20] 
[01:10:20] 
[01:10:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:20] 
[01:10:20] 
[01:10:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:20] Build completed unsuccessfully in 0:10:59
[01:10:20] Build completed unsuccessfully in 0:10:59
[01:10:20] make: *** [check] Error 1
[01:10:20] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23782b9c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 27 17:41:36 UTC 2019
---
travis_time:end:00034608:start=1548610897979051224,finish=1548610897983303605,duration=4252381
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01f910f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d0c0070
travis_time:start:1d0c0070
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08c08b02
$ dmesg | grep -i kill
