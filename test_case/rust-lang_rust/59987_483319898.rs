plain
travis_time:end:11aa6980:start=1555340456344902597,finish=1555340544012106544,duration=87667203947
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:55] 
[01:13:55] running 42 tests
[01:14:26] ERROR 2019-04-15T16:16:59Z: compiletest::runtest: Some("   bb0: {")
[01:14:28] .......................................F..
[01:14:28] failures:
[01:14:28] 
[01:14:28] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:14:28] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:14:28] thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:14:28] Current block:    bb0: {
[01:14:28] Actual Line: "        _3 = move _4 as &\'static [(u32, u32)] (Pointer(Unsize));"
[01:14:28] Expected Line: "       _3 = move _4 as &\'static [(u32, u32)] (Unsize);"
[01:14:28] Test Name: rustc.XXX.mir_map.0.mir
[01:14:28] ... (elided)
[01:14:28]    let mut _0: &'static Foo;
[01:14:28]    let mut _1: &'static Foo;
[01:14:28]    let _2: Foo;
[01:14:28]    let _2: Foo;
[01:14:28]    let mut _3: &'static [(u32, u32)];
[01:14:28]    let mut _4: &'static [(u32, u32); 42];
[01:14:28]    let mut _5: &'static [(u32, u32); 42];
[01:14:28]    let _6: [(u32, u32); 42];
[01:14:28]    let mut _7: (u32, u32);
[01:14:28]    let mut _8: (u32, u32);
[01:14:28]    let mut _9: (u32, u32);
[01:14:28]    let mut _10: (u32, u32);
[01:14:28]    let mut _11: (u32, u32);
[01:14:28]    let mut _12: (u32, u32);
[01:14:28]    let mut _13: (u32, u32);
[01:14:28]    let mut _14: (u32, u32);
[01:14:28]    let mut _15: (u32, u32);
[01:14:28]    let mut _16: (u32, u32);
[01:14:28]    let mut _17: (u32, u32);
[01:14:28]    let mut _18: (u32, u32);
[01:14:28]    let mut _19: (u32, u32);
[01:14:28]    let mut _20: (u32, u32);
[01:14:28]    let mut _21: (u32, u32);
[01:14:28]    let mut _22: (u32, u32);
[01:14:28]    let mut _23: (u32, u32);
[01:14:28]    let mut _24: (u32, u32);
[01:14:28]    let mut _25: (u32, u32);
[01:14:28]    let mut _26: (u32, u32);
[01:14:28]    let mut _27: (u32, u32);
[01:14:28]    let mut _28: (u32, u32);
[01:14:28]    let mut _29: (u32, u32);
[01:14:28]    let mut _30: (u32, u32);
[01:14:28]    let mut _31: (u32, u32);
[01:14:28]    let mut _32: (u32, u32);
[01:14:28]    let mut _33: (u32, u32);
[01:14:28]    let mut _34: (u32, u32);
[01:14:28]    let mut _35: (u32, u32);
[01:14:28]    let mut _36: (u32, u32);
[01:14:28]    let mut _37: (u32, u32);
[01:14:28]    let mut _38: (u32, u32);
[01:14:28]    let mut _39: (u32, u32);
[01:14:28]    let mut _40: (u32, u32);
[01:14:28]    let mut _41: (u32, u32);
[01:14:28]    let mut _42: (u32, u32);
[01:14:28]    let mut _43: (u32, u32);
[01:14:28]    let mut _44: (u32, u32);
[01:14:28]    let mut _45: (u32, u32);
[01:14:28]    let mut _46: (u32, u32);
[01:14:28]    let mut _47: (u32, u32);
[01:14:28]    let mut _48: (u32, u32);
[01:14:28]    bb0: {
[01:14:28]        StorageLive(_1);
[01:14:28]        StorageLive(_2);
[01:14:28]        StorageLive(_3);
[01:14:28]        StorageLive(_4);
[01:14:28]        StorageLive(_5);
[01:14:28]        StorageLive(_6);
[01:14:28]        StorageLive(_7);
[01:14:28]        _7 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_8);
[01:14:28]        _8 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_9);
[01:14:28]        _9 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_10);
[01:14:28]        _10 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_11);
[01:14:28]        _11 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_12);
[01:14:28]        _12 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_13);
[01:14:28]        _13 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_14);
[01:14:28]        _14 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_15);
[01:14:28]        _15 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_16);
[01:14:28]        _16 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_17);
[01:14:28]        _17 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_18);
[01:14:28]        _18 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_19);
[01:14:28]        _19 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_20);
[01:14:28]        _20 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_21);
[01:14:28]        _21 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_22);
[01:14:28]        _22 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_23);
[01:14:28]        _23 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_24);
[01:14:28]        _24 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_25);
[01:14:28]        _25 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_26);
[01:14:28]        _26 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_27);
[01:14:28]        _27 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_28);
[01:14:28]        _28 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_29);
[01:14:28]        _29 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_30);
[01:14:28]        _30 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_31);
[01:14:28]        _31 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_32);
[01:14:28]        _32 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_33);
[01:14:28]        _33 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_34);
[01:14:28]        _34 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_35);
[01:14:28]        _35 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_36);
[01:14:28]        _36 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_37);
[01:14:28]        _37 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_38);
[01:14:28]        _38 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_39);
[01:14:28]        _39 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_40);
[01:14:28]        _40 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_41);
[01:14:28]        _41 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_42);
[01:14:28]        _42 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_43);
[01:14:28]        _43 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_44);
[01:14:28]        _44 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_45);
[01:14:28]        _45 = (const 0u32, const 3u32);
[01:14:28]        StorageLive(_46);
[01:14:28]        _46 = (const 0u32, const 1u32);
[01:14:28]        StorageLive(_47);
[01:14:28]        _47 = (const 0u32, const 2u32);
[01:14:28]        StorageLive(_48);
[01:14:28]        _48 = (const 0u32, const 3u32);
[01:14:28]        _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
[01:14:28]        _5 = &_6;
[01:14:28]        _4 = &(*_5);
[01:14:28]        _3 = move _4 as &'static [(u32, u32)] (Unsize);
[01:14:28]        _2 = Foo { tup: const "hi", data: move _3 };
[01:14:28]        _1 = &_2;
[01:14:28]        _0 = &(*_1);
[01:14:28]        StorageDead(_1);
[01:14:28]        StorageDead(_5);
[01:14:28]        return;
[01:14:28] Actual:
[01:14:28] Actual:
[01:14:28] static  XXX: &'static Foo = {
[01:14:28]     let mut _0: &'static Foo;
[01:14:28]     let mut _1: &'static Foo;
[01:14:28]     let _2: Foo;
[01:14:28]     let mut _3: &'static [(u32, u32)];
[01:14:28]     let mut _4: &'static [(u32, u32); 42];
[01:14:28]     let mut _5: &'static [(u32, u32); 42];
[01:14:28]     let _6: [(u32, u32); 42];
[01:14:28]     let mut _7: (u32, u32);
[01:14:28]     let mut _8: (u32, u32);
[01:14:28]     let mut _9: (u32, u32);
[01:14:28]     let mut _10: (u32, u32);
[01:14:28]     let mut _11: (u32, u32);
[01:14:28]     let mut _12: (u32, u32);
[01:14:28]     let mut _13: (u32, u32);
[01:14:28]     let mut _14: (u32, u32);
[01:14:28]     let mut _15: (u32, u32);
[01:14:28]     let mut _16: (u32, u32);
[01:14:28]     let mut _17: (u32, u32);
[01:14:28]     let mut _18: (u32, u32);
[01:14:28]     let mut _19: (u32, u32);
[01:14:28]     let mut _20: (u32, u32);
[01:14:28]     let mut _21: (u32, u32);
[01:14:28]     let mut _22: (u32, u32);
[01:14:28]     let mut _23: (u32, u32);
[01:14:28]     let mut _24: (u32, u32);
[01:14:28]     let mut _25: (u32, u32);
[01:14:28]     let mut _26: (u32, u32);
[01:14:28]     let mut _27: (u32, u32);
[01:14:28]     let mut _28: (u32, u32);
[01:14:28]     let mut _29: (u32, u32);
[01:14:28]     let mut _30: (u32, u32);
[01:14:28]     let mut _31: (u32, u32);
[01:14:28]     let mut _32: (u32, u32);
[01:14:28]     let mut _33: (u32, u32);
[01:14:28]     let mut _34: (u32, u32);
[01:14:28]     let mut _35: (u32, u32);
[01:14:28]     let mut _36: (u32, u32);
[01:14:28]     let mut _37: (u32, u32);
[01:14:28]     let mut _38: (u32, u32);
[01:14:28]     let mut _39: (u32, u32);
[01:14:28]     let mut _40: (u32, u32);
[01:14:28]     let mut _41: (u32, u32);
[01:14:28]     let mut _42: (u32, u32);
[01:14:28]     let mut _43: (u32, u32);
[01:14:28]     let mut _44: (u32, u32);
[01:14:28]     let mut _45: (u32, u32);
[01:14:28]     let mut _46: (u32, u32);
[01:14:28]     let mut _47: (u32, u32);
[01:14:28]     let mut _48: (u32, u32);
[01:14:28]     bb0: {
[01:14:28]         StorageLive(_1);
[01:14:28]         StorageLive(_2);
[01:14:28]         StorageLive(_3);
[01:14:28]         StorageLive(_4);
[01:14:28]         StorageLive(_5);
[01:14:28]         StorageLive(_6);
[01:14:28]         StorageLive(_7);
[01:14:28]         _7 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_8);
[01:14:28]         _8 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_9);
[01:14:28]         _9 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_10);
[01:14:28]         _10 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_11);
[01:14:28]         _11 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_12);
[01:14:28]         _12 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_13);
[01:14:28]         _13 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_14);
[01:14:28]         _14 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_15);
[01:14:28]         _15 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_16);
[01:14:28]         _16 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_17);
[01:14:28]         _17 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_18);
[01:14:28]         _18 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_19);
[01:14:28]         _19 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_20);
[01:14:28]         _20 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_21);
[01:14:28]         _21 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_22);
[01:14:28]         _22 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_23);
[01:14:28]         _23 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_24);
[01:14:28]         _24 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_25);
[01:14:28]         _25 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_26);
[01:14:28]         _26 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_27);
[01:14:28]         _27 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_28);
[01:14:28]         _28 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_29);
[01:14:28]         _29 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_30);
[01:14:28]         _30 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_31);
[01:14:28]         _31 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_32);
[01:14:28]         _32 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_33);
[01:14:28]         _33 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_34);
[01:14:28]         _34 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_35);
[01:14:28]         _35 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_36);
[01:14:28]         _36 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_37);
[01:14:28]         _37 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_38);
[01:14:28]         _38 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_39);
[01:14:28]         _39 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_40);
[01:14:28]         _40 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_41);
[01:14:28]         _41 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_42);
[01:14:28]         _42 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_43);
[01:14:28]         _43 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_44);
[01:14:28]         _44 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_45);
[01:14:28]         _45 = (const 0u32, const 3u32);
[01:14:28]         StorageLive(_46);
[01:14:28]         _46 = (const 0u32, const 1u32);
[01:14:28]         StorageLive(_47);
[01:14:28]         _47 = (const 0u32, const 2u32);
[01:14:28]         StorageLive(_48);
[01:14:28]         _48 = (const 0u32, const 3u32);
[01:14:28]         _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
[01:14:28]         _5 = &_6;
[01:14:28]         _4 = &(*_5);
[01:14:28]         _3 = move _4 as &'static [(u32, u32)] (Pointer(Unsize));
[01:14:28]         _2 = Foo { tup: const "hi", data: move _3 };
[01:14:28]         _1 = &_2;
[01:14:28]         _0 = &(*_1);
[01:14:28]         StorageDead(_1);
[01:14:28]         StorageDead(_5);
[01:14:28]         return;
[01:14:28]     }
[01:14:28]     bb1 (cleanup): {
[01:14:28]         resume;
[01:14:28] }', src/tools/compiletest/src/runtest.rs:3100:13
[01:14:28] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:28] 
[01:14:28] 
[01:14:28] 
[01:14:28] failures:
[01:14:28]     [mir-opt] mir-opt/storage_live_dead_in_statics.rs
[01:14:28] 
[01:14:28] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:14:28] 
[01:14:28] 
[01:14:28] 
[01:14:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:28] 
[01:14:28] 
[01:14:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:28] Build completed unsuccessfully in 0:12:27
[01:14:28] Build completed unsuccessfully in 0:12:27
[01:14:28] Makefile:48: recipe for target 'check' failed
[01:14:28] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06ccc76b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 15 16:17:02 UTC 2019
