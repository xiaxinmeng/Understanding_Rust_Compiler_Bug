plain
travis_time:end:1db5af14:start=1556148656857222328,finish=1556148657639087918,duration=781865590
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:13] 
[01:17:13] running 42 tests
[01:17:14] ERROR 2019-04-25T00:48:23Z: compiletest::runtest: None
[01:17:14] ERROR 2019-04-25T00:48:23Z: compiletest::runtest: Some("    bb0: {")
[01:17:16] ERROR 2019-04-25T00:48:24Z: compiletest::runtest: None
[01:17:16] ERROR 2019-04-25T00:48:24Z: compiletest::runtest: None
[01:17:17] ERROR 2019-04-25T00:48:26Z: compiletest::runtest: None
[01:17:17] ERROR 2019-04-25T00:48:26Z: compiletest::runtest: None
[01:17:17] ERROR 2019-04-25T00:48:26Z: compiletest::runtest: None
[01:17:21] ERROR 2019-04-25T00:48:30Z: compiletest::runtest: Some(" bb8: { // binding1 and guard")
[01:17:23] ERROR 2019-04-25T00:48:32Z: compiletest::runtest: Some("    bb0: {")
[01:17:24] ERROR 2019-04-25T00:48:32Z: compiletest::runtest: Some("bb5: {")
[01:17:28] ERROR 2019-04-25T00:48:36Z: compiletest::runtest: None
[01:17:28] ERROR 2019-04-25T00:48:36Z: compiletest::runtest: None
[01:17:28] ERROR 2019-04-25T00:48:37Z: compiletest::runtest: Some("bb5 (cleanup): {")
[01:17:31] ERROR 2019-04-25T00:48:40Z: compiletest::runtest: Some("   bb0: {")
[01:17:32] .F.FFF..FFF..FFF.......F...FFF..F...FFFF.F
[01:17:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        _5 = &mut (*_6)_6;"
[01:17:32] Expected Line: "        _5 = &mut (*_6);"
[01:17:32] Test Name: rustc.main.EraseRegions.after.mir
[01:17:32] ... (elided)
[01:17:32]     bb0: {
[01:17:32] ... (elided)
[01:17:32]         _6 = &mut _2;
[01:17:32]         _6 = &mut _2;
[01:17:32]         _5 = &mut (*_6);
[01:17:32]         _4 = move _5 as *mut usize (Misc);
[01:17:32]         _3 = move _4;
[01:17:32] ... (elided)
[01:17:32]         _8 = _3;
[01:17:32]         _7 = const foo(move _8) -> bb1;
[01:17:32]     bb1: {
[01:17:32] ... (elided)
[01:17:32]         _9 = _2;
[01:17:32]         _10 = Len(_1);
[01:17:32]         _10 = Len(_1);
[01:17:32]         _11 = Lt(_9, _10);
[01:17:32]         assert(move _11, "index out of bounds: the len is move _10 but the index is _9") -> bb2;
[01:17:32]     bb2: {
[01:17:32]     bb2: {
[01:17:32]         _1[_9] = move _7;
[01:17:32] ... (elided)
[01:17:32]         return;
[01:17:32] Actual:
[01:17:32] | User Type Annotations
[01:17:32] | User Type Annotations
[01:17:32] | 0: Canonical { max_universe: U0, variables: [], value: Ty(*mut usize) } at /checkout/src/test/mir-opt/array-index-is-temporary.rs:13:12: 13:22
[01:17:32] | 1: Canonical { max_universe: U0, variables: [], value: Ty(*mut usize) } at /checkout/src/test/mir-opt/array-index-is-temporary.rs:13:12: 13:22
[01:17:32] |
[01:17:32] fn  main() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _4: *mut usize;
[01:17:32]     let mut _5: &mut usize;
[01:17:32]     let mut _6: &mut usize;
[01:17:32]     let mut _7: u32;
[01:17:32]     let mut _8: *mut usize;
[01:17:32]     let mut _9: usize;
[01:17:32]     let mut _10: usize;
[01:17:32]     let mut _11: bool;
[01:17:32]     scope 1 {
[01:17:32]         let mut _1: [u32; 3];
[01:17:32]     scope 2 {
[01:17:32]         scope 3 {
[01:17:32]             let mut _2: usize;
[01:17:32]         }
[01:17:32]         }
[01:17:32]         scope 4 {
[01:17:32]             scope 5 {
[01:17:32]                 let _3: *mut usize as UserTypeProjection { base: UserType(0), projs: [] };
[01:17:32]             scope 6 {
[01:17:32]                 scope 7 {
[01:17:32]                 }
[01:17:32]             }
[01:17:32]             }
[01:17:32]         }
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         _1 = [const 42u32, const 43u32, const 44u32];
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = const 1usize;
[01:17:32]         StorageLive(_3);
[01:17:32]         StorageLive(_4);
[01:17:32]         StorageLive(_5);
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = &mut _2;
[01:17:32]         _5 = &mut (*_6)_6;
[01:17:32]         _4 = move _5 as *mut usize (Misc);
[01:17:32]         _3 = move _4;
[01:17:32]         StorageDead(_4);
[01:17:32]         StorageDead(_5);
[01:17:32]         StorageDead(_6);
[01:17:32]         StorageLive(_7);
[01:17:32]         StorageLive(_8);
[01:17:32]         _8 = _3;
[01:17:32]         _7 = const foo(move _8) -> bb1;
[01:17:32]     bb1: {
[01:17:32]     bb1: {
[01:17:32]         StorageDead(_8);
[01:17:32]         StorageLive(_9);
[01:17:32]         _9 = _2;
[01:17:32]         _10 = Len(_1);
[01:17:32]         _11 = Lt(_9, _10);
[01:17:32]         assert(move _11, "index out of bounds: the len is move _10 but the index is _9") -> bb2;
[01:17:32]     bb2: {
[01:17:32]     bb2: {
[01:17:32]         _1[_9]_1 = move _7;
[01:17:32]         StorageDead(_7);
[01:17:32]         _0 = ();
[01:17:32]         StorageDead(_3);
[01:17:32]         StorageDead(_2);
[01:17:32]         StorageDead(_1);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block:     bb0: {
[01:17:32] Actual Line: "        (*_2)_2 = const S::new() -> [return: bb2, unwind: bb3];"
[01:17:32] Expected Line: "        (*_2) = const S::new() -> [return: bb2, unwind: bb3];"
[01:17:32] Test Name: rustc.main.ElaborateDrops.before.mir
[01:17:32] ... (elided)
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: std::boxed::Box<S>;
[01:17:32]     let mut _3: ();
[01:17:32]     let mut _3: ();
[01:17:32]     let mut _4: std::boxed::Box<S>;
[01:17:32]     scope 1 {
[01:17:32]         let _1: std::boxed::Box<S>;
[01:17:32]     }
[01:17:32]     scope 2 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = Box(S);
[01:17:32]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32]     bb2: {
[01:17:32]         _1 = move _2;
[01:17:32]         drop(_2) -> bb4;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb3 (cleanup): {
[01:17:32]         drop(_2) -> bb1;
[01:17:32]     bb4: {
[01:17:32]     bb4: {
[01:17:32]         StorageDead(_2);
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = move _1;
[01:17:32]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:17:32]     bb5: {
[01:17:32]     bb5: {
[01:17:32]         drop(_4) -> [return: bb8, unwind: bb6];
[01:17:32]     }
[01:17:32]     bb6 (cleanup): {
[01:17:32]         drop(_1) -> bb1;
[01:17:32]     }
[01:17:32]     bb7 (cleanup): {
[01:17:32]         drop(_4) -> bb6;
[01:17:32]     bb8: {
[01:17:32]     bb8: {
[01:17:32]         StorageDead(_4);
[01:17:32]         _0 = ();
[01:17:32]         drop(_1) -> bb9;
[01:17:32]     bb9: {
[01:17:32]     bb9: {
[01:17:32]         StorageDead(_1);
[01:17:32]         return;
[01:17:32] }
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  main() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: std::boxed::Box<S>;
[01:17:32]     let mut _3: ();
[01:17:32]     let mut _4: std::boxed::Box<S>;
[01:17:32]     scope 1 {
[01:17:32]         let _1: std::boxed::Box<S>;
[01:17:32]     scope 2 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = Box(S);
[01:17:32]         (*_2)_2 = const S::new() -> [return: bb2, unwind: bb3];
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32]     bb2: {
[01:17:32]         _1 = move _2;
[01:17:32]         drop(_2) -> bb4;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb3 (cleanup): {
[01:17:32]         drop(_2) -> bb1;
[01:17:32]     bb4: {
[01:17:32]     bb4: {
[01:17:32]         StorageDead(_2);
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = move _1;
[01:17:32]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:17:32]     bb5: {
[01:17:32]     bb5: {
[01:17:32]         drop(_4) -> [return: bb8, unwind: bb6];
[01:17:32]     }
[01:17:32]     bb6 (cleanup): {
[01:17:32]         drop(_1) -> bb1;
[01:17:32]     }
[01:17:32]     bb7 (cleanup): {
[01:17:32]         drop(_4) -> bb6;
[01:17:32]     bb8: {
[01:17:32]     bb8: {
[01:17:32]         StorageDead(_4);
[01:17:32]         _0 = ();
[01:17:32]         drop(_1) -> bb9;
[01:17:32]     bb9: {
[01:17:32]     bb9: {
[01:17:32]         StorageDead(_1);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/deaggregator_test.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:32] Expected Line: "    (_0.0: usize) = move _2;"
[01:17:32] Test Name: rustc.bar.Deaggregator.after.mir
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] ... (elided)
[01:17:32]     _2 = _1;
[01:17:32]     _2 = _1;
[01:17:32] ... (elided)
[01:17:32]     (_0.0: usize) = move _2;
[01:17:32]     (_0.1: f32) = const 0f32;
[01:17:32]     (_0.2: bool) = const false;
[01:17:32] ... (elided)
[01:17:32]     return;
[01:17:32] }
[01:17:32] Actual:
[01:17:32] fn  bar(_1: usize) -> Baz {
[01:17:32]     let mut _0: Baz;
[01:17:32]     let mut _2: usize;
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = _1;
[01:17:32]         (_0.0: usize)_0 = move _2;
[01:17:32]         (_0.1: f32)_0 = const 0f32;
[01:17:32]         (_0.2: bool)_0 = const false;
[01:17:32]         StorageDead(_2);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/deaggregator_test_enum_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        (_0 as A)((_0 as A)_0.0: i32)_0 = move _4;"
[01:17:32] Expected Line: "     ((_0 as A).0: i32) = move _4;"
[01:17:32] Test Name: rustc.test1.Deaggregator.after.mir
[01:17:32] ... (elided)
[01:17:32]  bb1: {
[01:17:32]  bb1: {
[01:17:32]      StorageLive(_4);
[01:17:32]      _4 = _2;
[01:17:32]      ((_0 as A).0: i32) = move _4;
[01:17:32]      discriminant(_0) = 0;
[01:17:32]      StorageDead(_4);
[01:17:32]      goto -> bb3;
[01:17:32]  bb2: {
[01:17:32]  bb2: {
[01:17:32]      StorageLive(_5);
[01:17:32]      _5 = _2;
[01:17:32]      ((_0 as B).0: i32) = move _5;
[01:17:32]      discriminant(_0) = 1;
[01:17:32]      StorageDead(_5);
[01:17:32]      goto -> bb3;
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  test1(_1: bool, _2: i32) -> Foo {
[01:17:32]     let mut _0: Foo;
[01:17:32]     let mut _3: bool;
[01:17:32]     let mut _4: i32;
[01:17:32]     let mut _5: i32;
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = _1;
[01:17:32]         switchInt(move _3) -> [false: bb2, otherwise: bb1];
[01:17:32]     bb1: {
[01:17:32]     bb1: {
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = _2;
[01:17:32]         (_0 as A)((_0 as A)_0.0: i32)_0 = move _4;
[01:17:32]         discriminant(_0) = 0;
[01:17:32]         StorageDead(_4);
[01:17:32]         goto -> bb3;
[01:17:32]     bb2: {
[01:17:32]     bb2: {
[01:17:32]         StorageLive(_5);
[01:17:32]         _5 = _2;
[01:17:32]         (_0 as B)((_0 as B)_0.0: i32)_0 = move _5;
[01:17:32]         discriminant(_0) = 1;
[01:17:32]         StorageDead(_5);
[01:17:32]         goto -> bb3;
[01:17:32]     bb3: {
[01:17:32]     bb3: {
[01:17:32]         StorageDead(_3);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb4 (cleanup): {
[01:17:32]         resume;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/deaggregator_test_multiple.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:32] Expected Line: "    ((_2 as A).0: i32) = move _3;"
[01:17:32] Test Name: rustc.test.Deaggregator.after.mir
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] ... (elided)
[01:17:32]     _3 = _1;
[01:17:32]     _3 = _1;
[01:17:32] ... (elided)
[01:17:32]     ((_2 as A).0: i32) = move _3;
[01:17:32]     discriminant(_2) = 0;
[01:17:32] ... (elided)
[01:17:32]     _5 = _1;
[01:17:32]     ((_4 as A).0: i32) = move _5;
[01:17:32]     discriminant(_4) = 0;
[01:17:32] ... (elided)
[01:17:32]     _0 = [move _2, move _4];
[01:17:32] ... (elided)
[01:17:32]     return;
[01:17:32] }
[01:17:32] Actual:
[01:17:32] fn  test(_1: i32) -> [Foo; 2] {
[01:17:32]     let mut _0: [Foo; 2];
[01:17:32]     let mut _2: Foo;
[01:17:32]     let mut _3: i32;
[01:17:32]     let mut _4: Foo;
[01:17:32]     let mut _5: i32;
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_2);
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = _1;
[01:17:32]         (_2 as A)((_2 as A)_2.0: i32)_2 = move _3;
[01:17:32]         discriminant(_2) = 0;
[01:17:32]         StorageDead(_3);
[01:17:32]         StorageLive(_4);
[01:17:32]         StorageLive(_5);
[01:17:32]         _5 = _1;
[01:17:32]         (_4 as A)((_4 as A)_4.0: i32)_4 = move _5;
[01:17:32]         discriminant(_4) = 0;
[01:17:32]         StorageDead(_5);
[01:17:32]         _0 = [move _2, move _4];
[01:17:32]         StorageDead(_4);
[01:17:32]         StorageDead(_2);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/generator-drop-cleanup.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        switchInt((*_1)((*_1)_1.0: u32)_1) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];"
[01:17:32] Expected Line: "    switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];"
[01:17:32] Test Name: rustc.main-{{closure}}.generator_drop.0.mir
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] bb0: {
[01:17:32]     switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:17:32] }
[01:17:32] bb1: {
[01:17:32]     goto -> bb5;
[01:17:32] }
[01:17:32] bb2: {
[01:17:32]     return;
[01:17:32] }
[01:17:32] bb3: {
[01:17:32]     return;
[01:17:32] }
[01:17:32] bb4: {
[01:17:32]     goto -> bb6;
[01:17:32] }
[01:17:32] bb5: {
[01:17:32]     goto -> bb2;
[01:17:32] }
[01:17:32] bb6: {
[01:17:32]     goto -> bb3;
[01:17:32] }
[01:17:32] bb7: {
[01:17:32]     StorageLive(_3);
[01:17:32]     goto -> bb1;
[01:17:32] }
[01:17:32] bb8: {
[01:17:32]     return;
[01:17:32] }
[01:17:32] Actual:
[01:17:32] fn  main::{{closure}}#0(_1: *mut [generator@/checkout/src/test/mir-opt/generator-drop-cleanup.rs:7:15: 9:6 {()}]) -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: ();
[01:17:32]     let mut _3: ();
[01:17:32]     let mut _4: ();
[01:17:32]     bb0: {
[01:17:32]         switchInt((*_1)((*_1)_1.0: u32)_1) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:17:32]     bb1: {
[01:17:32]         goto -> bb5;
[01:17:32]     }
[01:17:32]     bb2: {
---
[01:17:32]     }
[01:17:32]     bb6: {
[01:17:32]         goto -> bb3;
[01:17:32]     }
[01:17:32]     bb7: {
[01:17:32]         StorageLive(_3);
[01:17:32]         goto -> bb1;
[01:17:32]     bb8: {
[01:17:32]         return;
[01:17:32]     }
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/deaggregator_test_enum.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/deaggregator_test_enum.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        (_0 as Foo)((_0 as Foo)_0.0: usize)_0 = move _2;"
[01:17:32] Expected Line: "    ((_0 as Foo).0: usize) = move _2;"
[01:17:32] Test Name: rustc.bar.Deaggregator.after.mir
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] bb0: {
[01:17:32]     StorageLive(_2);
[01:17:32]     _2 = _1;
[01:17:32]     ((_0 as Foo).0: usize) = move _2;
[01:17:32]     discriminant(_0) = 1;
[01:17:32]     StorageDead(_2);
[01:17:32]     return;
[01:17:32] }
[01:17:32] Actual:
[01:17:32] fn  bar(_1: usize) -> Baz {
[01:17:32]     let mut _0: Baz;
[01:17:32]     let mut _2: usize;
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = _1;
[01:17:32]         (_0 as Foo)((_0 as Foo)_0.0: usize)_0 = move _2;
[01:17:32]         discriminant(_0) = 1;
[01:17:32]         StorageDead(_2);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        _8 = move (_5.0: i32)_5;"
[01:17:32] Expected Line: "    _8 = move (_5.0: i32);"
[01:17:32] Test Name: rustc.foo.Inline.after.mir
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:17:32] ... (elided)
[01:17:32]     _4 = &_3;
[01:17:32] ... (elided)
[01:17:32]     _6 = _2;
[01:17:32] ... (elided)
[01:17:32]     _7 = _2;
[01:17:32]     _5 = (move _6, move _7);
[01:17:32]     _8 = move (_5.0: i32);
[01:17:32]     _9 = move (_5.1: i32);
[01:17:32]     _0 = _8;
[01:17:32] ... (elided)
[01:17:32]     return;
[01:17:32] }
[01:17:32] ... (elided)
[01:17:32] Actual:
[01:17:32] fn  foo(_1: T, _2: i32) -> i32 {
[01:17:32]     let mut _0: i32;
[01:17:32]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:17:32]     let mut _5: (i32, i32);
[01:17:32]     let mut _6: i32;
[01:17:32]     let mut _7: i32;
[01:17:32]     let mut _8: i32;
[01:17:32]     let mut _9: i32;
[01:17:32]     scope 1 {
[01:17:32]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:17:32]     scope 2 {
[01:17:32]         scope 3 {
[01:17:32]         }
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 15 }];
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = &_3;
[01:17:32]         StorageLive(_5);
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = _2;
[01:17:32]         StorageLive(_7);
[01:17:32]         _7 = _2;
[01:17:32]         _5 = (move _6, move _7);
[01:17:32]         _8 = move (_5.0: i32)_5;
[01:17:32]         _9 = move (_5.1: i32)_5;
[01:17:32]         _0 = _8;
[01:17:32]         StorageDead(_5);
[01:17:32]         StorageDead(_7);
[01:17:32]         StorageDead(_6);
[01:17:32]         StorageDead(_4);
[01:17:32]         StorageDead(_3);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:32] Expected Line: "    _6 = &(*_2);"
[01:17:32] Test Name: rustc.foo.Inline.after.mir
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]     _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:17:32] ... (elided)
[01:17:32]     _4 = &_3;
[01:17:32] ... (elided)
[01:17:32]     _6 = &(*_2);
[01:17:32] ... (elided)
[01:17:32]     _7 = &(*_2);
[01:17:32]     _5 = (move _6, move _7);
[01:17:32]     _8 = move (_5.0: &i32);
[01:17:32]     _9 = move (_5.1: &i32);
[01:17:32] ... (elided)
[01:17:32]     _0 = (*_8);
[01:17:32] ... (elided)
[01:17:32]     return;
[01:17:32] }
[01:17:32] ... (elided)
[01:17:32] Actual:
[01:17:32] fn  foo(_1: T, _2: &i32) -> i32 {
[01:17:32]     let mut _0: i32;
[01:17:32]     let mut _4: &[closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:17:32]     let mut _5: (&i32, &i32);
[01:17:32]     let mut _6: &i32;
[01:17:32]     let mut _7: &i32;
[01:17:32]     let mut _8: &i32;
[01:17:32]     let mut _9: &i32;
[01:17:32]     scope 1 {
[01:17:32]         let _3: [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:17:32]     scope 2 {
[01:17:32]         scope 3 {
[01:17:32]         }
[01:17:32]     }
[01:17:32]     }
[01:17:32]     scope 4 {
[01:17:32]     }
[01:17:32]     scope 5 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = [closure@HirId { owner: DefIndex(0:4), local_id: 31 }];
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = &_3;
[01:17:32]         StorageLive(_5);
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = &(*_2)_2;
[01:17:32]         StorageLive(_7);
[01:17:32]         _7 = &(*_2)_2;
[01:17:32]         _5 = (move _6, move _7);
[01:17:32]         _8 = move (_5.0: &i32)_5;
[01:17:32]         _9 = move (_5.1: &i32)_5;
[01:17:32]         _0 = (*_8)_8;
[01:17:32]         StorageDead(_5);
[01:17:32]         StorageDead(_7);
[01:17:32]         StorageDead(_6);
[01:17:32]         StorageDead(_4);
[01:17:32]         StorageDead(_3);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/inline-retag.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/inline-retag.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/inline-retag.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        _9 = (*_3)_3;"
[01:17:32] Expected Line: "        _9 = (*_3);"
[01:17:32] Test Name: rustc.bar.Inline.after.mir
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]     bb0: {
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]         Retag(_3);
[01:17:32] ... (elided)
[01:17:32]         Retag(_3);
[01:17:32]         Retag(_6);
[01:17:32]         StorageLive(_9);
[01:17:32]         _9 = (*_3);
[01:17:32]         StorageLive(_10);
[01:17:32]         _10 = (*_6);
[01:17:32]         _0 = Eq(move _9, move _10);
[01:17:32] ... (elided)
[01:17:32]         return;
[01:17:32] ... (elided)
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  bar() -> bool {
[01:17:32]     let mut _0: bool;
[01:17:32]     let mut _2: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:17:32]     let mut _3: &i32;
[01:17:32]     let mut _4: &i32;
[01:17:32]     let _5: i32;
[01:17:32]     let mut _6: &i32;
[01:17:32]     let mut _7: &i32;
[01:17:32]     let _8: i32;
[01:17:32]     scope 1 {
[01:17:32]         let _1: for<'r, 's> fn(&'r i32, &'s i32) -> bool {foo};
[01:17:32]     scope 2 {
[01:17:32]         scope 3 {
[01:17:32]             let mut _9: i32;
[01:17:32]             let mut _10: i32;
[01:17:32]             let mut _10: i32;
[01:17:32]         }
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         _1 = const foo;
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = _1;
[01:17:32]         StorageLive(_3);
[01:17:32]         StorageLive(_4);
[01:17:32]         _4 = &(promoted[1]: i32);
[01:17:32]         Retag(_4);
[01:17:32]         _3 = &(*_4)_4;
[01:17:32]         Retag(_3);
[01:17:32]         StorageLive(_6);
[01:17:32]         StorageLive(_7);
[01:17:32]         _7 = &(promoted[0]: i32);
[01:17:32]         Retag(_7);
[01:17:32]         _6 = &(*_7)_7;
[01:17:32]         Retag(_6);
[01:17:32]         Retag(_3);
[01:17:32]         Retag(_6);
[01:17:32]         StorageLive(_9);
[01:17:32]         _9 = (*_3)_3;
[01:17:32]         StorageLive(_10);
[01:17:32]         _10 = (*_6)_6;
[01:17:32]         _0 = Eq(move _9, move _10);
[01:17:32]         StorageDead(_10);
[01:17:32]         StorageDead(_9);
[01:17:32]         StorageDead(_6);
[01:17:32]         StorageDead(_3);
[01:17:32]         StorageDead(_2);
[01:17:32]         StorageDead(_1);
[01:17:32]         StorageDead(_7);
[01:17:32]         StorageDead(_4);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block:  bb8: { // binding1 and guard
[01:17:32] Actual Line: "        _6 = &((promoted[0]: std::option::Option<i32>) as Some)(((promoted[0]: std::option::Option<i32>) as Some)(promoted[0]: std::option::Option<i32>).0: i32)(promoted[0]: std::option::Option<i32>);"
[01:17:32] Expected Line: "     _6 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);"
[01:17:32] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[01:17:32] ... (elided)
[01:17:32]  bb0: {
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]      _2 = std::option::Option::<i32>::Some(const 42i32,);
[01:17:32]      FakeRead(ForMatchedPlace, _2);
[01:17:32]      _3 = discriminant(_2);
[01:17:32]      switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:17:32]  }
[01:17:32]  bb1 (cleanup): {
[01:17:32]      resume;
[01:17:32]  bb2: {
[01:17:32]  bb2: {
[01:17:32]      falseEdges -> [real: bb8, imaginary: bb3]; //pre_binding1
[01:17:32]  bb3: {
[01:17:32]  bb3: {
[01:17:32]      falseEdges -> [real: bb11, imaginary: bb4]; //pre_binding2
[01:17:32]  bb4: {
[01:17:32]  bb4: {
[01:17:32]      falseEdges -> [real: bb12, imaginary: bb5]; //pre_binding3
[01:17:32]  bb5: {
[01:17:32]      unreachable;
[01:17:32]  }
[01:17:32]  }
[01:17:32]  bb6: { // to pre_binding2
[01:17:32]      falseEdges -> [real: bb3, imaginary: bb3];
[01:17:32]  bb7: {
[01:17:32]      unreachable;
[01:17:32]  }
[01:17:32]  }
[01:17:32]  bb8: { // binding1 and guard
[01:17:32]      StorageLive(_6);
[01:17:32]      _6 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[01:17:32]      _4 = &shallow _2;
[01:17:32]      StorageLive(_7);
[01:17:32]      _7 = const guard() -> [return: bb9, unwind: bb1];
[01:17:32]  bb9: {
[01:17:32]  bb9: {
[01:17:32]      FakeRead(ForMatchGuard, _4);
[01:17:32]      FakeRead(ForGuardBinding, _6);
[01:17:32]      switchInt(move _7) -> [false: bb6, otherwise: bb10];
[01:17:32]  bb10: {
[01:17:32]  bb10: {
[01:17:32]      StorageLive(_5);
[01:17:32]      _5 = ((_2 as Some).0: i32);
[01:17:32]      StorageLive(_8);
[01:17:32]      _8 = _5;
[01:17:32]      _1 = (const 1i32, move _8);
[01:17:32]      StorageDead(_8);
[01:17:32]      goto -> bb13;
[01:17:32]  bb11: {
[01:17:32]  bb11: {
[01:17:32]      StorageLive(_9);
[01:17:32]      _9 = ((_2 as Some).0: i32);
[01:17:32]      StorageLive(_10);
[01:17:32]      _10 = _9;
[01:17:32]      _1 = (const 2i32, move _10);
[01:17:32]      StorageDead(_10);
[01:17:32]      goto -> bb13;
[01:17:32]  bb12: {
[01:17:32]  bb12: {
[01:17:32]      _1 = (const 3i32, const 3i32);
[01:17:32]      goto -> bb13;
[01:17:32]  bb13: {
[01:17:32] ... (elided)
[01:17:32]      return;
[01:17:32]  }
[01:17:32]  }
[01:17:32] Actual:
[01:17:32] fn  full_tested_match() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _1: (i32, i32);
[01:17:32]     let mut _2: std::option::Option<i32>;
[01:17:32]     let mut _3: isize;
[01:17:32]     let mut _4: &std::option::Option<i32>;
[01:17:32]     let _5: i32;
[01:17:32]     let _6: &i32;
[01:17:32]     let mut _7: bool;
[01:17:32]     let mut _8: i32;
[01:17:32]     let mut _10: i32;
[01:17:32]     scope 1 {
[01:17:32]         let _9: i32;
[01:17:32]         scope 2 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = std::option::Option::<i32>::Some(const 42i32,);
[01:17:32]         FakeRead(ForMatchedPlace, _2);
[01:17:32]         _3 = discriminant(_2);
[01:17:32]         switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32]     bb2: {
[01:17:32]     bb2: {
[01:17:32]         falseEdges -> [real: bb8, imaginary: bb3];
[01:17:32]     bb3: {
[01:17:32]     bb3: {
[01:17:32]         falseEdges -> [real: bb11, imaginary: bb4];
[01:17:32]     bb4: {
[01:17:32]     bb4: {
[01:17:32]         falseEdges -> [real: bb12, imaginary: bb5];
[01:17:32]     bb5: {
[01:17:32]         unreachable;
[01:17:32]     }
[01:17:32]     bb6: {
[01:17:32]     bb6: {
[01:17:32]         falseEdges -> [real: bb3, imaginary: bb3];
[01:17:32]     bb7: {
[01:17:32]         unreachable;
[01:17:32]     }
[01:17:32]     bb8: {
[01:17:32]     bb8: {
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = &((promoted[0]: std::option::Option<i32>) as Some)(((promoted[0]: std::option::Option<i32>) as Some)(promoted[0]: std::option::Option<i32>).0: i32)(promoted[0]: std::option::Option<i32>);
[01:17:32]         _4 = &shallow _2;
[01:17:32]         StorageLive(_7);
[01:17:32]         _7 = const guard() -> [return: bb9, unwind: bb1];
[01:17:32]     bb9: {
[01:17:32]     bb9: {
[01:17:32]         FakeRead(ForMatchGuard, _4);
[01:17:32]         FakeRead(ForGuardBinding, _6);
[01:17:32]         switchInt(move _7) -> [false: bb6, otherwise: bb10];
[01:17:32]     bb10: {
[01:17:32]     bb10: {
[01:17:32]         StorageLive(_5);
[01:17:32]         _5 = (_2 as Some)((_2 as Some)_2.0: i32)_2;
[01:17:32]         StorageLive(_8);
[01:17:32]         _8 = _5;
[01:17:32]         _1 = (const 1i32, move _8);
[01:17:32]         StorageDead(_8);
[01:17:32]         goto -> bb13;
[01:17:32]     bb11: {
[01:17:32]     bb11: {
[01:17:32]         StorageLive(_9);
[01:17:32]         _9 = (_2 as Some)((_2 as Some)_2.0: i32)_2;
[01:17:32]         StorageLive(_10);
[01:17:32]         _10 = _9;
[01:17:32]         _1 = (const 2i32, move _10);
[01:17:32]         StorageDead(_10);
[01:17:32]         goto -> bb13;
[01:17:32]     bb12: {
[01:17:32]     bb12: {
[01:17:32]         _1 = (const 3i32, const 3i32);
[01:17:32]         goto -> bb13;
[01:17:32]     bb13: {
[01:17:32]     bb13: {
[01:17:32]         StorageDead(_9);
[01:17:32]         StorageDead(_5);
[01:17:32]         StorageDead(_7);
[01:17:32]         StorageDead(_6);
[01:17:32]         StorageDead(_1);
[01:17:32]         StorageDead(_2);
[01:17:32]         _0 = ();
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block:     bb0: {
[01:17:32] Actual Line: "        _6 = move (_1.0: Aligned)_1;"
[01:17:32] Expected Line: "        _6 = move (_1.0: Aligned);"
[01:17:32] Test Name: rustc.main.EraseRegions.before.mir
[01:17:32] ... (elided)
[01:17:32] fn main() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: Aligned;
[01:17:32]     let mut _2: Aligned;
[01:17:32]     let mut _3: Droppy;
[01:17:32]     let mut _4: Aligned;
[01:17:32]     let mut _5: Droppy;
[01:17:32]     let mut _6: Aligned;
[01:17:32]     scope 1 {
[01:17:32]         let mut _1: Packed;
[01:17:32]     scope 2 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32] ... (elided)
[01:17:32]         _1 = Packed(move _2,);
[01:17:32] ... (elided)
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = move (_1.0: Aligned);
[01:17:32]         drop(_6) -> [return: bb4, unwind: bb3];
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32]     bb2: {
[01:17:32]     bb2: {
[01:17:32]         StorageDead(_1);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb3 (cleanup): {
[01:17:32]         (_1.0: Aligned) = move _4;
[01:17:32]         drop(_1) -> bb1;
[01:17:32]     bb4: {
[01:17:32]     bb4: {
[01:17:32]         StorageDead(_6);
[01:17:32]         (_1.0: Aligned) = move _4;
[01:17:32]         StorageDead(_4);
[01:17:32]         _0 = ();
[01:17:32]         drop(_1) -> [return: bb2, unwind: bb1];
[01:17:32] }
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  main() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: Aligned;
[01:17:32]     let mut _3: Droppy;
[01:17:32]     let mut _4: Aligned;
[01:17:32]     let mut _5: Droppy;
[01:17:32]     let mut _6: Aligned;
[01:17:32]     scope 1 {
[01:17:32]         let mut _1: Packed;
[01:17:32]     scope 2 {
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = Droppy(const 0usize,);
[01:17:32]         _2 = Aligned(move _3,);
[01:17:32]         StorageDead(_3);
[01:17:32]         _1 = Packed(move _2,);
[01:17:32]         StorageDead(_2);
[01:17:32]         StorageLive(_4);
[01:17:32]         StorageLive(_5);
[01:17:32]         _5 = Droppy(const 0usize,);
[01:17:32]         _4 = Aligned(move _5,);
[01:17:32]         StorageDead(_5);
[01:17:32]         StorageLive(_6);
---
[01:17:32]     }
[01:17:32]     bb4: {
[01:17:32]         goto -> bb2;
[01:17:32]     }
[01:17:32]     bb5: {
[01:17:32]         switchInt((_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1)_1)_1) -> [0i32: bb1, otherwise: bb2];
[01:17:32]     bb6: {
[01:17:32]         _0 = const 0i32;
[01:17:32]         goto -> bb9;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb7: {
[01:17:32]         _4 = &shallow _1;
[01:17:32]         _5 = &shallow (_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1;
[01:17:32]         _6 = &shallow (_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1)_1;
[01:17:32]         _7 = &shallow (_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)(*(_1 as Some)((_1 as Some)_1.0: &'<empty> &'<empty> i32)_1)_1)_1;
[01:17:32]         StorageLive(_8);
[01:17:32]         _8 = _2;
[01:17:32]         FakeRead(ForMatchGuard, _4);
[01:17:32]         FakeRead(ForMatchGuard, _5);
[01:17:32]         FakeRead(ForMatchGuard, _6);
[01:17:32]         FakeRead(ForMatchGuard, _7);
[01:17:32]         switchInt(move _8) -> [false: bb4, otherwise: bb6];
[01:17:32]     bb8: {
[01:17:32]         _0 = const 1i32;
[01:17:32]         goto -> bb9;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb9: {
[01:17:32]         StorageDead(_8);
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     bb10 (cleanup): {
[01:17:32]         resume;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/retag.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:17:32] Expected Line: "        _0 = &mut (*_3);"
[01:17:32] Test Name: rustc.{{impl}}-foo.EraseRegions.after.mir
[01:17:32] ... (elided)
[01:17:32]     bb0: {
[01:17:32]     bb0: {
[01:17:32]         Retag([fn entry] _1);
[01:17:32]         Retag([fn entry] _2);
[01:17:32] ... (elided)
[01:17:32]         _0 = &mut (*_3);
[01:17:32]         Retag(_0);
[01:17:32] ... (elided)
[01:17:32]         return;
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  <impl at /checkout/src/test/mir-opt/retag.rs:8:1: 12:2>::foo(_1: &Test, _2: &mut i32) -> &mut i32 {
[01:17:32]     let mut _0: &mut i32;
[01:17:32]     let mut _3: &mut i32;
[01:17:32]     bb0: {
[01:17:32]         Retag([fn entry] _1);
[01:17:32]         Retag([fn entry] _2);
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = &mut (*_2)_2;
[01:17:32]         Retag(_3);
[01:17:32]         _0 = &mut (*_3)_3;
[01:17:32]         Retag(_0);
[01:17:32]         StorageDead(_3);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/uniform_array_move_out.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        _6 = move _1[-1 of 1]_1;"
[01:17:32] Expected Line: "     _6 = move _1[-1 of 1];"
[01:17:32] Test Name: rustc.move_out_from_end.UniformArrayMoveOut.before.mir
[01:17:32] ... (elided)
[01:17:32] ... (elided)
[01:17:32]     StorageLive(_6);
[01:17:32]      _6 = move _1[-1 of 1];
[01:17:32]      _0 = ();
[01:17:32] Actual:
[01:17:32] fn  move_out_from_end() -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: std::boxed::Box<i32>;
[01:17:32]     let mut _3: std::boxed::Box<i32>;
[01:17:32]     let mut _4: std::boxed::Box<i32>;
[01:17:32]     let mut _5: std::boxed::Box<i32>;
[01:17:32]     scope 1 {
[01:17:32]         let _1: [std::boxed::Box<i32>; 2];
[01:17:32]     scope 2 {
[01:17:32]         scope 3 {
[01:17:32]             let _6: std::boxed::Box<i32>;
[01:17:32]         }
[01:17:32]         }
[01:17:32]         scope 4 {
[01:17:32]         }
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         StorageLive(_3);
[01:17:32]         _3 = Box(i32);
[01:17:32]         (*_3)_3 = const 1i32;
[01:17:32]         _2 = move _3;
[01:17:32]         StorageDead(_3);
[01:17:32]         StorageLive(_4);
[01:17:32]         StorageLive(_5);
[01:17:32]         _5 = Box(i32);
[01:17:32]         (*_5)_5 = const 2i32;
[01:17:32]         _4 = move _5;
[01:17:32]         StorageDead(_5);
[01:17:32]         _1 = [move _2, move _4];
[01:17:32]         drop(_4) -> [return: bb3, unwind: bb2];
[01:17:32]     }
[01:17:32]     bb1 (cleanup): {
[01:17:32]         resume;
[01:17:32]     }
[01:17:32]     bb2 (cleanup): {
[01:17:32]         drop(_2) -> bb1;
[01:17:32]     bb3: {
[01:17:32]     bb3: {
[01:17:32]         StorageDead(_4);
[01:17:32]         drop(_2) -> [return: bb4, unwind: bb1];
[01:17:32]     bb4: {
[01:17:32]     bb4: {
[01:17:32]         StorageDead(_2);
[01:17:32]         FakeRead(ForLet, _1);
[01:17:32]         StorageLive(_6);
[01:17:32]         _6 = move _1[-1 of 1]_1;
[01:17:32]         _0 = ();
[01:17:32]         drop(_6) -> [return: bb6, unwind: bb5];
[01:17:32]     }
[01:17:32]     bb5 (cleanup): {
[01:17:32]         drop(_1) -> bb1;
[01:17:32]     bb6: {
[01:17:32]     bb6: {
[01:17:32]         StorageDead(_6);
[01:17:32]         drop(_1) -> [return: bb7, unwind: bb1];
[01:17:32]     bb7: {
[01:17:32]     bb7: {
[01:17:32]         StorageDead(_1);
[01:17:32]         return;
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/uninhabited-enum.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: None
[01:17:32] Actual Line: "        _2 = &(*_1)_1;"
[01:17:32] Expected Line: "    _2 = &(*_1);"
[01:17:32] Test Name: rustc.process_never.SimplifyLocals.after.mir
[01:17:32] ... (elided)
[01:17:32] bb0: {
[01:17:32] bb0: {
[01:17:32]     StorageLive(_2);
[01:17:32]     _2 = &(*_1);
[01:17:32]     StorageDead(_2);
[01:17:32] }
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] fn  process_never(_1: *const !) -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     scope 1 {
[01:17:32]         let _2: &!;
[01:17:32]     scope 2 {
[01:17:32]     }
[01:17:32]     scope 3 {
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_2);
[01:17:32]         _2 = &(*_1)_1;
[01:17:32]         StorageDead(_2);
[01:17:32]     }
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block: bb5 (cleanup): {
[01:17:32] Actual Line: "        drop((*_1)((*_1)_1.0: alloc::raw_vec::RawVec<i32>)_1) -> bb4;"
[01:17:32] Expected Line: "    drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb4;"
[01:17:32] Test Name: rustc.ptr-real_drop_in_place.std__vec__Vec_i32_.AddMovesForPackedDrops.before.mir
[01:17:32] ... (elided)
[01:17:32]     bb0: {
[01:17:32]     goto -> bb7;
[01:17:32] }
[01:17:32] }
[01:17:32] bb1: {
[01:17:32]     return;
[01:17:32] }
[01:17:32] bb2 (cleanup): {
[01:17:32]     resume;
[01:17:32] }
[01:17:32] bb3: {
[01:17:32]     goto -> bb1;
[01:17:32] }
[01:17:32] bb4 (cleanup): {
[01:17:32]     goto -> bb2;
[01:17:32] }
[01:17:32] bb5 (cleanup): {
[01:17:32]     drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb4;
[01:17:32] }
[01:17:32] bb6: {
[01:17:32]     drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb4];
[01:17:32] }
[01:17:32] bb7: {
[01:17:32]     _2 = &mut (*_1);
[01:17:32]     _3 = const std::ops::Drop::drop(move _2) -> [return: bb6, unwind: bb5];
[01:17:32] }
[01:17:32] Actual:
[01:17:32] fn  std::ptr::real_drop_in_place(_1: &mut std::vec::Vec<i32>) -> () {
[01:17:32]     let mut _0: ();
[01:17:32]     let mut _2: &mut std::vec::Vec<i32>;
[01:17:32]     let mut _3: ();
[01:17:32]     bb0: {
[01:17:32]         goto -> bb7;
[01:17:32]     bb1: {
[01:17:32]         return;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb2 (cleanup): {
[01:17:32]         resume;
[01:17:32]     bb3: {
[01:17:32]         goto -> bb1;
[01:17:32]     }
[01:17:32]     }
[01:17:32]     bb4 (cleanup): {
[01:17:32]         goto -> bb2;
[01:17:32]     }
[01:17:32]     bb5 (cleanup): {
[01:17:32]         drop((*_1)((*_1)_1.0: alloc::raw_vec::RawVec<i32>)_1) -> bb4;
[01:17:32]     bb6: {
[01:17:32]     bb6: {
[01:17:32]         drop((*_1)((*_1)_1.0: alloc::raw_vec::RawVec<i32>)_1) -> [return: bb3, unwind: bb4];
[01:17:32]     bb7: {
[01:17:32]     bb7: {
[01:17:32]         _2 = &mut (*_1)_1;
[01:17:32]         _3 = const std::ops::Drop::drop(move _2) -> [return: bb6, unwind: bb5];
[01:17:32] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:17:32] 
[01:17:32] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:17:32] ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
[01:17:32] thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:17:32] Current block:    bb0: {
[01:17:32] Actual Line: "        _4 = &(*_5)_5;"
[01:17:32] Expected Line: "       _4 = &(*_5);"
[01:17:32] Test Name: rustc.XXX.mir_map.0.mir
[01:17:32] ... (elided)
[01:17:32]    let mut _0: &'static Foo;
[01:17:32]    let mut _1: &'static Foo;
[01:17:32]    let _2: Foo;
[01:17:32]    let _2: Foo;
[01:17:32]    let mut _3: &'static [(u32, u32)];
[01:17:32]    let mut _4: &'static [(u32, u32); 42];
[01:17:32]    let mut _5: &'static [(u32, u32); 42];
[01:17:32]    let _6: [(u32, u32); 42];
[01:17:32]    let mut _7: (u32, u32);
[01:17:32]    let mut _8: (u32, u32);
[01:17:32]    let mut _9: (u32, u32);
[01:17:32]    let mut _10: (u32, u32);
[01:17:32]    let mut _11: (u32, u32);
[01:17:32]    let mut _12: (u32, u32);
[01:17:32]    let mut _13: (u32, u32);
[01:17:32]    let mut _14: (u32, u32);
[01:17:32]    let mut _15: (u32, u32);
[01:17:32]    let mut _16: (u32, u32);
[01:17:32]    let mut _17: (u32, u32);
[01:17:32]    let mut _18: (u32, u32);
[01:17:32]    let mut _19: (u32, u32);
[01:17:32]    let mut _20: (u32, u32);
[01:17:32]    let mut _21: (u32, u32);
[01:17:32]    let mut _22: (u32, u32);
[01:17:32]    let mut _23: (u32, u32);
[01:17:32]    let mut _24: (u32, u32);
[01:17:32]    let mut _25: (u32, u32);
[01:17:32]    let mut _26: (u32, u32);
[01:17:32]    let mut _27: (u32, u32);
[01:17:32]    let mut _28: (u32, u32);
[01:17:32]    let mut _29: (u32, u32);
[01:17:32]    let mut _30: (u32, u32);
[01:17:32]    let mut _31: (u32, u32);
[01:17:32]    let mut _32: (u32, u32);
[01:17:32]    let mut _33: (u32, u32);
[01:17:32]    let mut _34: (u32, u32);
[01:17:32]    let mut _35: (u32, u32);
[01:17:32]    let mut _36: (u32, u32);
[01:17:32]    let mut _37: (u32, u32);
[01:17:32]    let mut _38: (u32, u32);
[01:17:32]    let mut _39: (u32, u32);
[01:17:32]    let mut _40: (u32, u32);
[01:17:32]    let mut _41: (u32, u32);
[01:17:32]    let mut _42: (u32, u32);
[01:17:32]    let mut _43: (u32, u32);
[01:17:32]    let mut _44: (u32, u32);
[01:17:32]    let mut _45: (u32, u32);
[01:17:32]    let mut _46: (u32, u32);
[01:17:32]    let mut _47: (u32, u32);
[01:17:32]    let mut _48: (u32, u32);
[01:17:32]    bb0: {
[01:17:32]        StorageLive(_1);
[01:17:32]        StorageLive(_2);
[01:17:32]        StorageLive(_3);
[01:17:32]        StorageLive(_4);
[01:17:32]        StorageLive(_5);
[01:17:32]        StorageLive(_6);
[01:17:32]        StorageLive(_7);
[01:17:32]        _7 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_8);
[01:17:32]        _8 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_9);
[01:17:32]        _9 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_10);
[01:17:32]        _10 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_11);
[01:17:32]        _11 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_12);
[01:17:32]        _12 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_13);
[01:17:32]        _13 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_14);
[01:17:32]        _14 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_15);
[01:17:32]        _15 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_16);
[01:17:32]        _16 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_17);
[01:17:32]        _17 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_18);
[01:17:32]        _18 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_19);
[01:17:32]        _19 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_20);
[01:17:32]        _20 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_21);
[01:17:32]        _21 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_22);
[01:17:32]        _22 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_23);
[01:17:32]        _23 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_24);
[01:17:32]        _24 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_25);
[01:17:32]        _25 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_26);
[01:17:32]        _26 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_27);
[01:17:32]        _27 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_28);
[01:17:32]        _28 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_29);
[01:17:32]        _29 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_30);
[01:17:32]        _30 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_31);
[01:17:32]        _31 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_32);
[01:17:32]        _32 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_33);
[01:17:32]        _33 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_34);
[01:17:32]        _34 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_35);
[01:17:32]        _35 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_36);
[01:17:32]        _36 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_37);
[01:17:32]        _37 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_38);
[01:17:32]        _38 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_39);
[01:17:32]        _39 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_40);
[01:17:32]        _40 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_41);
[01:17:32]        _41 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_42);
[01:17:32]        _42 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_43);
[01:17:32]        _43 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_44);
[01:17:32]        _44 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_45);
[01:17:32]        _45 = (const 0u32, const 3u32);
[01:17:32]        StorageLive(_46);
[01:17:32]        _46 = (const 0u32, const 1u32);
[01:17:32]        StorageLive(_47);
[01:17:32]        _47 = (const 0u32, const 2u32);
[01:17:32]        StorageLive(_48);
[01:17:32]        _48 = (const 0u32, const 3u32);
[01:17:32]        _6 = [move _7, move _8, move _9, move _10, move _11, move _12, move _13, move _14, move _15, move _16, move _17, move _18, move _19, move _20, move _21, move _22, move _23, move _24, move _25, move _26, move _27, move _28, move _29, move _30, move _31, move _32, move _33, move _34, move _35, move _36, move _37, move _38, move _39, move _40, move _41, move _42, move _43, move _44, move _45, move _46, move _47, move _48];
[01:17:32]        _5 = &_6;
[01:17:32]        _4 = &(*_5);
[01:17:32]        _3 = move _4 as &'static [(u32, u32)] (Pointer(Unsize));
[01:17:32]        _2 = Foo { tup: const "hi", data: move _3 };
[01:17:32]        _1 = &_2;
[01:17:32]        _0 = &(*_1);
[01:17:32]        StorageDead(_1);
[01:17:32]        StorageDead(_5);
[01:17:32]        return;
[01:17:32] Actual:
[01:17:32] Actual:
[01:17:32] static  XXX: &'static Foo = {
[01:17:32]     let mut _0: &'static Foo;
[01:17:32]     let mut _1: &'static Foo;
[01:17:32]     let _2: Foo;
[01:17:32]     let mut _3: &'static [(u32, u32)];
[01:17:32]     let mut _4: &'static [(u32, u32); 42];
[01:17:32]     let mut _5: &'static [(u32, u32); 42];
[01:17:32]     let _6: [(u32, u32); 42];
[01:17:32]     let mut _7: (u32, u32);
[01:17:32]     let mut _8: (u32, u32);
[01:17:32]     let mut _9: (u32, u32);
[01:17:32]     let mut _10: (u32, u32);
[01:17:32]     let mut _11: (u32, u32);
[01:17:32]     let mut _12: (u32, u32);
[01:17:32]     let mut _13: (u32, u32);
[01:17:32]     let mut _14: (u32, u32);
[01:17:32]     let mut _15: (u32, u32);
[01:17:32]     let mut _16: (u32, u32);
[01:17:32]     let mut _17: (u32, u32);
[01:17:32]     let mut _18: (u32, u32);
[01:17:32]     let mut _19: (u32, u32);
[01:17:32]     let mut _20: (u32, u32);
[01:17:32]     let mut _21: (u32, u32);
[01:17:32]     let mut _22: (u32, u32);
[01:17:32]     let mut _23: (u32, u32);
[01:17:32]     let mut _24: (u32, u32);
[01:17:32]     let mut _25: (u32, u32);
[01:17:32]     let mut _26: (u32, u32);
[01:17:32]     let mut _27: (u32, u32);
[01:17:32]     let mut _28: (u32, u32);
[01:17:32]     let mut _29: (u32, u32);
[01:17:32]     let mut _30: (u32, u32);
[01:17:32]     let mut _31: (u32, u32);
[01:17:32]     let mut _32: (u32, u32);
[01:17:32]     let mut _33: (u32, u32);
[01:17:32]     let mut _34: (u32, u32);
[01:17:32]     let mut _35: (u32, u32);
[01:17:32]     let mut _36: (u32, u32);
[01:17:32]     let mut _37: (u32, u32);
[01:17:32]     let mut _38: (u32, u32);
[01:17:32]     let mut _39: (u32, u32);
[01:17:32]     let mut _40: (u32, u32);
[01:17:32]     let mut _41: (u32, u32);
[01:17:32]     let mut _42: (u32, u32);
[01:17:32]     let mut _43: (u32, u32);
[01:17:32]     let mut _44: (u32, u32);
[01:17:32]     let mut _45: (u32, u32);
[01:17:32]     let mut _46: (u32, u32);
[01:17:32]     let mut _47: (u32, u32);
[01:17:32]     let mut _48: (u32, u32);
[01:17:32]     bb0: {
[01:17:32]         StorageLive(_1);
[01:17:32]         StorageLive(_2);
[01:17:32]         StorageLive(_3);
[01:17:32]         StorageLive(_4);
[01:17:32]         StorageLive(_5);
[01:17:32]         StorageLive(_6);
[01:17:32]         StorageLive(_7);
[01:17:32]         _7 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_8);
[01:17:32]         _8 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_9);
[01:17:32]         _9 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_10);
[01:17:32]         _10 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_11);
[01:17:32]         _11 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_12);
[01:17:32]         _12 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_13);
[01:17:32]         _13 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_14);
[01:17:32]         _14 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_15);
[01:17:32]         _15 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_16);
[01:17:32]         _16 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_17);
[01:17:32]         _17 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_18);
[01:17:32]         _18 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_19);
[01:17:32]         _19 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_20);
[01:17:32]         _20 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_21);
[01:17:32]         _21 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_22);
[01:17:32]         _22 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_23);
[01:17:32]         _23 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_24);
[01:17:32]         _24 = (const 0u32, const 3u32);
[01:17:32]         StorageLive(_25);
[01:17:32]         _25 = (const 0u32, const 1u32);
[01:17:32]         StorageLive(_26);
[01:17:32]         _26 = (const 0u32, const 2u32);
[01:17:32]         StorageLive(_27);
---
[01:17:32] test result: FAILED. 22 passed; 20 failed; 0 ignored; 0 measured; 0 filtered out
[01:17:32] 
[01:17:32] 
[01:17:32] 
[01:17:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:32] 
[01:17:32] 
[01:17:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:32] Build completed unsuccessfully in 0:11:10
[01:17:32] Build completed unsuccessfully in 0:11:10
[01:17:32] make: *** [check] Error 1
[01:17:32] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f4706d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 00:48:42 UTC 2019
---
travis_time:end:1f860b90:start=1556153323678183868,finish=1556153323682712480,duration=4528612
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1adc9d55
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02639598
travis_time:start:02639598
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12b791b8
$ dmesg | grep -i kill
