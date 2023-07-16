plain
[00:47:21] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:27] .......................i............................................................................
[00:47:29] ....................................................................................................
[00:47:32] ........................................................................iiiiiiiii...................
[00:47:37] ....................................................................................................
[00:47:41] ....................................................................................................
[00:47:44] .....................................................i..............................................
[00:47:46] ....................................................................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:08] 
[00:54:08] running 46 tests
[00:54:13] ERROR 2018-09-02T20:45:33Z: compiletest::runtest: None
[00:54:23] ERROR 2018-09-02T20:45:43Z: compiletest::runtest: None
[00:54:25] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:54:25] ....................F......................F..
[00:54:25] 
[00:54:25] ---- [mir-opt] mir-opt/end_region_destruction_extents_1.rs stdout ----
[00:54:25] ---- [mir-opt] mir-opt/end_region_destruction_extents_1.rs stdout ----
[00:54:25] thread '[mir-opt] mir-opt/end_region_destruction_extents_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:54:25] Current block: None
[00:54:25] Actual Line: "    let _5: S1;"
[00:54:25] Expected Line: "    let mut _5: S1;"
[00:54:25] Test Name: rustc.main.QualifyAndPromoteConstants.before.mir
[00:54:25] ... (elided)
[00:54:25] fn main() -> () {
[00:54:25] fn main() -> () {
[00:54:25] let mut _0: ();
[00:54:25]     let mut _1: &'12ds S1;
[00:54:25]     let mut _2: D1<'12ds, '10s>;
[00:54:25]     let mut _3: &'12ds S1;
[00:54:25]     let mut _4: &'12ds S1;
[00:54:25]     let mut _5: S1;
[00:54:25]     let mut _6: &'10s S1;
[00:54:25]     let mut _7: &'10s S1;
[00:54:25]     let mut _8: S1;
[00:54:25]     bb0: {
[00:54:25]         StorageLive(_2);
[00:54:25]         StorageLive(_3);
[00:54:25]         StorageLive(_4    StorageLive(_4);
[00:54:25]         StorageLive(_5);
[00:54:25]         _5 = S1::{{constructor}}(const "ex1",);
[00:54:25]         _4 = &'12ds _5;
[00:54:25]         _3 = &'12ds (*_4);
[00:54:25]         StorageLive(_6);
[00:54:25]         StorageLive(_7);
[00:54:25]         StorageLive(_8);
[00:54:25]         _8 = S1::{{constructor}}(const "dang1",);
[00:54:25]         _7 = &'10s _8;
[00:54:25]         _6 = &'10s (*_7);
[00:54:25]         _2 = D1<'12ds, '10s>::{{constructor}}(move _3, move _6);
[00:54:25]         EndRegion('10s);
[00:54:25]         StorageDead(_6);
[00:54:25]         StorageDead(_3);
[00:54:25]         _1 = (_2.0: &'12ds S1);
[00:54:25]         drop(_2) -> [return: bb2, unwind: bb1];
[00:54:25]     bb1: {
[00:54:25]         resume;
[00:54:25]     }
[00:54:25]     }
[00:54:25]     bb2: {                              
[00:54:25]         StorageDead(_2);
[00:54:25]         StorageDead(_7);
[00:54:25]         StorageDead(_8);
[00:54:25]         StorageDead(_4);
[00:54:25]         StorageDead(_5);
[00:54:25]         EndRegion('12ds);
[00:54:25]         _0 = ();
[00:54:25]         return;
[00:54:25] }', tools/compiletest/src/runtest.rs:2851:13
[00:54:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:25] 
[00:54:25] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[00:54:25] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[00:54:25] thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:54:25] Current block: None
[00:54:25] Actual Line: "    let _2: Foo;"
[00:54:25] Expected Line: "   let mut _2: Foo;"
[00:54:25] Test Name: rustc.XXX.mir_map.0.mir
[00:54:25] ... (elided)
[00:54:25] ... (elided)
[00:54:25]    let mut _0: &'static Foo;
[00:54:25]    let mut _1: &'static Foo;
[00:54:25]    let mut _2: Foo;
[00:54:25]    let mut _3: &'static [(u32, u32)];
[00:54:25]    let mut _4: &'static [(u32, u32); 42];
[00:54:25]    let mut _5: &'static [(u32, u32); 42];
[00:54:25]    let mut _6: [(u32, u32); 42];
[00:54:25]    let mut _7: (u32, u32);
[00:54:25]    let mut _8: (u32, u32);
[00:54:25]    let mut _9: (u32, u32);
[00:54:25]    let mut _10: (u32, u32);
[00:54:25]    let mut _11: (u32, u32);
[00:54:25]    let mut _12: (u32, u32);
[00:54:25]    let mut _13: (u32, u32);
[00:54:25]    let mut _14: (u32, u32);
[00:54:25]    let mut _15: (u32, u32);
[00:54:25]    let mut _16: (u32, u32);
[00:54:25]    let mut _17: (u32, u32);
[00:54:25]    let mut _18: (u32, u32);
[00:54:25]    let mut _19: (u32, u32);
[00:54:25]    let mut _20: (u32, u32);
[00:54:25]    let mut _21: (u32, u32);
[00:54:25]    let mut _22: (u32, u32);
[00:54:25]    let mut _23: (u32, u32);
[00:54:25]    let mut _24: (u32, u32);
[00:54:25]    let mut _25: (u32, u32);
[00:54:25]    let mut _26: (u32, u32);
[00:54:25]    let mut _27: (u32, u32);
[00:54:25]    let mut _28: (u32, u32);
[00:54:25]    let mut _29: (u32, u32);
[00:54:25]    let mut _30: (u32, u32);
[00:54:25]    let mut _31: (u32, u32);
[00:54:25]    let mut _32: (u32, u32);
[00:54:25]    let mut _33: (u32, u32);
[00:54:25]    let mut _34: (u32, u32);
[00:54:25]    let mut _35: (u32, u32);
[00:54:25]    let mut _36: (u32, u32);
[00:54:25]    let mut _37: (u32, u32);
[00:54:25]    ve(_18);
[00:54:25]        _18 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_19);
[00:54:25]        _19 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_20);
[00:54:25]        _20 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_21);
[00:54:25]        _21 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_22);
[00:54:25]        _22 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_23);
[00:54:25]        _23 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_24);
[00:54:25]        _24 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_25);
[00:54:25]        _25 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_26);
[00:54:25]        _26 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_27);
[00:54:25]        _27 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_28);
[00:54:25]        _28 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_29);
[00:54:25]        _29 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_30);
[00:54:25]        _30 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_31);
[00:54:25]        _31 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_32);
[00:54:25]        _32 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_33);
[00:54:25]        _33 = (const 0u32, const 3u32);
[00:54:25]        StorageLive(_34);
[00:54:25]        _34 = (const 0u32, const 1u32);
[00:54:25]        StorageLive(_35);
[00:54:25]        _35 = (const 0u32, const 2u32);
[00:54:25]        StorageLive(_36);
[00:54:25]        _36 = (const 0u32, const 3u32);
[00:54:2         _33 = (const 0u32, const 3u32);
[00:54:25]         StorageLive(_34);
[00:54:25]         _34 = (const 0u32, const 1u32);
[00:54:25]         StorageLive(_35);
[00:54:25]         _35 = (const 0u32, const 2u32);
[00:54:25]         StorageLive(_36);
[00:54:25]         _36 = (const 0u32, const 3u32);
[00:54:25]         StorageLive(_37);
[00:54:25]         _37 = (const 0u32, const 1u32);
[00:54:25]         StorageLive(_38);
[00:54:25]         _38 = (const 0u32, const 2u32);
[00:54:25]         StorageLive(_39);
[00:54:25]         _39 = (const 0u32, const 3u32);
[00:54:25]         StorageLive(_40);
[00:54:25]         _40 = (const 0u32, const 1u32);
[00:54:25]         StorageLive(_41);
[00:54:25]         _41 = (const 0u32, const 2u32);
[00:54:25]         StorageLive(_42);
[00:54:25]         _42 = (const 0u32, const 3u32);
[00:54:25]         StorageLive(_43);
[00:54:25]         _43 = (const 0u32, const 1u32);
[00:54:25]         StorageLive(_44);
[00:54:25]         _44 = (const 0u32, const 2u32);
[00:54:25]         StorageLive(_45);
[00:54:25]         _45 = (const 0u32, const 3u32);
[00:54:25]         StorageLive(_46);
[00:54:25]         _46 = (const 0u32, const 1u32);
[00:54:25]         StorageLive(_47);
[00:54:25]         _47 = (const 0u32, const 2u32);
[00:54:25]         StorageLive(_48);
[00:54:25]         _48 = (const 0u32, const 3u32);
[00:54:25]         _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30,Sun, 02 Sep 2018 20:45:46 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
