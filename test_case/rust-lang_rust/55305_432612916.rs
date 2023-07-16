plain
travis_time:end:0503c469:start=1540375314282352514,finish=1540375316541289161,duration=2258936647
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
Setting environment variables from repository settings
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
---
[00:50:56] .................................................................................................... 2200/4943
[00:51:01] .................................................................................................... 2300/4943
[00:51:05] .................................................................................................... 2400/4943
[00:51:09] .................................................................................................... 2500/4943
[00:51:12] ................................................iiiiiiiii........................................... 2600/4943
[00:51:16] ..................................................................................................ii 2700/4943
[00:51:23] .................................................................................................... 2900/4943
[00:51:26] ........................................................................................i........... 3000/4943
[00:51:28] .................................................................................................... 3100/4943
[00:51:31] ...............................................i.i..ii.............................................. 3200/4943
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:05] 
[01:04:05] running 48 tests
[01:04:05] ERROR 2018-10-24T11:06:11Z: compiletest::runtest: None
ead(_3);
[01:04:25]        StorageLive(_4);
[01:04:25]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:04:25]        FakeRead(ForLet, _4);
[01:04:25]        AscribeUserType(_4, o, Ty(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }));
[01:04:25]        StorageLive(_5);
[01:04:25]        StorageLive(_6);
[01:04:25]        _6 = move _4;
[01:04:25]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:04:25] ... (elided)
[01:04:25]    bb2: {
[01:04:25]    bb2: {
[01:04:25]        drop(_6) -> [return: bb6, unwind: bb4];
[01:04:25] ... (elided)
[01:04:25]    bb5: {
[01:04:25]        drop(_6) -> bb4;
[01:04:25]    }
[01:04:25]    }
[01:04:25] Actual:
[01:04:25] fn main() -> (){
[01:04:25]     let mut _0: ();
[01:04:25]     scope 1 {
[01:04:25]         scope 3 {
[01:04:25]             scope 5 {
[01:04:25]                 scope 7 {
[01:04:25]                 scope 8 {
[01:04:25]                 scope 8 {
[01:04:25]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:04:25]             }
[01:04:25]             scope 6 {
[01:04:25]             scope 6 {
[01:04:25]                 let _4: std::option::Option<std::boxed::Box<u32>> as (Ty(Canonical { max_universe: U0, variables: [], value: std::option::Option<std::boxed::Box<u32>> }), /checkout/src/test/mir-opt/basic_assignment.rs:37:18: 37:34);
[01:04:25]         }
[01:04:25]         scope 4 {
[01:04:25]         scope 4 {
[01:04:25]             let _2: bool;
[01:04:25]     }
[01:04:25]     scope 2 {
[01:04:25]     scope 2 {
[01:04:25]         let _1: bool;
[01:04:25]     }
[01:04:25]     let mut _3: bool;
[01:04:25]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:04:25]     bb0: {                              
[01:04:25]         StorageLive(_1);
[01:04:25]         _1 = const false;
[01:04:25]         FakeRead(ForLet, _1);
[01:04:25]         StorageLive(_2);
[01:04:25]         StorageLive(_3);
[01:04:25]         _3 = _1;
[01:04:25]         _2 = move _3;
[01:04:25]         StorageDead(_3);
[01:04:25]         StorageLive(_4);
[01:04:25]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:04:25]         FakeRead(ForLet, _4);
[01:04:25]         AscribeUserType(_4, o, Ty(Canonical { max_universe: U0, variables: [], value: std::option::Option<std::boxed::Box<u32>> }));
[01:04:25]         StorageLive(_5);
[01:04:25]         StorageLive(_6);
[01:04:25]         _6 = move _4;
[01:04:25]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:04:25]     bb1: {
[01:04:25]         resume;
[01:04:25]     }
[01:04:25]     }
[01:04:25]     bb2: {                              
[01:04:25]         drop(_6) -> [return: bb6, unwind: bb4];
[01:04:25]     bb3: {
[01:04:25]         drop(_4) -> bb1;
[01:04:25]     }
[01:04:25]     bb4: {
[01:04:25]     bb4: {
[01:04:25]         drop(_5) -> bb3;
[01:04:25]     }
[01:04:25]     bb5: {
[01:04:25]         drop(_6) -> bb4;
[01:04:25]     }
[01:04:25]     bb6: {                              
[01:04:25]         StorageDead(_6);
[01:04:25]         _0 = ();
[01:04:25]         drop(_5) -> [return: bb7, unwind: bb3];
[01:04:25]     }
[01:04:25]     bb7: {                              
[01:04:25]         StorageDead(_5);
[01:04:25]         drop(_4) -> [return: bb8, unwind: bb1];
[01:04:25]     }
[01:04:25]     bb8: {                              
[01:04:25]         StorageDead(_4);
[01:04:25]         StorageDead(_2);
[01:04:25]         StorageDead(_1);
[01:04:25]         return;
[01:04:25] }', tools/compiletest/src/runtest.rs:2939:13
[01:04:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:25] 
[01:04:25] 
[01:04:25] 
[01:04:25] failures:
[01:04:25]     [mir-opt] mir-opt/basic_assignment.rs
[01:04:25] 
[01:04:25] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:25] 
[01:04:25] 
[01:04:25] 
[01:04:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:25] 
[01:04:25] 
[01:04:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:25] Build completed unsuccessfully in 0:18:17
---
38056 ./obj/build/x86_64-unknown-linux-gnu/test/ui/issues
37756 ./src/tools/lldb/www
37524 ./obj/build/x86_64-unknown-linux-gnu/doc/core/arch
37520 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
364\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05d40728
travis_time:start:05d40728
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:306b287c
$ dmesg | grep -i kill
