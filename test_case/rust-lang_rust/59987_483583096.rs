plain
travis_time:end:01ae947c:start=1555402306683683821,finish=1555402396381995890,duration=89698312069
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
[01:12:52] 
[01:12:52] running 42 tests
[01:13:10] ERROR 2019-04-16T09:26:35Z: compiletest::runtest: Some("   bb0: {")
[01:13:10] .......................................F..
[01:13:10] failures:
[01:13:10] 
[01:13:10] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:13:10] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:13:10] thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:13:10] Current block:    bb0: {
[01:13:10] Actual Line: "        _3 = move _4 as &\'static [(u32, u32)] (Pointer(Unsize));"
[01:13:10] Expected Line: "       _3 = move _4 as &\'static [(u32, u32)] (Unsize);"
[01:13:10] Test Name: rustc.XXX.mir_map.0.mir
[01:13:10] ... (elided)
[01:13:10]    let mut _0: &'static Foo;
[01:13:10]    let mut _1: &'static Foo;
[01:13:10]    let _2: Foo;
[01:13:10]    let _2: Foo;
[01:13:10]    let mut _3: &'static [(u32, u32)];
[01:13:10]    let mut _4: &'static [(u32, u32); 42];
[01:13:10]    let mut _5: &'static [(u32, u32); 42];
[01:13:10]    let _6: [(u32, u32); 42];
[01:13:10]    let mut _7: (u32, u32);
[01:13:10]    let mut _8: (u32, u32);
[01:13:10]    let mut _9: (u32, u32);
[01:13:10]    let mut _10: (u32, u32);
[01:13:10]    let mut _11: (u32, u32);
[01:13:10]    let mut _12: (u32, u32);
[01:13:10]    let mut _13: (u32, u32);
[01:13:10]    let mut _14: (u32, u32);
[01:13:10]    let mut _15: (u32, u32);
[01:13:10]    let mut _16: (u32, u32);
[01:13:10]    let mut _17: (u32, u32);
[01:13:10]    let mut _18: (u32, u32);
[01:13:10]    let mut _19: (u32, u32);
[01:13:10]    let mut _20: (u32, u32);
[01:13:10]    let mut _21: (u32, u32);
[01:13:10]    let mut _22: (u32, u32);
[01:13:10]    let mut _23: (u32, u32);
[01:13:10]    let mut _24: (u32, u32);
[01:13:10]    let mut _25: (u32, u32);
[01:13:10]    let mut _26: (u32, u32);
[01:13:10]    let mut _27: (u32, u32);
[01:13:10]    let mut _28: (u32, u32);
[01:13:10]    let mut _29: (u32, u32);
[01:13:10]    let mut _30: (u32, u32);
[01:13:10]    let mut _31: (u32, u32);
[01:13:10]    let mut _32: (u32, u32);
[01:13:10]    let mut _33: (u32, u32);
[01:13:10]    let mut _34: (u32, u32);
[01:13:10]    let mut _35: (u32, u32);
[01:13:10]    let mut _36: (u32, u32);
[01:13:10]    let mut _37: (u32, u32);
[01:13:10]    let mut _38: (u32, u32);
[01:13:10]    let mut _39: (u32, u32);
[01:13:10]    let mut _40: (u32, u32);
[01:13:10]    let mut _41: (u32, u32);
[01:13:10]    let mut _42: (u32, u32);
[01:13:10]    let mut _43: (u32, u32);
[01:13:10]    let mut _44: (u32, u32);
[01:13:10]    let mut _45: (u32, u32);
[01:13:10]    let mut _46: (u32, u32);
[01:13:10]    let mut _47: (u32, u32);
[01:13:10]    let mut _48: (u32, u32);
[01:13:10]    bb0: {
[01:13:10]        StorageLive(_1);
[01:13:10]        StorageLive(_2);
[01:13:10]        StorageLive(_3);
[01:13:10]        StorageLive(_4);
[01:13:10]        StorageLive(_5);
[01:13:10]        StorageLive(_6);
[01:13:10]        StorageLive(_7);
[01:13:10]        _7 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_8);
[01:13:10]        _8 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_9);
[01:13:10]        _9 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_10);
[01:13:10]        _10 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_11);
[01:13:10]        _11 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_12);
[01:13:10]        _12 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_13);
[01:13:10]        _13 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_14);
[01:13:10]        _14 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_15);
[01:13:10]        _15 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_16);
[01:13:10]        _16 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_17);
[01:13:10]        _17 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_18);
[01:13:10]        _18 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_19);
[01:13:10]        _19 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_20);
[01:13:10]        _20 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_21);
[01:13:10]        _21 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_22);
[01:13:10]        _22 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_23);
[01:13:10]        _23 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_24);
[01:13:10]        _24 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_25);
[01:13:10]        _25 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_26);
[01:13:10]        _26 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_27);
[01:13:10]        _27 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_28);
[01:13:10]        _28 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_29);
[01:13:10]        _29 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_30);
[01:13:10]        _30 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_31);
[01:13:10]        _31 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_32);
[01:13:10]        _32 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_33);
[01:13:10]        _33 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_34);
[01:13:10]        _34 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_35);
[01:13:10]        _35 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_36);
[01:13:10]        _36 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_37);
[01:13:10]        _37 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_38);
[01:13:10]        _38 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_39);
[01:13:10]        _39 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_40);
[01:13:10]        _40 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_41);
[01:13:10]        _41 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_42);
[01:13:10]        _42 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_43);
[01:13:10]        _43 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_44);
[01:13:10]        _44 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_45);
[01:13:10]        _45 = (const 0u32, const 3u32);
[01:13:10]        StorageLive(_46);
[01:13:10]        _46 = (const 0u32, const 1u32);
[01:13:10]        StorageLive(_47);
[01:13:10]        _47 = (const 0u32, const 2u32);
[01:13:10]        StorageLive(_48);
[01:13:10]        _48 = (const 0u32, const 3u32);
[01:13:10]        _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
[01:13:10]        _5 = &_6;
[01:13:10]        _4 = &(*_5);
[01:13:10]        _3 = move _4 as &'static [(u32, u32)] (Unsize);
[01:13:10]        _2 = Foo { tup: const "hi", data: move _3 };
[01:13:10]        _1 = &_2;
[01:13:10]        _0 = &(*_1);
[01:13:10]        StorageDead(_1);
[01:13:10]        StorageDead(_5);
[01:13:10]        return;
[01:13:10] Actual:
[01:13:10] Actual:
[01:13:10] static  XXX: &'static Foo = {
[01:13:10]     let mut _0: &'static Foo;
[01:13:10]     let mut _1: &'static Foo;
[01:13:10]     let _2: Foo;
[01:13:10]     let mut _3: &'static [(u32, u32)];
[01:13:10]     let mut _4: &'static [(u32, u32); 42];
[01:13:10]     let mut _5: &'static [(u32, u32); 42];
[01:13:10]     let _6: [(u32, u32); 42];
[01:13:10]     let mut _7: (u32, u32);
[01:13:10]     let mut _8: (u32, u32);
[01:13:10]     let mut _9: (u32, u32);
[01:13:10]     let mut _10: (u32, u32);
[01:13:10]     let mut _11: (u32, u32);
[01:13:10]     let mut _12: (u32, u32);
[01:13:10]     let mut _13: (u32, u32);
[01:13:10]     let mut _14: (u32, u32);
[01:13:10]     let mut _15: (u32, u32);
[01:13:10]     let mut _16: (u32, u32);
[01:13:10]     let mut _17: (u32, u32);
[01:13:10]     let mut _18: (u32, u32);
[01:13:10]     let mut _19: (u32, u32);
[01:13:10]     let mut _20: (u32, u32);
[01:13:10]     let mut _21: (u32, u32);
[01:13:10]     let mut _22: (u32, u32);
[01:13:10]     let mut _23: (u32, u32);
[01:13:10]     let mut _24: (u32, u32);
[01:13:10]     let mut _25: (u32, u32);
[01:13:10]     let mut _26: (u32, u32);
[01:13:10]     let mut _27: (u32, u32);
[01:13:10]     let mut _28: (u32, u32);
[01:13:10]     let mut _29: (u32, u32);
[01:13:10]     let mut _30: (u32, u32);
[01:13:10]     let mut _31: (u32, u32);
[01:13:10]     let mut _32: (u32, u32);
[01:13:10]     let mut _33: (u32, u32);
[01:13:10]     let mut _34: (u32, u32);
[01:13:10]     let mut _35: (u32, u32);
[01:13:10]     let mut _36: (u32, u32);
[01:13:10]     let mut _37: (u32, u32);
[01:13:10]     let mut _38: (u32, u32);
[01:13:10]     let mut _39: (u32, u32);
[01:13:10]     let mut _40: (u32, u32);
[01:13:10]     let mut _41: (u32, u32);
[01:13:10]     let mut _42: (u32, u32);
[01:13:10]     let mut _43: (u32, u32);
[01:13:10]     let mut _44: (u32, u32);
[01:13:10]     let mut _45: (u32, u32);
[01:13:10]     let mut _46: (u32, u32);
[01:13:10]     let mut _47: (u32, u32);
[01:13:10]     let mut _48: (u32, u32);
[01:13:10]     bb0: {
[01:13:10]         StorageLive(_1);
[01:13:10]         StorageLive(_2);
[01:13:10]         StorageLive(_3);
[01:13:10]         StorageLive(_4);
[01:13:10]         StorageLive(_5);
[01:13:10]         StorageLive(_6);
[01:13:10]         StorageLive(_7);
[01:13:10]         _7 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_8);
[01:13:10]         _8 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_9);
[01:13:10]         _9 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_10);
[01:13:10]         _10 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_11);
[01:13:10]         _11 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_12);
[01:13:10]         _12 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_13);
[01:13:10]         _13 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_14);
[01:13:10]         _14 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_15);
[01:13:10]         _15 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_16);
[01:13:10]         _16 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_17);
[01:13:10]         _17 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_18);
[01:13:10]         _18 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_19);
[01:13:10]         _19 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_20);
[01:13:10]         _20 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_21);
[01:13:10]         _21 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_22);
[01:13:10]         _22 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_23);
[01:13:10]         _23 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_24);
[01:13:10]         _24 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_25);
[01:13:10]         _25 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_26);
[01:13:10]         _26 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_27);
[01:13:10]         _27 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_28);
[01:13:10]         _28 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_29);
[01:13:10]         _29 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_30);
[01:13:10]         _30 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_31);
[01:13:10]         _31 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_32);
[01:13:10]         _32 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_33);
[01:13:10]         _33 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_34);
[01:13:10]         _34 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_35);
[01:13:10]         _35 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_36);
[01:13:10]         _36 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_37);
[01:13:10]         _37 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_38);
[01:13:10]         _38 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_39);
[01:13:10]         _39 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_40);
[01:13:10]         _40 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_41);
[01:13:10]         _41 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_42);
[01:13:10]         _42 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_43);
[01:13:10]         _43 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_44);
[01:13:10]         _44 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_45);
[01:13:10]         _45 = (const 0u32, const 3u32);
[01:13:10]         StorageLive(_46);
[01:13:10]         _46 = (const 0u32, const 1u32);
[01:13:10]         StorageLive(_47);
[01:13:10]         _47 = (const 0u32, const 2u32);
[01:13:10]         StorageLive(_48);
[01:13:10]         _48 = (const 0u32, const 3u32);
[01:13:10]         _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
[01:13:10]         _5 = &_6;
[01:13:10]         _4 = &(*_5);
[01:13:10]         _3 = move _4 as &'static [(u32, u32)] (Pointer(Unsize));
[01:13:10]         _2 = Foo { tup: const "hi", data: move _3 };
[01:13:10]         _1 = &_2;
[01:13:10]         _0 = &(*_1);
[01:13:10]         StorageDead(_1);
[01:13:10]         StorageDead(_5);
[01:13:10]         return;
[01:13:10]     }
[01:13:10]     bb1 (cleanup): {
[01:13:10]         resume;
[01:13:10] }', src/tools/compiletest/src/runtest.rs:3100:13
[01:13:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:10] 
[01:13:10] 
[01:13:10] 
[01:13:10] failures:
[01:13:10]     [mir-opt] mir-opt/storage_live_dead_in_statics.rs
[01:13:10] 
[01:13:10] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:13:10] 
[01:13:10] 
[01:13:10] 
[01:13:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:10] 
[01:13:10] 
[01:13:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:10] Build completed unsuccessfully in 0:11:09
[01:13:10] Build completed unsuccessfully in 0:11:09
[01:13:10] make: *** [check] Error 1
[01:13:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00837030
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 16 09:26:36 UTC 2019
---
travis_time:end:01e8d202:start=1555406798324812584,finish=1555406798329222534,duration=4409950
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0919e200
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25a343c5
travis_time:start:25a343c5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2878254f
$ dmesg | grep -i kill
