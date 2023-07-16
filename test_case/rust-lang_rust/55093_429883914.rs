plain
[00:51:25] .................................................................................................... 2200/4605
[00:51:29] ..................i................................................................................. 2300/4605
[00:51:33] .................................................................................................... 2400/4605
[00:51:37] .................................................................................................... 2500/4605
[00:51:41] ...............................iiiiiiiii............................................................ 2600/4605
[00:51:47] .................................................................................................... 2800/4605
[00:51:52] .................................................................................................... 2900/4605
[00:51:55] ......................................................i............................................. 3000/4605
[00:51:58] .................................................................................................... 3100/4605
---
[01:05:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:05:25] .F..............................................
[01:05:25] failures:
[01:05:25] 
[01:05:25] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:05:25] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:05:25] Current block: None
[01:05:25] Actual Line: "        AscribeUserType(_4, o, Ty(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }));"
[01:05:25] Expected Line: "       AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });"
[01:05:25] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:05:25] ... (elided)
[01:05:25]    bb0: {
[01:05:25]    bb0: {
[01:05:25]        StorageLive(_1);
[01:05:25]        _1 = const false;
[01:05:25]        FakeRead(ForLet, _1);
[01:05:25]        StorageLive(_2);
[01:05:25]        StorageLive(_3);
[01:05:25]        _3 = _1;
[01:05:25]        _2 = move _3;
[01:05:25]        StorageDead(_3);
[01:05:25]        StorageLive(_4);
[01:05:25]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:05:25]        FakeRead(ForLet, _4);
[01:05:25]        AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:05:25]        StorageLive(_5);
[01:05:25]        StorageLive(_6);
[01:05:25]        _6 = move _4;
[01:05:25]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:05:25] ... (elided)
[01:05:25]    bb2: {
[01:05:25]    bb2: {
[01:05:25]        drop(_6) -> [return: bb6, unwind: bb4];
[01:05:25] ... (elided)
[01:05:25]    bb5: {
[01:05:25]        drop(_6) -> bb4;
[01:05:25]    }
[01:05:25]    }
[01:05:25] Actual:
[01:05:25] fn main() -> (){
[01:05:25]     let mut _0: ();
[01:05:25]     scope 1 {
[01:05:25]         scope 3 {
[01:05:25]             scope 5 {
[01:05:25]                 scope 7 {
[01:05:25]                 scope 8 {
[01:05:25]                 scope 8 {
[01:05:25]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:05:25]             }
[01:05:25]             scope 6 {
[01:05:25]             scope 6 {
[01:05:25]                 let _4: std::option::Option<std::boxed::Box<u32>> as (Ty(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }), /checkout/src/test/mir-opt/basic_assignment.rs:37:18: 37:34);
[01:05:25]         }
[01:05:25]         scope 4 {
[01:05:25]         scope 4 {
[01:05:25]             let _2: bool;
[01:05:25]     }
[01:05:25]     scope 2 {
[01:05:25]     scope 2 {
[01:05:25]         let _1: bool;
[01:05:25]     }
[01:05:25]     let mut _3: bool;
[01:05:25]     let mut _6: std25]     }
[01:05:25]     bb8: {                              
[01:05:25]         StorageDead(_4);
[01:05:25]         StorageDead(_2);
[01:05:25]         StorageDead(_1);
[01:05:25]         return;
[01:05:25] }', tools/compiletest/src/runtest.rs:2932:13
[01:05:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:25] 
[01:05:25] 
[01:05:25] 
[01:05:25] failures:
[01:05:25]     [mir-opt] mir-opt/basic_assignment.rs
[01:05:25] 
[01:05:25] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:05:25] 
[01:05:25] 
[01:05:25] 
[01:05:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:25] 
[01:05:25] 
[01:05:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:25] Build completed unsuccessfully in 0:18:57
[01:05:25] Build completed unsuccessfully in 0:18:57
[01:05:25] make: *** [check] Error 1
[01:05:25] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08bd2490
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00cc59fb:start=1539614915685238599,finish=1539614915689433081,duration=4194482
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1159acde
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0985ca1e
travis_time:start:0985ca1e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05964000
$ dmesg | grep -i kill
