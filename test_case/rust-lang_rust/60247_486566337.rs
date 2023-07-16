plain
travis_time:end:164b1bc0:start=1556174794451330992,finish=1556174878877354361,duration=84426023369
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:26:10] running 42 tests
[01:26:12] ERROR 2019-04-25T08:14:20Z: compiletest::runtest: None
[01:26:13] ERROR 2019-04-25T08:14:20Z: compiletest::runtest: None
[01:26:13] ERROR 2019-04-25T08:14:20Z: compiletest::runtest: None
[01:26:18] ERROR 2019-04-25T08:14:26Z: compiletest::runtest: Some(" bb8: { // binding1 and guard")
[01:26:21] ERROR 2019-04-25T08:14:28Z: compiletest::runtest: Some("bb5: {")
[01:26:25] ERROR 2019-04-25T08:14:33Z: compiletest::runtest: Some("bb5 (cleanup): {")
[01:26:30] .....F..FFF............F.....F........F...
[01:26:30] failures:
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/deaggregator_test_enum_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block: None
[01:26:30] Actual Line: "        ((_0.0: i32) as A) = move _4;"
[01:26:30] Expected Line: "     ((_0 as A).0: i32) = move _4;"
[01:26:30] Test Name: rustc.test1.Deaggregator.after.mir
[01:26:30] ... (elided)
[01:26:30]  bb1: {
[01:26:30]  bb1: {
[01:26:30]      StorageLive(_4);
[01:26:30]      _4 = _2;
[01:26:30]      ((_0 as A).0: i32) = move _4;
[01:26:30]      discriminant(_0) = 0;
[01:26:30]      StorageDead(_4);
[01:26:30]      goto -> bb3;
[01:26:30]  bb2: {
[01:26:30]  bb2: {
[01:26:30]      StorageLive(_5);
[01:26:30]      _5 = _2;
[01:26:30]      ((_0 as B).0: i32) = move _5;
[01:26:30]      discriminant(_0) = 1;
[01:26:30]      StorageDead(_5);
[01:26:30]      goto -> bb3;
[01:26:30] Actual:
[01:26:30] Actual:
[01:26:30] fn  test1(_1: bool, _2: i32) -> Foo {
[01:26:30]     let mut _0: Foo;
[01:26:30]     let mut _3: bool;
[01:26:30]     let mut _4: i32;
[01:26:30]     let mut _5: i32;
[01:26:30]     bb0: {
[01:26:30]         StorageLive(_3);
[01:26:30]         _3 = _1;
[01:26:30]         switchInt(move _3) -> [false: bb2, otherwise: bb1];
[01:26:30]     bb1: {
[01:26:30]     bb1: {
[01:26:30]         StorageLive(_4);
[01:26:30]         _4 = _2;
[01:26:30]         ((_0.0: i32) as A) = move _4;
[01:26:30]         discriminant(_0) = 0;
[01:26:30]         StorageDead(_4);
[01:26:30]         goto -> bb3;
[01:26:30]     bb2: {
[01:26:30]     bb2: {
[01:26:30]         StorageLive(_5);
[01:26:30]         _5 = _2;
[01:26:30]         ((_0.0: i32) as B) = move _5;
[01:26:30]         discriminant(_0) = 1;
[01:26:30]         StorageDead(_5);
[01:26:30]         goto -> bb3;
[01:26:30]     bb3: {
[01:26:30]     bb3: {
[01:26:30]         StorageDead(_3);
[01:26:30]         return;
[01:26:30]     }
[01:26:30]     bb4 (cleanup): {
[01:26:30]         resume;
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/deaggregator_test_multiple.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:26:30] Expected Line: "    ((_2 as A).0: i32) = move _3;"
[01:26:30] Test Name: rustc.test.Deaggregator.after.mir
[01:26:30] ... (elided)
[01:26:30] bb0: {
[01:26:30] ... (elided)
[01:26:30]     _3 = _1;
[01:26:30]     _3 = _1;
[01:26:30] ... (elided)
[01:26:30]     ((_2 as A).0: i32) = move _3;
[01:26:30]     discriminant(_2) = 0;
[01:26:30] ... (elided)
[01:26:30]     _5 = _1;
[01:26:30]     ((_4 as A).0: i32) = move _5;
[01:26:30]     discriminant(_4) = 0;
[01:26:30] ... (elided)
[01:26:30]     _0 = [move _2, move _4];
[01:26:30] ... (elided)
[01:26:30]     return;
[01:26:30] }
[01:26:30] Actual:
[01:26:30] fn  test(_1: i32) -> [Foo; 2] {
[01:26:30]     let mut _0: [Foo; 2];
[01:26:30]     let mut _2: Foo;
[01:26:30]     let mut _3: i32;
[01:26:30]     let mut _4: Foo;
[01:26:30]     let mut _5: i32;
[01:26:30]     bb0: {
[01:26:30]         StorageLive(_2);
[01:26:30]         StorageLive(_3);
[01:26:30]         _3 = _1;
[01:26:30]         ((_2.0: i32) as A) = move _3;
[01:26:30]         discriminant(_2) = 0;
[01:26:30]         StorageDead(_3);
[01:26:30]         StorageLive(_4);
[01:26:30]         StorageLive(_5);
[01:26:30]         _5 = _1;
[01:26:30]         ((_4.0: i32) as A) = move _5;
[01:26:30]         discriminant(_4) = 0;
[01:26:30]         StorageDead(_5);
[01:26:30]         _0 = [move _2, move _4];
[01:26:30]         StorageDead(_4);
[01:26:30]         StorageDead(_2);
[01:26:30]         return;
[01:26:30]     }
[01:26:30]     bb1 (cleanup): {
[01:26:30]         resume;
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/generator-drop-cleanup.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block: None
[01:26:30] Actual Line: "        switchInt((*(_1.0: u32))) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];"
[01:26:30] Expected Line: "    switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];"
[01:26:30] Test Name: rustc.main-{{closure}}.generator_drop.0.mir
[01:26:30] ... (elided)
[01:26:30] bb0: {
[01:26:30] bb0: {
[01:26:30]     switchInt(((*_1).0: u32)) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:26:30] }
[01:26:30] bb1: {
[01:26:30]     goto -> bb5;
[01:26:30] }
[01:26:30] bb2: {
[01:26:30]     return;
[01:26:30] }
[01:26:30] bb3: {
[01:26:30]     return;
[01:26:30] }
[01:26:30] bb4: {
[01:26:30]     goto -> bb6;
[01:26:30] }
[01:26:30] bb5: {
[01:26:30]     goto -> bb2;
[01:26:30] }
[01:26:30] bb6: {
[01:26:30]     goto -> bb3;
[01:26:30] }
[01:26:30] bb7: {
[01:26:30]     StorageLive(_3);
[01:26:30]     goto -> bb1;
[01:26:30] }
[01:26:30] bb8: {
[01:26:30]     return;
[01:26:30] }
[01:26:30] Actual:
[01:26:30] fn  main::{{closure}}#0(_1: *mut [generator@/checkout/src/test/mir-opt/generator-drop-cleanup.rs:7:15: 9:6 {()}]) -> () {
[01:26:30]     let mut _0: ();
[01:26:30]     let mut _2: ();
[01:26:30]     let mut _3: ();
[01:26:30]     let mut _4: ();
[01:26:30]     bb0: {
[01:26:30]         switchInt((*(_1.0: u32))) -> [0u32: bb4, 3u32: bb7, otherwise: bb8];
[01:26:30]     bb1: {
[01:26:30]         goto -> bb5;
[01:26:30]     }
[01:26:30]     bb2: {
---
[01:26:30]     }
[01:26:30]     bb6: {
[01:26:30]         goto -> bb3;
[01:26:30]     }
[01:26:30]     bb7: {
[01:26:30]         StorageLive(_3);
[01:26:30]         goto -> bb1;
[01:26:30]     bb8: {
[01:26:30]         return;
[01:26:30]     }
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/deaggregator_test_enum.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/deaggregator_test_enum.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block: None
[01:26:30] Actual Line: "        ((_0.0: usize) as Foo) = move _2;"
[01:26:30] Expected Line: "    ((_0 as Foo).0: usize) = move _2;"
[01:26:30] Test Name: rustc.bar.Deaggregator.after.mir
[01:26:30] ... (elided)
[01:26:30] bb0: {
[01:26:30] bb0: {
[01:26:30]     StorageLive(_2);
[01:26:30]     _2 = _1;
[01:26:30]     ((_0 as Foo).0: usize) = move _2;
[01:26:30]     discriminant(_0) = 1;
[01:26:30]     StorageDead(_2);
[01:26:30]     return;
[01:26:30] }
[01:26:30] Actual:
[01:26:30] fn  bar(_1: usize) -> Baz {
[01:26:30]     let mut _0: Baz;
[01:26:30]     let mut _2: usize;
[01:26:30]     bb0: {
[01:26:30]         StorageLive(_2);
[01:26:30]         _2 = _1;
[01:26:30]         ((_0.0: usize) as Foo) = move _2;
[01:26:30]         discriminant(_0) = 1;
[01:26:30]         StorageDead(_2);
[01:26:30]         return;
[01:26:30]     }
[01:26:30]     bb1 (cleanup): {
[01:26:30]         resume;
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block:  bb8: { // binding1 and guard
[01:26:30] Actual Line: "        _6 = &(((promoted[0]: std::option::Option<i32>).0: i32) as Some);"
[01:26:30] Expected Line: "     _6 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);"
[01:26:30] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[01:26:30] ... (elided)
[01:26:30]  bb0: {
[01:26:30] ... (elided)
[01:26:30] ... (elided)
[01:26:30]      _2 = std::option::Option::<i32>::Some(const 42i32,);
[01:26:30]      FakeRead(ForMatchedPlace, _2);
[01:26:30]      _3 = discriminant(_2);
[01:26:30]      switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:26:30]  }
[01:26:30]  bb1 (cleanup): {
[01:26:30]      resume;
[01:26:30]  bb2: {
[01:26:30]  bb2: {
[01:26:30]      falseEdges -> [real: bb8, imaginary: bb3]; //pre_binding1
[01:26:30]  bb3: {
[01:26:30]  bb3: {
[01:26:30]      falseEdges -> [real: bb11, imaginary: bb4]; //pre_binding2
[01:26:30]  bb4: {
[01:26:30]  bb4: {
[01:26:30]      falseEdges -> [real: bb12, imaginary: bb5]; //pre_binding3
[01:26:30]  bb5: {
[01:26:30]      unreachable;
[01:26:30]  }
[01:26:30]  }
[01:26:30]  bb6: { // to pre_binding2
[01:26:30]      falseEdges -> [real: bb3, imaginary: bb3];
[01:26:30]  bb7: {
[01:26:30]      unreachable;
[01:26:30]  }
[01:26:30]  }
[01:26:30]  bb8: { // binding1 and guard
[01:26:30]      StorageLive(_6);
[01:26:30]      _6 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[01:26:30]      _4 = &shallow _2;
[01:26:30]      StorageLive(_7);
[01:26:30]      _7 = const guard() -> [return: bb9, unwind: bb1];
[01:26:30]  bb9: {
[01:26:30]  bb9: {
[01:26:30]      FakeRead(ForMatchGuard, _4);
[01:26:30]      FakeRead(ForGuardBinding, _6);
[01:26:30]      switchInt(move _7) -> [false: bb6, otherwise: bb10];
[01:26:30]  bb10: {
[01:26:30]  bb10: {
[01:26:30]      StorageLive(_5);
[01:26:30]      _5 = ((_2 as Some).0: i32);
[01:26:30]      StorageLive(_8);
[01:26:30]      _8 = _5;
[01:26:30]      _1 = (const 1i32, move _8);
[01:26:30]      StorageDead(_8);
[01:26:30]      goto -> bb13;
[01:26:30]  bb11: {
[01:26:30]  bb11: {
[01:26:30]      StorageLive(_9);
[01:26:30]      _9 = ((_2 as Some).0: i32);
[01:26:30]      StorageLive(_10);
[01:26:30]      _10 = _9;
[01:26:30]      _1 = (const 2i32, move _10);
[01:26:30]      StorageDead(_10);
[01:26:30]      goto -> bb13;
[01:26:30]  bb12: {
[01:26:30]  bb12: {
[01:26:30]      _1 = (const 3i32, const 3i32);
[01:26:30]      goto -> bb13;
[01:26:30]  bb13: {
[01:26:30] ... (elided)
[01:26:30]      return;
[01:26:30]  }
[01:26:30]  }
[01:26:30] Actual:
[01:26:30] fn  full_tested_match() -> () {
[01:26:30]     let mut _0: ();
[01:26:30]     let mut _1: (i32, i32);
[01:26:30]     let mut _2: std::option::Option<i32>;
[01:26:30]     let mut _3: isize;
[01:26:30]     let mut _4: &std::option::Option<i32>;
[01:26:30]     let _5: i32;
[01:26:30]     let _6: &i32;
[01:26:30]     let mut _7: bool;
[01:26:30]     let mut _8: i32;
[01:26:30]     let mut _10: i32;
[01:26:30]     scope 1 {
[01:26:30]         let _9: i32;
[01:26:30]         scope 2 {
[01:26:30]     }
[01:26:30]     bb0: {
[01:26:30]     bb0: {
[01:26:30]         StorageLive(_1);
[01:26:30]         StorageLive(_2);
[01:26:30]         _2 = std::option::Option::<i32>::Some(const 42i32,);
[01:26:30]         FakeRead(ForMatchedPlace, _2);
[01:26:30]         _3 = discriminant(_2);
[01:26:30]         switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:26:30]     }
[01:26:30]     bb1 (cleanup): {
[01:26:30]         resume;
[01:26:30]     bb2: {
[01:26:30]     bb2: {
[01:26:30]         falseEdges -> [real: bb8, imaginary: bb3];
[01:26:30]     bb3: {
[01:26:30]     bb3: {
[01:26:30]         falseEdges -> [real: bb11, imaginary: bb4];
[01:26:30]     bb4: {
[01:26:30]     bb4: {
[01:26:30]         falseEdges -> [real: bb12, imaginary: bb5];
[01:26:30]     bb5: {
[01:26:30]         unreachable;
[01:26:30]     }
[01:26:30]     bb6: {
[01:26:30]     bb6: {
[01:26:30]         falseEdges -> [real: bb3, imaginary: bb3];
[01:26:30]     bb7: {
[01:26:30]         unreachable;
[01:26:30]     }
[01:26:30]     bb8: {
[01:26:30]     bb8: {
[01:26:30]         StorageLive(_6);
[01:26:30]         _6 = &(((promoted[0]: std::option::Option<i32>).0: i32) as Some);
[01:26:30]         _4 = &shallow _2;
[01:26:30]         StorageLive(_7);
[01:26:30]         _7 = const guard() -> [return: bb9, unwind: bb1];
[01:26:30]     bb9: {
[01:26:30]     bb9: {
[01:26:30]         FakeRead(ForMatchGuard, _4);
[01:26:30]         FakeRead(ForGuardBinding, _6);
[01:26:30]         switchInt(move _7) -> [false: bb6, otherwise: bb10];
[01:26:30]     bb10: {
[01:26:30]     bb10: {
[01:26:30]         StorageLive(_5);
[01:26:30]         _5 = ((_2.0: i32) as Some);
[01:26:30]         StorageLive(_8);
[01:26:30]         _8 = _5;
[01:26:30]         _1 = (const 1i32, move _8);
[01:26:30]         StorageDead(_8);
[01:26:30]         goto -> bb13;
[01:26:30]     bb11: {
[01:26:30]     bb11: {
[01:26:30]         StorageLive(_9);
[01:26:30]         _9 = ((_2.0: i32) as Some);
[01:26:30]         StorageLive(_10);
[01:26:30]         _10 = _9;
[01:26:30]         _1 = (const 2i32, move _10);
[01:26:30]         StorageDead(_10);
[01:26:30]         goto -> bb13;
[01:26:30]     bb12: {
[01:26:30]     bb12: {
[01:26:30]         _1 = (const 3i32, const 3i32);
[01:26:30]         goto -> bb13;
[01:26:30]     bb13: {
[01:26:30]     bb13: {
[01:26:30]         StorageDead(_9);
[01:26:30]         StorageDead(_5);
[01:26:30]         StorageDead(_7);
[01:26:30]         StorageDead(_6);
[01:26:30]         StorageDead(_1);
[01:26:30]         StorageDead(_2);
[01:26:30]         _0 = ();
[01:26:30]         return;
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/remove_fake_borrows.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block: bb5: {
[01:26:30] Actual Line: "        switchInt((((*(*_1)).0: &\'<empty> &\'<empty> i32) as Some)) -> [0i32: bb1, otherwise: bb2];"
[01:26:30] Expected Line: "    switchInt((*(*((_1 as Some).0: &\'<empty> &\'<empty> i32)))) -> [0i32: bb1, otherwise: bb2];"
[01:26:30] Test Name: rustc.match_guard.CleanupNonCodegenStatements.before.mir
[01:26:30] ... (elided)
[01:26:30] bb0: {
[01:26:30] bb0: {
[01:26:30]     FakeRead(ForMatchedPlace, _1);
[01:26:30]     _3 = discriminant(_1);
[01:26:30]     switchInt(move _3) -> [1isize: bb5, otherwise: bb2];
[01:26:30] }
[01:26:30] bb1: {
[01:26:30]     goto -> bb7;
[01:26:30] }
[01:26:30] bb2: {
[01:26:30]     goto -> bb8;
[01:26:30] }
[01:26:30] bb3: {
[01:26:30] }
[01:26:30] bb4: {
[01:26:30]     goto -> bb2;
[01:26:30] }
[01:26:30] }
[01:26:30] bb5: {
[01:26:30]     switchInt((*(*((_1 as Some).0: &'<empty> &'<empty> i32)))) -> [0i32: bb1, otherwise: bb2];
[01:26:30] }
[01:26:30] bb6: {
[01:26:30]     _0 = const 0i32;
[01:26:30]     goto -> bb9;
[01:26:30] }
[01:26:30] bb7: {
[01:26:30]     _4 = &shallow _1;
[01:26:30]     _5 = &shallow ((_1 as Some).0: &'<empty> &'<empty> i32);
[01:26:30]     _6 = &shallow (*((_1 as Some).0: &'<empty> &'<empty> i32));
[01:26:30]     _7 = &shallow (*(*((_1 as Some).0: &'<empty> &'<empty> i32)));
[01:26:30]     StorageLive(_8);
[01:26:30]     _8 = _2;
[01:26:30]     FakeRead(ForMatchGuard, _4);
[01:26:30]     FakeRead(ForMatchGuard, _5);
[01:26:30]     FakeRead(ForMatchGuard, _6);
[01:26:30]     FakeRead(ForMatchGuard, _7);
[01:26:30]     switchInt(move _8) -> [false: bb4, otherwise: bb6];
[01:26:30] }
[01:26:30] bb8: {
[01:26:30]     _0 = const 1i32;
[01:26:30]     goto -> bb9;
[01:26:30] }
[01:26:30] bb9: {
[01:26:30]     StorageDead(_8);
[01:26:30]     return;
[01:26:30] }
[01:26:30] bb10 (cleanup): {
[01:26:30]     resume;
[01:26:30] }
[01:26:30] Actual:
[01:26:30] fn  match_guard(_1: std::option::Option<&&i32>, _2: bool) -> i32 {
[01:26:30]     let mut _0: i32;
[01:26:30]     let mut _3: isize;
[01:26:30]     let mut _4: &std::option::Option<&&i32>;
[01:26:30]     let mut _5: &&'<empty> &'<empty> i32;
[01:26:30]     let mut _6: &&'<empty> i32;
[01:26:30]     let mut _7: &i32;
[01:26:30]     let mut _8: bool;
[01:26:30]     bb0: {
[01:26:30]         FakeRead(ForMatchedPlace, _1);
[01:26:30]         _3 = discriminant(_1);
[01:26:30]         switchInt(move _3) -> [1isize: bb5, otherwise: bb2];
[01:26:30]     bb1: {
[01:26:30]         goto -> bb7;
[01:26:30]     }
[01:26:30]     bb2: {
---
[01:26:30]     }
[01:26:30]     bb4: {
[01:26:30]         goto -> bb2;
[01:26:30]     }
[01:26:30]     bb5: {
[01:26:30]         switchInt((((*(*_1)).0: &'<empty> &'<empty> i32) as Some)) -> [0i32: bb1, otherwise: bb2];
[01:26:30]     bb6: {
[01:26:30]         _0 = const 0i32;
[01:26:30]         goto -> bb9;
[01:26:30]     }
[01:26:30]     }
[01:26:30]     bb7: {
[01:26:30]         _4 = &shallow _1;
[01:26:30]         _5 = &shallow ((_1.0: &'<empty> &'<empty> i32) as Some);
[01:26:30]         _6 = &shallow (((*_1).0: &'<empty> &'<empty> i32) as Some);
[01:26:30]         _7 = &shallow (((*(*_1)).0: &'<empty> &'<empty> i32) as Some);
[01:26:30]         StorageLive(_8);
[01:26:30]         _8 = _2;
[01:26:30]         FakeRead(ForMatchGuard, _4);
[01:26:30]         FakeRead(ForMatchGuard, _5);
[01:26:30]         FakeRead(ForMatchGuard, _6);
[01:26:30]         FakeRead(ForMatchGuard, _7);
[01:26:30]         switchInt(move _8) -> [false: bb4, otherwise: bb6];
[01:26:30]     bb8: {
[01:26:30]         _0 = const 1i32;
[01:26:30]         goto -> bb9;
[01:26:30]     }
[01:26:30]     }
[01:26:30]     bb9: {
[01:26:30]         StorageDead(_8);
[01:26:30]         return;
[01:26:30]     }
[01:26:30]     bb10 (cleanup): {
[01:26:30]         resume;
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
[01:26:30] ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
[01:26:30] thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:26:30] Current block: bb5 (cleanup): {
[01:26:30] Actual Line: "        drop((*(_1.0: alloc::raw_vec::RawVec<i32>))) -> bb4;"
[01:26:30] Expected Line: "    drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb4;"
[01:26:30] Test Name: rustc.ptr-real_drop_in_place.std__vec__Vec_i32_.AddMovesForPackedDrops.before.mir
[01:26:30] ... (elided)
[01:26:30]     bb0: {
[01:26:30]     goto -> bb7;
[01:26:30] }
[01:26:30] }
[01:26:30] bb1: {
[01:26:30]     return;
[01:26:30] }
[01:26:30] bb2 (cleanup): {
[01:26:30]     resume;
[01:26:30] }
[01:26:30] bb3: {
[01:26:30]     goto -> bb1;
[01:26:30] }
[01:26:30] bb4 (cleanup): {
[01:26:30]     goto -> bb2;
[01:26:30] }
[01:26:30] bb5 (cleanup): {
[01:26:30]     drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb4;
[01:26:30] }
[01:26:30] bb6: {
[01:26:30]     drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb4];
[01:26:30] }
[01:26:30] bb7: {
[01:26:30]     _2 = &mut (*_1);
[01:26:30]     _3 = const std::ops::Drop::drop(move _2) -> [return: bb6, unwind: bb5];
[01:26:30] }
[01:26:30] Actual:
[01:26:30] fn  std::ptr::real_drop_in_place(_1: &mut std::vec::Vec<i32>) -> () {
[01:26:30]     let mut _0: ();
[01:26:30]     let mut _2: &mut std::vec::Vec<i32>;
[01:26:30]     let mut _3: ();
[01:26:30]     bb0: {
[01:26:30]         goto -> bb7;
[01:26:30]     bb1: {
[01:26:30]         return;
[01:26:30]     }
[01:26:30]     }
[01:26:30]     bb2 (cleanup): {
[01:26:30]         resume;
[01:26:30]     bb3: {
[01:26:30]         goto -> bb1;
[01:26:30]     }
[01:26:30]     }
[01:26:30]     bb4 (cleanup): {
[01:26:30]         goto -> bb2;
[01:26:30]     }
[01:26:30]     bb5 (cleanup): {
[01:26:30]         drop((*(_1.0: alloc::raw_vec::RawVec<i32>))) -> bb4;
[01:26:30]     bb6: {
[01:26:30]     bb6: {
[01:26:30]         drop((*(_1.0: alloc::raw_vec::RawVec<i32>))) -> [return: bb3, unwind: bb4];
[01:26:30]     bb7: {
[01:26:30]     bb7: {
[01:26:30]         _2 = &mut (*_1);
[01:26:30]         _3 = const std::ops::Drop::drop(move _2) -> [return: bb6, unwind: bb5];
[01:26:30] }', src/tools/compiletest/src/runtest.rs:3061:13
[01:26:30] 
[01:26:30] 
[01:26:30] failures:
---
[01:26:30] test result: FAILED. 35 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:30] 
[01:26:30] 
[01:26:30] 
[01:26:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:30] 
[01:26:30] 
[01:26:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:30] Build completed unsuccessfully in 0:12:15
[01:26:30] Build completed unsuccessfully in 0:12:15
[01:26:30] make: *** [check] Error 1
[01:26:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a44aa3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 08:14:38 UTC 2019
---
travis_time:end:050f9662:start=1556180080025804040,finish=1556180080073759520,duration=47955480
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3066dd22
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00e8e7d4
$ dmesg | grep -i kill
