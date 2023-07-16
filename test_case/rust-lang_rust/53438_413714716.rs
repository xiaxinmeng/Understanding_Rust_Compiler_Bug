plain
[00:48:03] ....................................................................................................
[00:48:07] ....................................................................................................
[00:48:09] i...................................................................................................
[00:48:13] ....................................................................................................
[00:48:15] .................................................iiiiiiiii..........................................
[00:48:21] ....................................................................................................
[00:48:25] ....................................................................................................
[00:48:28] .............................i......................................................................
[00:48:31] ................................i...............................................i.i..ii.............
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:18] 
[00:55:18] running 46 tests
[00:55:24] ERROR 2018-08-16T23:32:04Z: compiletest::runtest: None
[00:55:26] ERROR 2018-08-16T23:32:07Z: compiletest::runtest: None
[00:55:36] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:55:36] ..........................F.F.................
[00:55:36] 
[00:55:36] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[00:55:36] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[00:55:36] thread '[mir-opt] mir-opt/issue-49232.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:36] Current block: None
[00:55:36] Actual Line: "    let mut _4: &\'<empty> bool;"
[00:55:36] Expected Line: "    let mut _4: u8;"
[00:55:36] Test Name: rustc.main.mir_map.0.mir
[00:55:36] ... (elided)
[00:55:36] fn main() -> (){
[00:55:36] fn main() -> (){
[00:55:36]     let mut _0: ();
[00:55:36]     scope 1 {
[00:55:36]     scope 2 {
[00:55:36]         let _2: i32;
[00:55:36]     }
[00:55:36]     }
[00:55:36]     let mut _1: ();
[00:55:36]     let mut _3: bool;
[00:55:36]     let mut _4: u8;
[00:55:36]     let mut _5: !;
[00:55:36]     let mut _6: ();
[00:55:36]     let mut _7: &i32;
[00:55:36]     bb0: {
[00:55:36]         goto -> bb1;
[00:55:36]     bb1: {
[00:55:36]     bb1: {
[00:55:36]         falseUnwind -> [real: bb3, cleanup: bb4];
[00:55:36]     bb2: {
[00:55:36]         goto -> bb29;
[00:55:36]     }
[00:55:36]     bb3: {
[00:55:36]     bb3: {
[00:55:36]         StorageLive(_2);
[00:55:36]         StorageLive(_3);
[00:55:36]         _3 = const true;
[00:55:36]         _4 = discriminant(_3);
[00:55:36]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[00:55:36]     bb4: {
[00:55:36]         resume;
[00:55:36]     }
[00:55:36]     bb5: {
[00:55:36]     bb5: {
[00:55:36]         _2 = const 4i32;
[00:55:36]         goto -> bb14;
[00:55:36]     }
[00:55:36]     bb6: {
[00:55:36]         _0 = ();
[00:55:36]         goto -> bb15;
[00:55:36]     }
[00:55:36]     bb7: {
[00:55:36]         falseEdges -> [real: bb12, imaginary: bb8];
[00:55:36]     bb8: {
[00:55:36]     bb8: {
[00:55:36]         falseEdges -> [real: bb13, imaginary: bb9];
[00:55:36]     bb9: {
[00:55:36]         unreachable;
[00:55:36]     }
[00:55:36]     bb10: {
---
[00:55:36]     }
[00:55:36]     bb13: {
[00:55:36]         goto -> bb6;
[00:55:36]     }
[00:55:36]     bb14: {
[00:55:36]         StorageDead(_3);
[00:55:36]         StorageLive(_7);
[00:55:36]         _7 = &_2;
[00:55:36]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[00:55:36]     bb15: {
[00:55:36]         goto -> bb16;
[00:55:36]     }
[00:55:36]     bb16: {
[00:55:36]     bb16: {
[00:55:36]         goto -> bb17;
[00:55:36]     }
[00:55:36]     bb17: {
[00:55:36]         goe;
[00:55:36]     }
[00:55:36]     bb27: {                             
[00:55:36]         StorageDead(_5);
[00:55:36]         goto -> bb14;
[00:55:36]     }
[00:55:36]     bb28: {                             
[00:55:36]         StorageDead(_7);
[00:55:36]         _1 = ();
[00:55:36]         StorageDead(_2);
[00:55:36]         goto -> bb1;
[00:55:36]     }
[00:55:36]     bb29: {                             
[00:55:36]         return;
[00:55:36] }', tools/compiletest/src/runtest.rs:2828:13
[00:55:36] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:36] 
[00:55:36] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:55:36] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:55:36] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:36] Current block: None
[00:55:36] Actual Line: "        _3 = &(promoted[1]: std::option::Option<i32>);"
[00:55:36] Expected Line: "     _3 = discriminant(_2);"
[00:55:36] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[00:55:36] ... (elided)
[00:55:36]  bb0: {
[00:55:36] ... (elided)
[00:55:36] ... (elided)
[00:55:36]      _2 = std::option::Option<i32>::Some(const 42i32,);
[00:55:36]      _3 = discriminant(_2);
[00:55:36]      _4 = &(promoted[1]: std::option::Option<i32>);
[00:55:36]      _9 = discriminant(_2);
[00:55:36]      switchInt(move _9) -> [0isize: bb5, 1isize: bb3, otherwise: bb7];
[00:55:36]  bb1: {
[00:55:36]      resume;
[00:55:36]  }
[00:55:36]  }
[00:55:36]  bb2: {  // arm1
[00:55:36]      _1 = (const 3i32, const 3i32);
[00:55:36]      goto -> bb13;
[00:55:36]  }
[00:55:36]  bb3: { // binding3(empty) and arm3
[00:55:36]      ReadForMatch(_4);
[00:55:36]      falseEdges -> [real: bb8, imaginary: bb4]; //pre_binding1
[00:55:36]  bb4: {
[00:55:36]      ReadForMatch(_4);
[00:55:36]      ReadForMatch(_4);
[00:55:36]      falseEdges -> [real: bb12, imaginary: bb5]; //pre_binding2
[00:55:36]  bb5: {
[00:55:36]      ReadForMatch(_4);
[00:55:36]      ReadForMatch(_4);
[00:55:36]      falseEdges -> [real: bb2, imaginary: bb6]; //pre_binding3
[00:55:36]  bb6: {
[00:55:36]      unreachable;
[00:55:36]  }
[00:55:36]  bb7: {
[00:55:36]  bb7: {
[00:55:36]      unreachable;
[00:55:36]  }
[00:55:36]  bb8: { // binding1 and guard
[00:55:36]      StorageLive(_7);
[00:55:36]      _7 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[00:55:36]      StorageLive(_10);
[00:55:36]      _10 = const guard() -> [return: bb9, unwind: bb1];
[00:55:36]  bb9: {
[00:55:36]  bb9: {
[00:55:36]      switchInt(move _10) -> [false: bb10, otherwise: bb11];
[00:55:36]  }
[00:55:36]  bb10: { // to pre_binding2
[00:55:36]      falseEdges -> [real: bb4, imaginary: bb4];
[00:55:36]  }
[00:55:36]  bb11: { // bindingNoLandingPads.before.mir2 and arm2
[00:55:36]      StorageLive(_5);
[00:55:36]      _5 = ((_2 as Some).0: i32);
[00:55:36]      StorageLive(_11);
[00:55:36]      _11 = _5;
[00:55:36]      _1 = (const 1i32, move _11);
[00:55:36]      StorageDead(_11);
[00:55:36]      goto -> bb13;
[00:55:36]  bb12: {
[00:55:36]  bb12: {
[00:55:36]      StorageLive(_8);
[00:55:36]      _8 = ((_2 as Some).0: i32);
[00:55:36]      StorageLive(_12);
[00:55:36]      _12 = _8;
[00:55:36]      _1 = (const 2i32, move_12);
[00:55:36]      StorageDead(_12);
[00:55:36]      goto -> bb13;
[00:55:36]  bb13: {
[00:55:36] ... (elided)
[00:55:36]      return;
[00:55:36]  }
[00:55:36]  }
[00:55:36] Actual:
[00:55:36] fn full_tested_match() -> (){
[00:55:36]     let mut _0: ();
[00:55:36]     scope 1 {
[00:55:36]     scope 2 {
[00:55:36]     }
[00:55:36]     }
[00:55:36]     let mut _1: (i32, i32);
[00:55:36]     let mut _2: std::option::Option<i32>;
[00:55:36]     let mut _3: &'<empty> std::option::Option<i32>;
[00:55:36]     let mut _8: isize;
[00:55:36]     let mut _9: bool;
[00:55:36]     let mut _10: i32;
[00:55:36]     let mut _11: i32;
[00:55:36]     bb0: {                              
[00:55:36]         StorageLive(_1);
[00:55:36]         StorageLive(_2);
[00:55:36]         _2 = std::option::Option<i32>::Some(const 42i32,);
[00:55:36]         _3 = &(promoted[1]: std::option::Option<i32>);
[00:55:36]         _8 = discriminant(_2);
[00:55:36]         switchInt(move _8) -> [0isize: bb5, 1isize: bb3, otherwise: bb7];
[00:55:36]     bb1: {
[00:55:36]         resume;
[00:55:36]     }
[00:55:36]     }
[00:55:36]     bb2: {                              
[00:55:36]         _1 = (const 3i32, const 3i32);
[00:55:36]         goto -> bb13;
[00:55:36]     }
[00:55:36]     bb3: {                              
[00:55:36]         ReadForMatch(_3);
[00:55:36]         falseEdges -> [real: bb8, imaginary: bb4];
[00:55:36]     }
[00:55:36]     bb4: {                              
[00:55:36]         ReadForMatch(_3);
[00:55:36]         falseEdges -> [real: bb12, imaginary: bb5];
[00:55:36]     }
[00:55:36]     bb5: {                              
[00:55:36Thu, 16 Aug 2018 23:32:17 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
