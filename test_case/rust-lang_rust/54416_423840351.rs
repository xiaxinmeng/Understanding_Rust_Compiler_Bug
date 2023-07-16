plain
[01:21:36] test [mir-opt] mir-opt/lower_128bit_debug_test.rs ... ok
[01:21:36] 
[01:21:36] failures:
[01:21:36] 
[01:21:36] ---- [mir-opt] mir-opt/inline-any-operand.rs stdout ----
[01:21:36] thread '[mir-opt] mir-opt/inline-any-operand.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:21:36] Expected Line: "    _0 = Add(move _3, move _4);"
[01:21:36] Test Name: rustc.bar.Inline.after.mir
[01:21:36] ... (elided)
[01:21:36] ... (elided)
[01:21:36] bb0: {
[01:21:36] ... (elided)
[01:21:36] ... (elided)
[01:21:36]     _0 = Add(move _3, move _4);
[01:21:36] ... (elided)
[01:21:36]     return;
[01:21:36] }
[01:21:36] ... (elided)
[01:21:36] Actual:
[01:21:36] fn bar() -> i32{
[01:21:36]     let mut _0: i32;
[01:21:36]     scope 1 {
[01:21:36]         scope 3 {
[01:21:36]     }
[01:21:36]     scope 2 {
[01:21:36]     scope 2 {
[01:21:36]         let _1: fn(i32, i32) -> i32 {foo};
[01:21:36]     }
[01:21:36]     let mut _2: fn(i32, i32) -> i32 {foo};
[01:21:36]     let mut _3: (i32, bool);
[01:21:36]     let mut _4: i32;
[01:21:36]     let mut _5: i32;
[01:21:36]     bb0: {                              
[01:21:36]         StorageLive(_1);
[01:21:36]         _1 = const foo;
[01:21:36]         FakeRead(ForLet, _1);
[01:21:36]         StorageLive(_2);
[01:21:36]         _2 = _1;
[01:21:36]         _4 = const 1i32;
[01:21:36]         _5 = const -1i32;
[01:21:36]         _3 = CheckedAdd(move _4, move _5);
[01:21:36]         assert(!move (_3.1: bool), "attempt to add with overflow") -> bb1;
[01:21:36]     }
[01:21:36]     bb1: {                              
[01:21:36]         _0 = move (_3.0: i32);
[01:21:36]         StorageDead(_2);
[01:21:36]         StorageDead(_1);
[01:21:36]         return;
[01:21:36] }', tools/compiletest/src/runtest.rs:2893:13
[01:21:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:21:36] 
[01:21:36] 
---
[01:21:36] 
[01:21:36] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:21:36] 
[01:21:36] 
[01:21:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:36] 
[01:21:36] 
[01:21:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:36] Build completed unsuccessfully in 0:17:33
[01:21:36] Build completed unsuccessfully in 0:17:33
[01:21:36] make: *** [check] Error 1
[01:21:36] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ace4c80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3a03c314:start=1537730004150473770,finish=1537730004156518305,duration=6044535
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06379c15
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:37001098
travis_time:start:37001098
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0aa4c324
$ dmesg | grep -i kill
