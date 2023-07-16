plain
travis_time:end:0302dd08:start=1549745896002837309,finish=1549745898145742487,duration=2142905178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:53] 
[01:11:53] running 38 tests
[01:12:00] ERROR 2019-02-09T22:10:29Z: compiletest::runtest: Some("    bb5: {")
[01:12:03] ERROR 2019-02-09T22:10:33Z: compiletest::runtest: Some(" bb2: {")
[01:12:04] ERROR 2019-02-09T22:10:34Z: compiletest::runtest: Some("   bb1: {")
[01:12:06] ERROR 2019-02-09T22:10:36Z: compiletest::runtest: Some("bb1: {")
[01:12:17] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:12:17] ....................FFF....F..........
[01:12:17] 
[01:12:17] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[01:12:17] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[01:12:17] thread '[mir-opt] mir-opt/issue-49232.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:17] Current block:     bb5: {
[01:12:17] Actual Line: "        falseEdges -> [real: bb11, imaginary: bb6];"
[01:12:17] Expected Line: "        falseEdges -> [real: bb12, imaginary: bb6];"
[01:12:17] Test Name: rustc.main.mir_map.0.mir
[01:12:17] ... (elided)
[01:12:17] fn main() -> (){
[01:12:17] fn main() -> (){
[01:12:17]     let mut _0: ();
[01:12:17]     scope 1 {
[01:12:17]     scope 2 {
[01:12:17]         let _2: i32;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     let mut _1: ();
[01:12:17]     let mut _3: bool;
[01:12:17]     let mut _4: !;
[01:12:17]     let mut _5: ();
[01:12:17]     let mut _6: &i32;
[01:12:17]     bb0: {
[01:12:17]         goto -> bb1;
[01:12:17]     bb1: {
[01:12:17]     bb1: {
[01:12:17]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:12:17]     bb2: {
[01:12:17]         goto -> bb20;
[01:12:17]     }
[01:12:17]     bb3: {
[01:12:17]     bb3: {
[01:12:17]         StorageLive(_2);
[01:12:17]         StorageLive(_3);
[01:12:17]         _3 = const true;
[01:12:17]         FakeRead(ForMatchedPlace, _3);
[01:12:17]         switchInt(_3) -> [false: bb9, otherwise: bb8];
[01:12:17]     bb4: {
[01:12:17]         resume;
[01:12:17]     }
[01:12:17]     bb5: {
[01:12:17]     bb5: {
[01:12:17]         falseEdges -> [real: bb12, imaginary: bb6];
[01:12:17]     bb6: {
[01:12:17]     bb6: {
[01:12:17]         falseEdges -> [real: bb14, imaginary: bb7];
[01:12:17]     bb7: {
[01:12:17]         unreachable;
[01:12:17]     }
[01:12:17]     bb8: {
[01:12:17]     bb8: {
[01:12:17]         goto -> bb6;
[01:12:17]     }
[01:12:17]     bb9: {
[01:12:17]         goto -> bb5;
[01:12:17]     }
[01:12:17]     bb10: {
[01:12:17]         FakeRead(ForLet, _2);
[01:12:17]         StorageDead(_3);
[01:12:17]         StorageLive(_6);
[01:12:17]         _6 = &_2;
[01:12:17]         _5 = const std::mem::drop(move _6) -> [return: bb19, unwind: bb4];
[01:12:17]     bb11: {
[01:12:17]         _2 = const 4i32;
[01:12:17]         goto -> bb10;
[01:12:17]     }
---
[01:12:17]     }
[01:12:17]     bb14: {
[01:12:17]         goto -> bb13;
[01:12:17]     }
[01:12:17]     bb15: {
[01:12:17]         StorageDead(_3);
[01:12:17]         goto -> bb16;
[01:12:17]     bb16: {
[01:12:17]     bb16: {
[01:12:17]         StorageDead(_2);
[01:12:17]         goto -> bb2;
[01:12:17]     bb17: {
[01:12:17]         _4 = ();
[01:12:17]         unreachable;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb18: {
[01:12:17]         StorageDead(_4);
[01:12:17]         goto -> bb10;
[01:12:17]     bb19: {
[01:12:17]     bb19: {
[01:12:17]         StorageDead(_6);
[01:12:17]         _1 = ();
[01:12:17]         StorageDead(_2);
[01:12:17]         goto -> bb1;
[01:12:17]     bb20: {
[01:12:17]         return;
[01:12:17]     }
[01:12:17] }
[01:12:17] }
[01:12:17] Actual:
[01:12:17] fn main() -> (){
[01:12:17]     let mut _0: ();
[01:12:17]     scope 1 {
[01:12:17]     scope 2 {
[01:12:17]         let _2: i32;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     let mut _1: ();
[01:12:17]     let mut _3: bool;
[01:12:17]     let mut _4: !;
[01:12:17]     let mut _5: ();
[01:12:17]     let mut _6: &i32;
[01:12:17]     bb0: {                              
[01:12:17]         goto -> bb1;
[01:12:17]     }
[01:12:17]     bb1: {                              
[01:12:17]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:12:17]     }
[01:12:17]     bb2: {                              
[01:12:17]         goto -> bb20;
[01:12:17]     }
[01:12:17]     bb3: {                              
[01:12:17]         StorageLive(_2);
[01:12:17]         StorageLive(_3);
[01:12:17]         _3 = const true;
[01:12:17]         FakeRead(ForMatchedPlace, _3);
[01:12:17]         switchInt(_3) -> [false: bb9, otherwise: bb8];
[01:12:17]     bb4: {
[01:12:17]         resume;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb5: {                              
[01:12:17]         falseEdges -> [real: bb11, imaginary: bb6];
[01:12:17]     }
[01:12:17]     bb6: {                              
[01:12:17]         falseEdges -> [real: bb13, imaginary: bb7];
[01:12:17]     }
[01:12:17]     bb7: {                              
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb8: {                              
[01:12:17]         goto -> bb6;
[01:12:17]     }
[01:12:17]     bb9: {                              
[01:12:17]         goto -> bb5;
[01:12:17]     }
[01:12:17]     bb10: {                             
[01:12:17]         _2 = const 4i32;
[01:12:17]         goto -> bb18;
[01:12:17]     }
[01:12:17]     bb11: {                             
[01:12:17]         goto -> bb10;
[01:12:17]     }
[01:12:17]     bb12: {                             
[01:12:17]         _0 = ();
[01:12:17]         goto -> bb14;
[01:12:17]     }
[01:12:17]     bb13: {                             
[01:12:17]         goto -> bb12;
[01:12:17]     }
[01:12:17]     bb14: {                             
[01:12:17]         StorageDead(_3);
[01:12:17]         goto -> bb15;
[01:12:17]     }
[01:12:17]     bb15: {                             
[01:12:17]         StorageDead(_2);
[01:12:17]         goto -> bb2;
[01:12:17]     }
[01:12:17]     bb16: {                             
[01:12:17]         _4 = ();
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb17: {                             
[01:12:17]         StorageDead(_4);
[01:12:17]         goto -> bb18;
[01:12:17]     }
[01:12:17]     bb18: {                             
[01:12:17]         FakeRead(ForLet, _2);
[01:12:17]         StorageDead(_3);
[01:12:17]         StorageLive(_6);
[01:12:17]         _6 = &_2;
[01:12:17]         _5 = const std::mem::drop(move _6) -> [return: bb19, unwind: bb4];
[01:12:17]     }
[01:12:17]     bb19: {                             
[01:12:17]         StorageDead(_6);
[01:12:17]         _1 = ();
[01:12:17]         StorageDead(_2);
[01:12:17]         goto -> bb1;
[01:12:17]     }
[01:12:17]     bb20: {                             
[01:12:17]         return;
[01:12:17] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:17] 
[01:12:17] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:12:17] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:12:17] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:17] Current block:  bb2: {
[01:12:17] Actual Line: "        falseEdges -> [real: bb8, imaginary: bb3];"
[01:12:17] Expected Line: "     falseEdges -> [real: bb9, imaginary: bb3]; //pre_binding1"
[01:12:17] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[01:12:17] ... (elided)
[01:12:17]  bb0: {
[01:12:17] ... (elided)
[01:12:17] ... (elided)
[01:12:17]      _2 = std::option::Option<i32>::Some(const 42i32,);
[01:12:17]      FakeRead(ForMatchedPlace, _2);
[01:12:17]      _3 = discriminant(_2);
[01:12:17]      switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:12:17]  bb1: {
[01:12:17]      resume;
[01:12:17]  }
[01:12:17]  bb2: {
[01:12:17]  bb2: {
[01:12:17]      falseEdges -> [real: bb9, imaginary: bb3]; //pre_binding1
[01:12:17]  bb3: {
[01:12:17]  bb3: {
[01:12:17]      falseEdges -> [real: bb12, imaginary: bb4]; //pre_binding2
[01:12:17]  bb4: {
[01:12:17]  bb4: {
[01:12:17]      falseEdges -> [real: bb13, imaginary: bb5]; //pre_binding3
[01:12:17]  bb5: {
[01:12:17]      unreachable;
[01:12:17]  }
[01:12:17]  }
[01:12:17]  bb6: { // to pre_binding2
[01:12:17]      falseEdges -> [real: bb3, imaginary: bb3];
[01:12:17]  bb7: {
[01:12:17]      unreachable;
[01:12:17]  }
[01:12:17]  bb8: {
[01:12:17]  bb8: {
[01:12:17] ... (elided)
[01:12:17]      return;
[01:12:17]  }
[01:12:17]  bb9: { // binding1 and guard
[01:12:17]      StorageLive(_6);
[01:12:17]      _6 = &(((promoted[1]: std::option::Option<i32>) as Some).0: i32);
[01:12:17]      _4 = &shallow (promoted[0]: std::option::Option<i32>);
[01:12:17]      StorageLive(_7);
[01:12:17]      _7 = const guard() -> [return: bb10, unwind: bb1];
[01:12:17]  bb10: {
[01:12:17]  bb10: {
[01:12:17]      FakeRead(ForMatchGuard, _4);
[01:12:17]      FakeRead(ForGuardBinding, _6);
[01:12:17]      switchInt(move _7) -> [false: bb6, otherwise: bb11];
[01:12:17]  bb11: {
[01:12:17]  bb11: {
[01:12:17]      StorageLive(_5);
[01:12:17]      _5 = ((_2 as Some).0: i32);
[01:12:17]      StorageLive(_8);
[01:12:17]      _8 = _5;
[01:12:17]      _1 = (const 1i32, move _8);
[01:12:17]      StorageDead(_8);
[01:12:17]      goto -> bb8;
[01:12:17]  bb12: {
[01:12:17]  bb12: {
[01:12:17]      StorageLive(_9);
[01:12:17]      _9 = ((_2 as Some).0: i32);
[01:12:17]      StorageLive(_10);
[01:12:17]      _10 = _9;
[01:12:17]      _1 = (const 2i32, move _10);
[01:12:17]      StorageDead(_10);
[01:12:17]      goto -> bb8;
[01:12:17]  bb13: {
[01:12:17]  bb13: {
[01:12:17]      _1 = (const 3i32, const 3i32);
[01:12:17]      goto -> bb8;
[01:12:17] Actual:
[01:12:17] Actual:
[01:12:17] fn full_tested_match() -> (){
[01:12:17]     let mut _0: ();
[01:12:17]     scope 1 {
[01:12:17]         let _9: i32;
[01:12:17]         scope 2 {
[01:12:17]     }
[01:12:17]     }
[01:12:17]     let mut _1: (i32, i32);
[01:12:17]     let mut _2: std::option::Option<i32>;
[01:12:17]     let mut _3: isize;
[01:12:17]     let mut _4: &std::option::Option<i32>;
[01:12:17]     let mut _7: bool;
[01:12:17]     let mut _8: i32;
[01:12:17]     let mut _10: i32;
[01:12:17]     bb0: {                              
[01:12:17]         StorageLive(_1);
[01:12:17]         StorageLive(_2);
[01:12:17]         _2 = std::option::Option<i32>::Some(const 42i32,);
[01:12:17]         FakeRead(ForMatchedPlace, _2);
[01:12:17]         _3 = discriminant(_2);
[01:12:17]         switchInt(move _3) -> [0isize: bb4, 1isize: bb2, otherwise: bb7];
[01:12:17]     bb1: {
[01:12:17]         resume;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb2: {                              
[01:12:17]         falseEdges -> [real: bb8, imaginary: bb3];
[01:12:17]     }
[01:12:17]     bb3: {                              
[01:12:17]         falseEdges -> [real: bb11, imaginary: bb4];
[01:12:17]     }
[01:12:17]     bb4: {                              
[01:12:17]         falseEdges -> [real: bb12, imaginary: bb5];
[01:12:17]     }
[01:12:17]     bb5: {                              
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb6: {                              
[01:12:17]         falseEdges -> [real: bb3, imaginary: bb3];
[01:12:17]     }
[01:12:17]     bb7: {                              
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb8: {                              
[01:12:17]         StorageLive(_6);
[01:12:17]         _6 = &(((promoted[1]: std::option::Option<i32>) as Some).0: i32);
[01:12:17]         _4 = &shallow (promoted[0]: std::option::Option<i32>);
[01:12:17]         StorageLive(_7);
[01:12:17]         _7 = const guard() -> [return: bb9, unwind: bb1];
[01:12:17]     }
[01:12:17]     bb9: {                              
[01:12:17]         FakeRead(ForMatchGuard, _4);
[01:12:17]         FakeRead(ForGuardBinding, _6);
[01:12:17]         switchInt(move _7) -> [false: bb6, otherwise: bb10];
[01:12:17]     }
[01:12:17]     bb10: {                             
[01:12:17]         StorageLive(_5);
[01:12:17]         _5 = ((_2 as Some).0: i32);
[01:12:17]         StorageLive(_8);
[01:12:17]         _8 = _5;
[01:12:17]         _1 = (const 1i32, move _8);
[01:12:17]         StorageDead(_8);
[01:12:17]         goto -> bb13;
[01:12:17]     }
[01:12:17]     bb11: {                             
[01:12:17]         StorageLive(_9);
[01:12:17]         _9 = ((_2 as Some).0: i32);
[01:12:17]         StorageLive(_10);
[01:12:17]         _10 = _9;
[01:12:17]         _1 = (const 2i32, move _10);
[01:12:17]         StorageDead(_10);
[01:12:17]         goto -> bb13;
[01:12:17]     }
[01:12:17]     bb12: {                             
[01:12:17]         _1 = (const 3i32, const 3i32);
[01:12:17]         goto -> bb13;
[01:12:17]     }
[01:12:17]     bb13: {                             
[01:12:17]         StorageDead(_9);
[01:12:17]         StorageDead(_5);
[01:12:17]         StorageDead(_7);
[01:12:17]         StorageDead(_6);
[01:12:17]         StorageDead(_1);
[01:12:17]         StorageDead(_2);
[01:12:17]         _0 = ();
[01:12:17]         return;
[01:12:17] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:17] 
[01:12:17] ---- [mir-opt] mir-opt/match_test.rs stdout ----
[01:12:17] ---- [mir-opt] mir-opt/match_test.rs stdout ----
[01:12:17] thread '[mir-opt] mir-opt/match_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:17] Current block:    bb1: {
[01:12:17] Actual Line: "        falseEdges -> [real: bb12, imaginary: bb2];"
[01:12:17] Expected Line: "       falseEdges -> [real: bb13, imaginary: bb2];"
[01:12:17] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:12:17] ... (elided)
[01:12:17]    bb0: {
[01:12:17] ... (elided)
[01:12:17] ... (elided)
[01:12:17]        switchInt(move _4) -> [false: bb7, otherwise: bb8];
[01:12:17]    bb1: {
[01:12:17]    bb1: {
[01:12:17]        falseEdges -> [real: bb13, imaginary: bb2];
[01:12:17]    bb2: {
[01:12:17]    bb2: {
[01:12:17]        falseEdges -> [real: bb14, imaginary: bb3];
[01:12:17]    bb3: {
[01:12:17]    bb3: {
[01:12:17]        falseEdges -> [real: bb15, imaginary: bb4];
[01:12:17]    bb4: {
[01:12:17]    bb4: {
[01:12:17]        falseEdges -> [real: bb16, imaginary: bb5];
[01:12:17]    bb5: {
[01:12:17]        unreachable;
[01:12:17]    }
[01:12:17]    bb6: {
[01:12:17]    bb6: {
[01:12:17]        falseEdges -> [real: bb4, imaginary: bb2];
[01:12:17]    bb7: {
[01:12:17]    bb7: {
[01:12:17]        _6 = Le(const 10i32, _1);
[01:12:17]        switchInt(move _6) -> [false: bb9, otherwise: bb10];
[01:12:17]    bb8: {
[01:12:17]    bb8: {
[01:12:17]        _5 = Lt(_1, const 10i32);
[01:12:17]        switchInt(move _5) -> [false: bb7, otherwise: bb1];
[01:12:17]    bb9: {
[01:12:17]    bb9: {
[01:12:17]        switchInt(_1) -> [-1i32: bb3, otherwise: bb4];
[01:12:17]    bb10: {
[01:12:17]    bb10: {
[01:12:17]        _7 = Le(_1, const 20i32);
[01:12:17]        switchInt(move _7) -> [false: bb9, otherwise: bb2];
[01:12:17]    bb11: {
[01:12:17]    bb11: {
[01:12:17]        StorageDead(_8);
[01:12:17]        _0 = ();
[01:12:17]        StorageDead(_2);
[01:12:17]        StorageDead(_1);
[01:12:17]        return;
[01:12:17]    bb12: {
[01:12:17]        _3 = const 0i32;
[01:12:17]        goto -> bb11;
[01:12:17]    }
[01:12:17]    }
[01:12:17]    bb13: {
[01:12:17]        StorageLive(_8);
[01:12:17]        _8 = _2;
[01:12:17]        switchInt(move _8) -> [false: bb6, otherwise: bb12];
[01:12:17]    bb14: {
[01:12:17]        _3 = const 1i32;
[01:12:17]        goto -> bb11;
[01:12:17]    }
---
[01:12:17]    bb16: {
[01:12:17]        _3 = const 3i32;
[01:12:17]        goto -> bb11;
[01:12:17]    }
[01:12:17] Actual:
[01:12:17] fn main() -> (){
[01:12:17]     let mut _0: ();
[01:12:17]     scope 1 {
[01:12:17]         scope 3 {
[01:12:17]         scope 4 {
[01:12:17]         scope 4 {
[01:12:17]             let _2: bool;
[01:12:17]     }
[01:12:17]     scope 2 {
[01:12:17]         let _1: i32;
[01:12:17]     }
[01:12:17]     }
[01:12:17]     let mut _3: i32;
[01:12:17]     let mut _4: bool;
[01:12:17]     let mut _5: bool;
[01:12:17]     let mut _6: bool;
[01:12:17]     let mut _7: bool;
[01:12:17]     let mut _8: bool;
[01:12:17]     bb0: {                              
[01:12:17]         StorageLive(_1);
[01:12:17]         _1 = const 3i32;
[01:12:17]         FakeRead(ForLet, _1);
[01:12:17]         StorageLive(_2);
[01:12:17]         _2 = const true;
[01:12:17]         FakeRead(ForLet, _2);
[01:12:17]         FakeRead(ForMatchedPlace, _1);
[01:12:17]         _4 = Le(const 0i32, _1);
[01:12:17]         switchInt(move _4) -> [false: bb7, otherwise: bb8];
[01:12:17]     }
[01:12:17]     bb1: {                              
[01:12:17]         falseEdges -> [real: bb12, imaginary: bb2];
[01:12:17]     }
[01:12:17]     bb2: {                              
[01:12:17]         falseEdges -> [real: bb13, imaginary: bb3];
[01:12:17]     }
[01:12:17]     bb3: {                              
[01:12:17]         falseEdges -> [real: bb14, imaginary: bb4];
[01:12:17]     }
[01:12:17]     bb4: {                              
[01:12:17]         falseEdges -> [real: bb15, imaginary: bb5];
[01:12:17]     }
[01:12:17]     bb5: {                              
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb6: {                              
[01:12:17]         falseEdges -> [real: bb4, imaginary: bb2];
[01:12:17]     }
[01:12:17]     bb7: {                              
[01:12:17]         _6 = Le(const 10i32, _1);
[01:12:17]         switchInt(move _6) -> [false: bb9, otherwise: bb10];
[01:12:17]     }
[01:12:17]     bb8: {                              
[01:12:17]         _5 = Lt(_1, const 10i32);
[01:12:17]         switchInt(move _5) -> [false: bb7, otherwise: bb1];
[01:12:17]     }
[01:12:17]     bb9: {                              
[01:12:17]         switchInt(_1) -> [-1i32: bb3, otherwise: bb4];
[01:12:17]     }
[01:12:17]     bb10: {                             
[01:12:17]         _7 = Le(_1, const 20i32);
[01:12:17]         switchInt(move _7) -> [false: bb9, otherwise: bb2];
[01:12:17]     }
[01:12:17]     bb11: {                             
[01:12:17]         _3 = const 0i32;
[01:12:17]         goto -> bb16;
[01:12:17]     }
[01:12:17]     bb12: {                             
[01:12:17]         StorageLive(_8);
[01:12:17]         _8 = _2;
[01:12:17]         switchInt(move _8) -> [false: bb6, otherwise: bb11];
[01:12:17]     }
[01:12:17]     bb13: {                             
[01:12:17]         _3 = const 1i32;
[01:12:17]         goto -> bb16;
[01:12:17]     }
[01:12:17]     bb14: {                             
[01:12:17]         _3 = const 2i32;
[01:12:17]         goto -> bb16;
[01:12:17]     }
[01:12:17]     bb15: {                             
[01:12:17]         _3 = const 3i32;
[01:12:17]         goto -> bb16;
[01:12:17]     }
[01:12:17]     bb16: {                             
[01:12:17]         StorageDead(_8);
[01:12:17]         _0 = ();
[01:12:17]         StorageDead(_2);
[01:12:17]         StorageDead(_1);
[01:12:17]         return;
[01:12:17] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:17] 
[01:12:17] ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
[01:12:17] ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
[01:12:17] thread '[mir-opt] mir-opt/remove_fake_borrows.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:17] Current block: bb1: {
[01:12:17] Actual Line: "        goto -> bb7;"
[01:12:17] Expected Line: "    goto -> bb8;"
[01:12:17] Test Name: rustc.match_guard.CleanupNonCodegenStatements.before.mir
[01:12:17] ... (elided)
[01:12:17] bb0: {
[01:12:17] bb0: {
[01:12:17]     FakeRead(ForMatchedPlace, _1);
[01:12:17]     _3 = discriminant(_1);
[01:12:17]     switchInt(move _3) -> [1isize: bb5, otherwise: bb2];
[01:12:17] }
[01:12:17] bb1: {
[01:12:17]     goto -> bb8;
[01:12:17] }
[01:12:17] bb2: {
[01:12:17]     goto -> bb9;
[01:12:17] }
[01:12:17] bb3: {
[01:12:17] }
[01:12:17] bb4: {
[01:12:17]     goto -> bb2;
[01:12:17] }
[01:12:17] }
[01:12:17] bb5: {
[01:12:17]     switchInt((*(*((_1 as Some).0: &'<empty> &'<empty> i32)))) -> [0i32: bb1, otherwise: bb2];
[01:12:17] }
[01:12:17] bb6: {
[01:12:17]     StorageDead(_8);
[01:12:17]     return;
[01:12:17] }
[01:12:17] bb7: {
[01:12:17]     _0 = const 0i32;
[01:12:17]     goto -> bb6;
[01:12:17] }
[01:12:17] bb8: {
[01:12:17]     _4 = &shallow _1;
[01:12:17]     _5 = &shallow ((_1 as Some).0: &'<empty> &'<empty> i32);
[01:12:17]     _6 = &shallow (*((_1 as Some).0: &'<empty> &'<empty> i32));
[01:12:17]     _7 = &shallow (*(*((_1 as Some).0: &'<empty> &'<empty> i32)));
[01:12:17]     StorageLive(_8);
[01:12:17]     _8 = _2;
[01:12:17]     FakeRead(ForMatchGuard, _4);
[01:12:17]     FakeRead(ForMatchGuard, _5);
[01:12:17]     FakeRead(ForMatchGuard, _6);
[01:12:17]     FakeRead(ForMatchGuard, _7);
[01:12:17]     switchInt(move _8) -> [false: bb4, otherwise: bb7];
[01:12:17] }
[01:12:17] bb9: {
[01:12:17]     _0 = const 1i32;
[01:12:17]     goto -> bb6;
[01:12:17] }
[01:12:17] bb10: {
[01:12:17]     resume;
[01:12:17] }
[01:12:17] Actual:
[01:12:17] fn match_guard(_1: std::option::Option<&&i32>, _2: bool) -> i32{
[01:12:17]     let mut _0: i32;
[01:12:17]     let mut _3: isize;
[01:12:17]     let mut _4: &std::option::Option<&&i32>;
[01:12:17]     let mut _5: &&'<empty> &'<empty> i32;
[01:12:17]     let mut _6: &&'<empty> i32;
[01:12:17]     let mut _7: &i32;
[01:12:17]     let mut _8: bool;
[01:12:17]     bb0: {                              
[01:12:17]         FakeRead(ForMatchedPlace, _1);
[01:12:17]         _3 = discriminant(_1);
[01:12:17]         switchInt(move _3) -> [1isize: bb5, otherwise: bb2];
[01:12:17]     }
[01:12:17]     bb1: {                              
[01:12:17]         goto -> bb7;
[01:12:17]     }
[01:12:17]     bb2: {                              
[01:12:17]         goto -> bb8;
[01:12:17]     }
[01:12:17]     bb3: {                              
[01:12:17]     }
[01:12:17]     }
[01:12:17]     bb4: {                              
[01:12:17]         goto -> bb2;
[01:12:17]     }
[01:12:17]     bb5: {                              
[01:12:17]         switchInt((*(*((_1 as Some).0: &'<empty> &'<empty> i32)))) -> [0i32: bb1, otherwise: bb2];
[01:12:17]     }
[01:12:17]     bb6: {                              
[01:12:17]         _0 = const 0i32;
[01:12:17]         goto -> bb9;
[01:12:17]     }
[01:12:17]     bb7: {                              
[01:12:17]         _4 = &shallow _1;
[01:12:17]         _5 = &shallow ((_1 as Some).0: &'<empty> &'<empty> i32);
[01:12:17]         _6 = &shallow (*((_1 as Some).0: &'<empty> &'<empty> i32));
[01:12:17]         _7 = &shallow (*(*((_1 as Some).0: &'<empty> &'<empty> i32)));
[01:12:17]         StorageLive(_8);
[01:12:17]         _8 = _2;
[01:12:17]         FakeRead(ForMatchGuard, _4);
[01:12:17]         FakeRead(ForMatchGuard, _5);
[01:12:17]         FakeRead(ForMatchGuard, _6);
[01:12:17]         FakeRead(ForMatchGuard, _7);
[01:12:17]         switchInt(move _8) -> [false: bb4, otherwise: bb6];
[01:12:17]     }
[01:12:17]     bb8: {                              
[01:12:17]         _0 = const 1i32;
[01:12:17]         goto -> bb9;
[01:12:17]     }
[01:12:17]     bb9: {                              
[01:12:17]         StorageDead(_8);
[01:12:17]         return;
[01:12:17]     bb10: {
[01:12:17]         resume;
[01:12:17]     }
[01:12:17] }', src/tools/compiletest/src/runtest.rs:2960:13
---
[01:12:17] test result: FAILED. 34 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:17] 
[01:12:17] 
[01:12:17] 
[01:12:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:17] 
[01:12:17] 
[01:12:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:17] Build completed unsuccessfully in 0:11:44
[01:12:17] Build completed unsuccessfully in 0:11:44
[01:12:17] Makefile:48: recipe for target 'check' failed
[01:12:17] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20f6ce0b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 22:10:47 UTC 2019
