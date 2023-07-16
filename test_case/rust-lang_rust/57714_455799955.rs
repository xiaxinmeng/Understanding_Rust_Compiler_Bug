plain
travis_time:end:25cf54d6:start=1547915537305158192,finish=1547915539544887434,duration=2239729242
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
[01:08:34] 
[01:08:34] running 38 tests
[01:08:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:08:54] ...............................F......
[01:08:54] 
[01:08:54] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:08:54] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:08:54] thread '[mir-opt] mir-opt/retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:08:54] Expected Line: "        _14 = &mut (*_10);"
[01:08:54] Test Name: rustc.main.EraseRegions.after.mir
[01:08:54] ... (elided)
[01:08:54] fn main() -> () {
[01:08:54] ... (elided)
[01:08:54]     bb0: {
[01:08:54]     bb0: {
[01:08:54] ... (elided)
[01:08:54]         _3 = const Test::foo(move _4, move _6) -> bb1;
[01:08:54]     bb1: {
[01:08:54]         Retag(_3);
[01:08:54] ... (elided)
[01:08:54]         _9 = move _3;
[01:08:54]         _9 = move _3;
[01:08:54]         Retag(_9);
[01:08:54]         _8 = &mut (*_9);
[01:08:54]         Retag(_8);
[01:08:54]         StorageDead(_9);
[01:08:54]         StorageLive(_10);
[01:08:54]         _10 = move _8;
[01:08:54]         Retag(_10);
[01:08:54] ... (elided)
[01:08:54]         _14 = &mut (*_10);
[01:08:54]         Retag(_14);
[01:08:54]         _13 = move _14 as *mut i32 (Misc);
[01:08:54]         Retag([raw] _13);
[01:08:54] ... (elided)
[01:08:54]         _17 = move _18(move _19) -> bb2;
[01:08:54]     bb2: {
[01:08:54]         Retag(_17);
[01:08:54] ... (elided)
[01:08:54] ... (elided)
[01:08:54]         _21 = const Test::foo_shr(move _22, move _24) -> bb3;
[01:08:54]     bb3: {
[01:08:54] ... (elided)
[01:08:54]         return;
[01:08:54]     }
[01:08:54]     }
[01:08:54] ... (elided)
[01:08:54] }
[01:08:54] Actual:
[01:08:54] | User Type Annotations
[01:08:54] | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*mut ^0) } at /checkout/src/test/mir-opt/retag.rs:21:18: 21:29
[01:08:54] | 1: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:25:12: 25:28
[01:08:54] | 2: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:25:12: 25:28
[01:08:54] | 3: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*const ^0) } at /checkout/src/test/mir-opt/retag.rs:32:14: 32:28
[01:08:54] fn main() -> (){
[01:08:54] fn main() -> (){
[01:08:54]     let mut _0: ();
[01:08:54]     scope 1 {
[01:08:54]         scope 3 {
[01:08:54]             scope 5 {
[01:08:54]                 scope 7 {
[01:08:54]                     scope 9 {
[01:08:54]                     scope 10 {
[01:08:54]                     scope 10 {
[01:08:54]                         let _11: *mut i32;
[01:08:54]                 }
[01:08:54]                 scope 8 {
[01:08:54]                     let _10: &mut i32;
[01:08:54]                 }
---
[01:08:54]         scope 11 {
[01:08:54]             scope 13 {
[01:08:54]                 scope 15 {
[01:08:54]                 }
[01:08:54]                 scope 16 {
[01:08:54]                     let _28: *const i32;
[01:08:54]             }
[01:08:54]             scope 14 {
[01:08:54]                 let _18: &i32;
[01:08:54]             }
[01:08:54]             }
[01:08:54]         }
[01:08:54]         scope 12 {
[01:08:54]             let _16: for<'r> fn(&'r i32) -> &'r i32 as UserTypeProjection { base: UserType(1), projs: [] };
[01:08:54]     }
[01:08:54]     scope 2 {
[01:08:54]         let mut _1: i32;
[01:08:54]     }
[01:08:54]     }
[01:08:54]     let mut _2: ();
[01:08:54]     let mut _4: &Test;
[01:08:54]     let _5: Test;
[01:08:54]     let mut _6: &mut i32;
[01:08:54]     let mut _7: &mut i32;
[01:08:54]     let mut _9: &mut i32;
[01:08:54]     let mut _12: *mut i32;
[01:08:54]     let mut _13: *mut i32;
[01:08:54]     let mut _14: *mut i32;
[01:08:54]     let mut _15: &mut i32;
[01:08:54]     let mut _17: [closure@NodeId(124)];
[01:08:54]     let mut _19: for<'r> fn(&'r i32) -> &'r i32;
[01:08:54]     let mut _20: &i32;
[01:08:54]     let mut _21: &i32;
[01:08:54]     let mut _22: &i32;
[01:08:54]     let mut _23: &Test;
[01:08:54]     let _24: Test;
[01:08:54]     let mut _25: &i32;
[01:08:54]     let mut _26: &i32;
[01:08:54]     let _27: i32;
[01:08:54]     let mut _29: *const i32;
[01:08:54]     let mut _30: *const i32;
[01:08:54]     let mut _31: *const i32;
[01:08:54]     let mut _32: &i32;
[01:08:54]     bb0: {                              
[01:08:54]         StorageLive(_1);
[01:08:54]         _1 = const 0i32;
[01:08:54]         StorageLive(_3);
[01:08:54]         StorageLive(_4);
[01:08:54]         _4 = &(promoted[2]: Test);
[01:08:54]         Retag(_4);
[01:08:54]         StorageLive(_6);
[01:08:54]         StorageLive(_7);
[01:08:54]         _7 = &mut _1;
[01:08:54]         Retag(_7);
[01:08:54]         _6 = &mut (*_7);
[01:08:54]         Retag([2phase] _6);
[01:08:54]         _3 = const Test::foo(move _4, move _6) -> bb1;
[01:08:54]     }
[01:08:54]     bb1: {                              
[01:08:54]         Retag(_3);
[01:08:54]         StorageDead(_6);
[01:08:54]         StorageDead(_4);
[01:08:54]         StorageDead(_7);
[01:08:54]         StorageLive(_8);
[01:08:54]         StorageLive(_9);
[01:08:54]         _9 = move _3;
[01:08:54]         Retag(_9);
[01:08:54]         _8 = &mut (*_9);
[01:08:54]         Retag(_8);
[01:08:54]         StorageDead(_9);
[01:08:54]         StorageLive(_10);
[01:08:54]         _10 = move _8;
[01:08:54]         Retag(_10);
[01:08:54]         StorageLive(_11);
[01:08:54]         StorageLive(_12);
[01:08:54]         StorageLive(_13);
[01:08:54]         StorageLive(_14);
[01:08:54]         StorageLive(_15);
[01:08:54]         _15 = &mut (*_10);
[01:08:54]         Retag(_15);
[01:08:54]         _14 = move _15 as *mut i32 (Misc);
[01:08:54]         Retag([raw] _14);
[01:08:54]         _13 = move _14;
[01:08:54]         StorageDead(_14);
[01:08:54]         StorageDead(_15);
[01:08:54]         _12 = move _13;
[01:08:54]         _11 = _12;
[01:08:54]         StorageDead(_13);
[01:08:54]         StorageDead(_12);
[01:08:54]         _2 = ();
[01:08:54]         StorageDead(_11);
[01:08:54]         StorageDead(_10);
[01:08:54]         StorageDead(_8);
[01:08:54]         StorageDead(_3);
[01:08:54]         StorageLive(_16);
[01:08:54]         StorageLive(_17);
[01:08:54]         _17 = [closure@NodeId(124)];
[01:08:54]         Retag(_17);
[01:08:54]         _16 = move _17 as for<'r> fn(&'r i32) -> &'r i32 (ClosureFnPointer);
[01:08:54]         StorageDead(_17);
[01:08:54]         StorageLive(_18);
[01:08:54]         StorageLive(_19);
[01:08:54]         _19 = _16;
[01:08:54]         StorageLive(_20);
[01:08:54]         StorageLive(_21);
[01:08:54]         _21 = &_1;
[01:08:54]         Retag(_21);
[01:08:54]         _20 = &(*_21);
[01:08:54]         Retag(_20);
[01:08:54]         _18 = move _19(move _20) -> bb2;
[01:08:54]     }
[01:08:54]     bb2: {                              
[01:08:54]         Retag(_18);
[01:08:54]         StorageDead(_20);
[01:08:54]         StorageDead(_19);
[01:08:54]         StorageDead(_21);
[01:08:54]         StorageLive(_23);
[01:08:54]         _23 = &(promoted[1]: Test);
[01:08:54]         Retag(_23);
[01:08:54]         StorageLive(_25);
[01:08:54]         StorageLive(_26);
[01:08:54]         _26 = &(promoted[0]: i32);
[01:08:54]         Retag(_26);
[01:08:54]         _25 = &(*_26);
[01:08:54]         Retag(_25);
[01:08:54]         _22 = const Test::foo_shr(move _23, move _25) -> bb3;
[01:08:54]     }
[01:08:54]     bb3: {                              
[01:08:54]         Retag(_22);
[01:08:54]         StorageDead(_25);
[01:08:54]         StorageDead(_23);
[01:08:54]         StorageDead(_26);
[01:08:54]         StorageLive(_28);
[01:08:54]         StorageLive(_29);
[01:08:54]         StorageLive(_30);
[01:08:54]         StorageLive(_31);
[01:08:54]         StorageLive(_32);
[01:08:54]         _32 = &(*_18);
[01:08:54]         Retag(_32);
[01:08:54]         _31 = move _32 as *const i32 (Misc);
[01:08:54]         Retag([raw] _31);
[01:08:54]         _30 = move _31;
[01:08:54]         StorageDead(_31);
[01:08:54]         StorageDead(_32);
[01:08:54]         _29 = move _30;
[01:08:54]         _28 = _29;
[01:08:54]         StorageDead(_30);
[01:08:54]         StorageDead(_29);
[01:08:54]         _0 = ();
[01:08:54]         StorageDead(_28);
[01:08:54]         StorageDead(_18);
[01:08:54]         StorageDead(_16);
[01:08:54]         StorageDead(_1);
[01:08:54]         return;
[01:08:54] }', src/tools/compiletest/src/runtest.rs:2910:13
[01:08:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:54] 
[01:08:54] 
[01:08:54] 
[01:08:54] failures:
[01:08:54]     [mir-opt] mir-opt/retag.rs
[01:08:54] 
[01:08:54] test result: FAILED. 37 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:08:54] 
[01:08:54] 
[01:08:54] 
[01:08:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:54] 
[01:08:54] 
[01:08:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:54] Build completed unsuccessfully in 0:10:44
[01:08:54] Build completed unsuccessfully in 0:10:44
[01:08:54] Makefile:48: recipe for target 'check' failed
[01:08:54] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05b86e27
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 17:41:25 UTC 2019
---
travis_time:end:033ba189:start=1547919686513960118,finish=1547919686562087063,duration=48126945
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a072d31
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:106d4191
$ dmesg | grep -i kill
