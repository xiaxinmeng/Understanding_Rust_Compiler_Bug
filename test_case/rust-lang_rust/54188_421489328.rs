plain
[00:54:06] ....................................................................................................
[00:54:09] ....................................................i...............................................
[00:54:11] ....................................................................................................
[00:54:14] ....................................................................................................
[00:54:17] iiiiiiiii...........................................................................................
[00:54:23] ....................................................................................................
[00:54:26] .................................................................................i..................
[00:54:29] ....................................................................................................
[00:54:32] ..................................i.i..ii...........................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:12] 
[01:01:12] running 45 tests
[01:01:13] ERROR 2018-09-14T21:28:21Z: compiletest::runtest: None
[01:01:13] ERROR 2018-09-14T21:28:21Z: compiletest::runtest: Some("    bb4: {")
[01:01:14] ERROR 2018-09-14T21:28:22Z: compiletest::runtest: None
[01:01:14] ERROR 2018-09-14T21:28:23Z: compiletest::runtest: Some("    bb2: {")
[01:01:14] ERROR 2018-09-14T21:28:23Z: compiletest::runtest: Some("    bb2: {")
[01:01:16] ERROR 2018-09-14T21:28:24Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:16] ERROR 2018-09-14T21:28:24Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:16] ERROR 2018-09-14T21:28:25Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:16] ERROR 2018-09-14T21:28:25Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:17] ERROR 2018-09-14T21:28:25Z: compiletest::runtest: Some("    bb0: {")
[01:01:17] ERROR 2018-09-14T21:28:25Z: compiletest::runtest: Some("    bb4: {")
[01:01:18] ERROR 2018-09-14T21:28:26Z: compiletest::runtest: Some("       _1 = D::{{constructor}}(const 0i32,);")
[01:01:18] ERROR 2018-09-14T21:28:26Z: compiletest::runtest: None
[01:01:18] ERROR 2018-09-14T21:28:26Z: compiletest::runtest: None
[01:01:19] ERROR 2018-09-14T21:28:27Z: compiletest::runtest: Some("    bb14: {")
[01:01:19] ERROR 2018-09-14T21:28:27Z: compiletest::runtest: Some("   bb5: { // The loop body (body_block)")
[01:01:21] ERROR 2018-09-14T21:28:29Z: compiletest::runtest: Some(" bb3: { // binding3(empty) and arm3")
[01:01:23] ERROR 2018-09-14T21:28:32Z: compiletest::runtest: None
[01:01:25] ERROR 2018-09-14T21:28:33Z: compiletest::runtest: Some("        Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);")
[01:01:25] ERROR 2018-09-14T21:28:34Z: compiletest::runtest: None
[01:01:26] ERROR 2018-09-14T21:28:34Z: compiletest::runtest: Some("    bb0: {")
[01:01:31] .FF...F.FF..FFFFFFFF.F....FFF.F....F.FFF.....
[01:01:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block: None
[01:01:31] Actual Line: "        FakeRead(ForLet, _1);"
[01:01:31] Expected Line: "       StorageLive(_2);"
[01:01:31] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:01:31] ... (elided)
[01:01:31]    bb0: {
[01:01:31]    bb0: {
[01:01:31]        StorageLive(_1);
[01:01:31]        _1 = const false;
[01:01:31]        StorageLive(_2);
[01:01:31]        StorageLive(_3);
[01:01:31]        _3 = _1;
[01:01:31]        _2 = move _3;
[01:01:31]        StorageDead(_3);
[01:01:31]        StorageLive(_4);
[01:01:31]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:01:31]        AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:01:31]        StorageLive(_5);
[01:01:31]        StorageLive(_6);
[01:01:31]        _6 = move _4;
[01:01:31]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:01:31] ... (elided)
[01:01:31]    bb2: {
[01:01:31]    bb2: {
[01:01:31]        drop(_6) -> [return: bb6, unwind: bb4];
[01:01:31] ... (elided)
[01:01:31]    bb5: {
[01:01:31]        drop(_6) -> bb4;
[01:01:31]    }
[01:01:31]    }
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]         scope 3 {
[01:01:31]             scope 5 {
[01:01:31]                 scope 7 {
[01:01:31]                 scope 8 {
[01:01:31]                 scope 8 {
[01:01:31]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:01:31]             }
[01:01:31]             scope 6 {
[01:01:31]             scope 6 {
[01:01:31]                 let _4: std::option::Option<std::boxed::Box<u32>> as Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> };
[01:01:31]         }
[01:01:31]         scope 4 {
[01:01:31]         scope 4 {
[01:01:31]             let _2: bool;
[01:01:31]     }
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
[01:01:31]         let _1: bool;
[01:01:31]     }
[01:01:31]     let mut _3: bool;
[01:01:31]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:01:31]     bb0: {                              
[01:01:31]         StorageLive(_1);
[01:01:31]         _1 = const false;
[01:01:31]         FakeRead(ForLet, _1);
[01:01:31]         StorageLive(_2);
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = _1;
[01:01:31]         _2 = move _3;
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageLive(_4);
[01:01:31]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:01:31]         AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:01:31]         FakeRead(ForLet, _4);
[01:01:31]         StorageLive(_5);
[01:01:31]         StorageLive(_6);
[01:01:31]         _6 = move _4;
[01:01:31]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         drop(_6) -> [return: bb6, unwind: bb4];
[01:01:31]     bb3: {
[01:01:31]         drop(_4) -> bb1;
[01:01:31]     }
[01:01:31]     bb4: {
[01:01:31]     bb4: {
[01:01:31]         drop(_5) -> bb3;
[01:01:31]     }
[01:01:31]     bb5: {
[01:01:31]         drop(_6) -> bb4;
[01:01:31]     }
[01:01:31]     bb6: {                              
[01:01:31]         StorageDead(_6);
[01:01:31]         _0 = ();
[01:01:31]         drop(_5) -> [return: bb7, unwind: bb3];
[01:01:31]     }
[01:01:31]     bb7: {                              
[01:01:31]         StorageDead(_5);
[01:01:31]         drop(_4) -> [return: bb8, unwind: bb1];
[01:01:31]     }
[01:01:31]     bb8: {                              
[01:01:31]         StorageDead(_4);
[01:01:31]         StorageDead(_2);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block:     bb4: {
[01:01:31] Actual Line: "        FakeRead(ForLet, _1);"
[01:01:31] Expected Line: "        StorageLive(_4);"
[01:01:31] Test Name: rustc.main.ElaborateDrops.before.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
[01:01:31]         let _1: std::boxed::Box<S>;
[01:01:31]     }
[01:01:31]     let mut _2: std::boxed::Box<S>;
[01:01:31]     let mut _3: ();
[01:01:31]     let mut _4: std::boxed::Box<S>;
[01:01:31]     bb0: {
[01:01:31]         StorageLive(_1);
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = Box(S);
[01:01:31]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     bb2: {
[01:01:31]     bb2: {
[01:01:31]         _1 = move _2;
[01:01:31]         drop(_2) -> bb4;
[01:01:31]     }
[01:01:31]     bb3: {
[01:01:31]         drop(_2) -> bb1;
[01:01:31]     }
[01:01:31]     bb4: {
[01:01:31]         StorageDead(_2);
[01:01:31]         StorageLive(_4);
[01:01:31]         _4 = move _1;
[01:01:31]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:01:31]     bb5: {
[01:01:31]     bb5: {
[01:01:31]         drop(_4) -> [return: bb8, unwind: bb6];
[01:01:31]     bb6: {
[01:01:31]         drop(_1) -> bb1;
[01:01:31]     }
[01:01:31]     bb7: {
[01:01:31]     bb7: {
[01:01:31]         drop(_4) -> bb6;
[01:01:31]     }
[01:01:31]     bb8: {
[01:01:31]         StorageDead(_4);
[01:01:31]         _0 = ();
[01:01:31]         drop(_1) -> bb9;
[01:01:31]     bb9: {
[01:01:31]     bb9: {
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] }
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
[01:01:31]         let _1: std::boxed::Box<S>;
[01:01:31]     }
[01:01:31]     let mut _2: std::boxed::Box<S>;
[01:01:31]     let mut _3: ();
[01:01:31]     let mut _4: std::boxed::Box<S>;
[01:01:31]     bb0: {                              
[01:01:31]         StorageLive(_1);
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = Box(S);
[01:01:31]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         _1 = move _2;
[01:01:31]         drop(_2) -> bb4;
[01:01:31]     bb3: {
[01:01:31]         drop(_2) -> bb1;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb4: {                              
[01:01:31]         StorageDead(_2);
[01:01:31]         FakeRead(ForLet, _1);
[01:01:31]         StorageLive(_4);
[01:01:31]         _4 = move _1;
[01:01:31]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         drop(_4) -> [return: bb8, unwind: bb6];
[01:01:31]     bb6: {
[01:01:31]         drop(_1) -> bb1;
[01:01:31]     }
[01:01:31]     bb7: {
[01:01:31]     bb7: {
[01:01:31]         drop(_4) -> bb6;
[01:01:31]     }
[01:01:31]     bb8: {                              
[01:01:31]         StorageDead(_4);
[01:01:31]         _0 = ();
[01:01:31]         drop(_1) -> bb9;
[01:01:31]     }
[01:01:31]     bb9: {                              
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/end_region_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block: None
[01:01:31] Actual Line: "        FakeRead(ForLet, _1);"
[01:01:31] Expected Line: "        StorageLive(_2);"
[01:01:31] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]     let mut _0: ();
[01:01:31] ... (elided)
[01:01:31]     let _2: &'10_1rs i32;
[01:01:31] ... (elided)
[01:01:31]     let _1: i32;
[01:01:31] ... (elided)
[01:01:31]     bb0: {
[01:01:31]         StorageLive(_1);
[01:01:31]         _1 = const 3i32;
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = &'10_1rs _1;
[01:01:31]         _0 = ();
[01:01:31]         EndRegion('10_1rs);
[01:01:31]         StorageDead(_2);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]         scope 3 {
[01:01:31]         scope 4 {
[01:01:31]         scope 4 {
[01:01:31]             let _2: &'10_1rs i32;
[01:01:31]     }
[01:01:31]     scope 2 {
[01:01:31]         let _1: i32;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb0: {                              
[01:01:31]         StorageLive(_1);
[01:01:31]         _1 = const 3i32;
[01:01:31]         FakeRead(ForLet, _1);
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = &'10_1rs _1;
[01:01:31]         FakeRead(ForLet, _2);
[01:01:31]         _0 = ();
[01:01:31]         EndRegion('10_1rs);
[01:01:31]         StorageDead(_2);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/end_region_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block:     bb2: {
[01:01:31] Actual Line: "        FakeRead(ForLet, _2);"
[01:01:31] Expected Line: "        StorageLive(_3);"
[01:01:31] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]     let mut _0: ();
[01:01:31] ... (elided)
[01:01:31]     let _7: &'23_3rs bool;
[01:01:31] ... (elided)
[01:01:31]     let _3: &'23_1rs bool;
[01:01:31] ... (elided)
[01:01:31]     let _2: bool;
[01:01:31] ... (elided)
[01:01:31]     let mut _4: ();
[01:01:31]     let mut _5: bool;
[01:01:31] ... (elided)
[01:01:31]     bb0: {
[01:01:31]         goto -> bb1;
[01:01:31]     bb1: {
[01:01:31]     bb1: {
[01:01:31]          falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:31]     bb2: {
[01:01:31]     bb2: {
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = const true;
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = &'23_1rs _2;
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = _2;
[01:01:31]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:31]     bb3: {
[01:01:31] ... (elided)
[01:01:31]     }
[01:01:31]     bb4: {
[01:01:31]     bb4: {
[01:01:31]         _0 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         EndRegion('23_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageDead(_2);
[01:01:31]         return;
[01:01:31]     bb5: {
[01:01:31]         _4 = ();
[01:01:31]         _4 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &'23_3rs _2;
[01:01:31]         _1 = ();
[01:01:31]         EndRegion('23_3rs);
[01:01:31]         StorageDead(_7);
[01:01:31]         EndRegion('23_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb1;
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]         scope 3 {
[01:01:31]             scope 5 {
[01:01:31]             scope 6 {
[01:01:31]             scope 6 {
[01:01:31]                 let _7: &'23_3rs bool;
[01:01:31]         }
[01:01:31]         scope 4 {
[01:01:31]         scope 4 {
[01:01:31]             let _3: &'23_1rs bool;
[01:01:31]     }
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
[01:01:31]         let _2: bool;
[01:01:31]     }
[01:01:31]     let mut _1: ();
[01:01:31]     let mut _4: ();
[01:01:31]     let mut _5: bool;
[01:01:31]     let mut _6: !;
[01:01:31]     bb0: {                              
[01:01:31]         goto -> bb1;
[01:01:31]     }
[01:01:31]     bb1: {                              
[01:01:31]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = const true;
[01:01:31]         FakeRead(ForLet, _2);
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = &'23_1rs _2;
[01:01:31]         FakeRead(ForLet, _3);
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = _2;
[01:01:31]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:31]     bb3: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb4: {                              
[01:01:31]         _0 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         EndRegion('23_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageDead(_2);
[01:01:31]         return;
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         _4 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &'23_3rs _2;
[01:01:31]         FakeRead(ForLet, _7);
[01:01:31]         _1 = ();
[01:01:31]         EndRegion('23_3rs);
[01:01:31]         StorageDead(_7);
[01:01:31]         EndRegion('23_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb1;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/end_region_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block:     bb2: {
[01:01:31] Actual Line: "        FakeRead(ForLet, _3);"
[01:01:31] Expected Line: "        StorageLive(_5);"
[01:01:31] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]     let mut _0: ();
[01:01:31] ... (elided)
[01:01:31]     let _7: &'26_3rs bool;
[01:01:31] ... (elided)
[01:01:31]     let _3: &'26_1rs bool;
[01:01:31] ... (elided)
[01:01:31]     let mut _1: bool;
[01:01:31] ... (elided)
[01:01:31]     let mut _2: ();
[01:01:31]     let mut _4: ();
[01:01:31]     let mut _5: bool;
[01:01:31]     let mut _6: !;
[01:01:31]     bb0: {
[01:01:31]         StorageLive(_1);
[01:01:31]         goto -> bb1;
[01:01:31]     bb1: {
[01:01:31]     bb1: {
[01:01:31]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:31]     bb2: {
[01:01:31]         _1 = const true;
[01:01:31]         _1 = const true;
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = &'26_1rs _1;
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = _1;
[01:01:31]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:31]     bb3: {
[01:01:31] ... (elided)
[01:01:31]     }
[01:01:31]     bb4: {
[01:01:31]     bb4: {
[01:01:31]         _0 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         EndRegion('26_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31]     bb5: {
[01:01:31]         _4 = ();
[01:01:31]         _4 = ();
[01:01:31]         StorageDead(_5);
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &'26_3rs _1;
[01:01:31]         _2 = ();
[01:01:31]         EndRegion('26_3rs);
[01:01:31]         StorageDead(_7);
[01:01:31]         EndRegion('26_1rs);
[01:01:31]         StorageDead(_3);
[01:01:31]         goto -> bb1;
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]         scope 3 {
[01:01:31]             scope 5 {
[01:01:31]             scope 6 {
[01:01:31]             scope 6 {
[01:01:31]                 let _7: &'26_3rs bool;
[01:01:31]         }
[01:01:31]         scope 4 {
[01:01:31]         scope 4 {
[01:01:31]             let _3: &'26_1rs bool;
[01:01:31]     }
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
---
[01:01:31]     }
[01:01:31]     bb13: {
[01:01:31]         goto -> bb6;
[01:01:31]     }
[01:01:31]     bb14: {
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &_2;
[01:01:31]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:01:31]     bb15: {
[01:01:31]         goto -> bb16;
[01:01:31]     }
[01:01:31]     bb16: {
---
[01:01:31]     }
[01:01:31]     bb19: {
[01:01:31]         goto -> bb20;
[01:01:31]     }
[01:01:31]     bb20: {
[01:01:31]         StorageDead(_3);
[01:01:31]         goto -> bb21;
[01:01:31]     bb21: {
[01:01:31]         goto -> bb22;
[01:01:31]     }
[01:01:31]     bb22: {
[01:01:31]     bb22: {
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb23;
[01:01:31]     bb23: {
[01:01:31]         goto -> bb24;
[01:01:31]     }
[01:01:31]     bb24: {
---
[01:01:31]     bb26: {
[01:01:31]         _5 = ();
[01:01:31]         unreachable;
[01:01:31]     }
[01:01:31]     bb27: {
[01:01:31]         StorageDead(_5);
[01:01:31]         goto -> bb14;
[01:01:31]     bb28: {
[01:01:31]     bb28: {
[01:01:31]         StorageDead(_7);
[01:01:31]         _1 = ();
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb1;
[01:01:31]     bb29: {
[01:01:31]         return;
[01:01:31]     }
[01:01:31] }
[01:01:31] }
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]     scope 2 {
[01:01:31]         let _2: i32;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     let mut _1: ();
[01:01:31]     let mut _3: bool;
[01:01:31]     let mut _4: u8;
[01:01:31]     let mut _5: !;
[01:01:31]     let mut _6: ();
[01:01:31]     let mut _7: &i32;
[01:01:31]     bb0: {                              
[01:01:31]         goto -> bb1;
[01:01:31]     }
[01:01:31]     bb1: {                              
[01:01:31]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         goto -> bb29;
[01:01:31]     }
[01:01:31]     bb3: {                              
[01:01:31]         StorageLive(_2);
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = const true;
[01:01:31]         _4 = discriminant(_3);
[01:01:31]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[01:01:31]     bb4: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         _2 = const 4i32;
[01:01:31]         goto -> bb14;
[01:01:31]     }
[01:01:31]     bb6: {                              
[01:01:31]         _0 = ();
[01:01:31]         goto -> bb15;
[01:01:31]     }
[01:01:31]     bb7: {                              
[01:01:31]         falseEdges -> [real: bb12, imaginary: bb8];
[01:01:31]     }
[01:01:31]     bb8: {                              
[01:01:31]         falseEdges -> [real: bb13, imaginary: bb9];
[01:01:31]     }
[01:01:31]     bb9: {                              
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb10: {                             
[01:01:31]         goto -> bb8;
[01:01:31]     }
[01:01:31]     bb11: {                             
[01:01:31]         goto -> bb7;
[01:01:31]     }
[01:01:31]     bb12: {                             
[01:01:31]         goto -> bb5;
[01:01:31]     }
[01:01:31]     bb13: {                             
[01:01:31]         goto -> bb6;
[01:01:31]     }
[01:01:31]     bb14: {                             
[01:01:31]         FakeRead(ForLet, _2);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &_2;
[01:01:31]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:01:31]     }
[01:01:31]     bb15: {                             
[01:01:31]         goto -> bb16;
[01:01:31]     }
[01:01:31]     bb16: {                             
[01:01:31]         goto -> bb17;
[01:01:31]     }
[01:01:31]     bb17: {                             
[01:01:31]         goto -> bb18;
[01:01:31]     }
[01:01:31]     bb18: {                             
[01:01:31]         goto -> bb19;
[01:01:31]     }
[01:01:31]     bb19: {                             
[01:01:31]         goto -> bb20;
[01:01:31]     }
[01:01:31]     bb20: {                             
[01:01:31]         StorageDead(_3);
[01:01:31]         goto -> bb21;
[01:01:31]     }
[01:01:31]     bb21: {                             
[01:01:31]         goto -> bb22;
[01:01:31]     }
[01:01:31]     bb22: {                             
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb23;
[01:01:31]     }
[01:01:31]     bb23: {                             
[01:01:31]         goto -> bb24;
[01:01:31]     }
[01:01:31]     bb24: {                             
[01:01:31]         goto -> bb25;
[01:01:31]     }
[01:01:31]     bb25: {                             
[01:01:31]         goto -> bb2;
[01:01:31]     }
[01:01:31]     bb26: {                             
[01:01:31]         _5 = ();
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb27: {                             
[01:01:31]         StorageDead(_5);
[01:01:31]         goto -> bb14;
[01:01:31]     }
[01:01:31]     bb28: {                             
[01:01:31]         StorageDead(_7);
[01:01:31]         _1 = ();
[01:01:31]         StorageDead(_2);
[01:01:31]         goto -> bb1;
[01:01:31]     }
[01:01:31]     bb29: {                             
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block:    bb5: { // The loop body (body_block)
[01:01:31] Actual Line: "        FakeRead(ForLet, _5);"
[01:01:31] Expected Line: "       StorageDead(_5);"
[01:01:31] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]    bb1: { // The cleanup block
[01:01:31]        resume;
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]    bb3: { // Entry into the loop
[01:01:31]        _1 = ();
[01:01:31]        goto -> bb4;
[01:01:31]    }
[01:01:31]    bb4: { // The loop_block
[01:01:31]        falseUnwind -> [real: bb5, cleanup: bb1];
[01:01:31]    }
[01:01:31]    bb5: { // The loop body (body_block)
[01:01:31]        StorageLive(_5);
[01:01:31]        _5 = const 1i32;
[01:01:31]        StorageDead(_5);
[01:01:31]        goto -> bb4;
[01:01:31] ... (elided)
[01:01:31] Actual:
[01:01:31] fn main() -> (){
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]     scope 2 {
[01:01:31]         let _5: i32;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     let mut _1: ();
[01:01:31]     let mut _2: !;
[01:01:31]     let mut _3: !;
[01:01:31]     let mut _4: ();
[01:01:31]     bb0: {                              
[01:01:31]         switchInt(const true) -> [false: bb3, otherwise: bb2];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         _0 = ();
[01:01:31]         return;
[01:01:31]     }
[01:01:31]     bb3: {                              
[01:01:31]         _1 = ();
[01:01:31]         goto -> bb4;
[01:01:31]     }
[01:01:31]     bb4: {                              
[01:01:31]         falseUnwind -> [real: bb5, cleanup: bb1];
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = const 1i32;
[01:01:31]         FakeRead(ForLet, _5);
[01:01:31]         StorageDead(_5);
[01:01:31]         goto -> bb4;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block:  bb3: { // binding3(empty) and arm3
[01:01:31] Actual Line: "        FakeRead(ForMatch, _4);"
[01:01:31] Expected Line: "     ReadForMatch(_4);"
[01:01:31] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[01:01:31] ... (elided)
[01:01:31]  bb0: {
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31]      _2 = std::option::Option<i32>::Some(const 42i32,);
[01:01:31]      _3 = discriminant(_2);
[01:01:31]      _4 = &(promoted[1]: std::option::Option<i32>);
[01:01:31]      _9 = discriminant(_2);
[01:01:31]      switchInt(move _9) -> [0isize: bb5, 1isize: bb3, otherwise: bb7];
[01:01:31]  bb1: {
[01:01:31]      resume;
[01:01:31]  }
[01:01:31]  }
[01:01:31]  bb2: {  // arm1
[01:01:31]      _1 = (const 3i32, const 3i32);
[01:01:31]      goto -> bb13;
[01:01:31]  }
[01:01:31]  bb3: { // binding3(empty) and arm3
[01:01:31]      ReadForMatch(_4);
[01:01:31]      falseEdges -> [real: bb8, imaginary: bb4]; //pre_binding1
[01:01:31]  bb4: {
[01:01:31]      ReadForMatch(_4);
[01:01:31]      ReadForMatch(_4);
[01:01:31]      falseEdges -> [real: bb12, imaginary: bb5]; //pre_binding2
[01:01:31]  bb5: {
[01:01:31]      ReadForMatch(_4);
[01:01:31]      ReadForMatch(_4);
[01:01:31]      falseEdges -> [real: bb2, imaginary: bb6]; //pre_binding3
[01:01:31]  bb6: {
[01:01:31]      unreachable;
[01:01:31]  }
[01:01:31]  bb7: {
[01:01:31]  bb7: {
[01:01:31]      unreachable;
[01:01:31]  }
[01:01:31]  bb8: { // binding1 and guard
[01:01:31]      StorageLive(_7);
[01:01:31]      _7 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[01:01:31]      StorageLive(_10);
[01:01:31]      _10 = const guard() -> [return: bb9, unwind: bb1];
[01:01:31]  bb9: {
[01:01:31]  bb9: {
[01:01:31]      switchInt(move _10) -> [false: bb10, otherwise: bb11];
[01:01:31]  }
[01:01:31]  bb10: { // to pre_binding2
[01:01:31]      falseEdges -> [real: bb4, imaginary: bb4];
[01:01:31]  }
[01:01:31]  bb11: { // bindingNoLandingPads.before.mir2 and arm2
[01:01:31]      StorageLive(_5);
[01:01:31]      _5 = ((_2 as Some).0: i32);
[01:01:31]      StorageLive(_11);
[01:01:31]      _11 = _5;
[01:01:31]      _1 = (const 1i32, move _11);
[01:01:31]      StorageDead(_11);
[01:01:31]      goto -> bb13;
[01:01:31]  bb12: {
[01:01:31]  bb12: {
[01:01:31]      StorageLive(_8);
[01:01:31]      _8 = ((_2 as Some).0: i32);
[01:01:31]      StorageLive(_12);
[01:01:31]      _12 = _8;
[01:01:31]      _1 = (const 2i32, move_12);
[01:01:31]      StorageDead(_12);
[01:01:31]      goto -> bb13;
[01:01:31]  bb13: {
[01:01:31] ... (elided)
[01:01:31]      return;
[01:01:31]  }
[01:01:31]  }
[01:01:31] Actual:
[01:01:31] fn full_tested_match() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]     scope 2 {
[01:01:31]     }
[01:01:31]     }
[01:01:31]     let mut _1: (i32, i32);
[01:01:31]     let mut _2: std::option::Option<i32>;
[01:01:31]     let mut _3: isize;
[01:01:31]     let mut _4: &'<empty> std::option::Option<i32>;
[01:01:31]     let mut _9: isize;
[01:01:31]     let mut _10: bool;
[01:01:31]     let mut _11: i32;
[01:01:31]     let mut _12: i32;
[01:01:31]     bb0: {                              
[01:01:31]         StorageLive(_1);
[01:01:31]         StorageLive(_2);
[01:01:31]         _2 = std::option::Option<i32>::Some(const 42i32,);
[01:01:31]         _3 = discriminant(_2);
[01:01:31]         _4 = &(promoted[1]: std::option::Option<i32>);
[01:01:31]         _9 = discriminant(_2);
[01:01:31]         switchInt(move _9) -> [0isize: bb5, 1isize: bb3, otherwise: bb7];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         _1 = (const 3i32, const 3i32);
[01:01:31]         goto -> bb13;
[01:01:31]     }
[01:01:31]     bb3: {                              
[01:01:31]         FakeRead(ForMatch, _4);
[01:01:31]         falseEdges -> [real: bb8, imaginary: bb4];
[01:01:31]     }
[01:01:31]     bb4: {                              
[01:01:31]         FakeRead(ForMatch, _4);
[01:01:31]         falseEdges -> [real: bb12, imaginary: bb5];
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         FakeRead(ForMatch, _4);
[01:01:31]         falseEdges -> [real: bb2, imaginary: bb6];
[01:01:31]     }
[01:01:31]     bb6: {                              
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb7: {                              
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb8: {                              
[01:01:31]         StorageLive(_7);
[01:01:31]         _7 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[01:01:31]         StorageLive(_10);
[01:01:31]         _10 = const guard() -> [return: bb9, unwind: bb1];
[01:01:31]     }
[01:01:31]     bb9: {                              
[01:01:31]         switchInt(move _10) -> [false: bb10, otherwise: bb11];
[01:01:31]     }
[01:01:31]     bb10: {                             
[01:01:31]         falseEdges -> [real: bb4, imaginary: bb4];
[01:01:31]     }
[01:01:31]     bb11: {                             
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = ((_2 as Some).0: i32);
[01:01:31]         StorageLive(_11);
[01:01:31]         _11 = _5;
[01:01:31]         _1 = (const 1i32, move _11);
[01:01:31]         StorageDead(_11);
[01:01:31]         goto -> bb13;
[01:01:31]     }
[01:01:31]     bb12: {                             
[01:01:31]         StorageLive(_8);
[01:01:31]         _8 = ((_2 as Some).0: i32);
[01:01:31]         StorageLive(_12);
[01:01:31]         _12 = _8;
[01:01:31]         _1 = (const 2i32, move _12);
[01:01:31]         StorageDead(_12);
[01:01:31]         goto -> bb13;
[01:01:31]     }
[01:01:31]     bb13: {                             
[01:01:31]         StorageDead(_8);
[01:01:31]         StorageDead(_5);
[01:01:31]         StorageDead(_10);
[01:01:31]         StorageDead(_7);
[01:01:31]         StorageDead(_1);
[01:01:31]         StorageDead(_2);
[01:01:31]         _0 = ();
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:01:31] Expected Line: "| \'_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}"
[01:01:31] Test Name: rustc.main.nll.0.mir
[01:01:31] ... (elided)
[01:01:31] ... (elided)
[01:01:31] | '_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}
[01:01:31] | '_#3r    | U0 | {bb2[1..=3], bb3[0..=1]}
[01:01:31] | '_#4r    | U0 | {bb2[3], bb3[0..=1]}
[01:01:31] Actual:
[01:01:31] | Free Region Mapping
[01:01:31] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:01:31] | '_#1r    | Local    | ['_#1r]
[01:01:31] | Inferred Region Values
[01:01:31] | Inferred Region Values
[01:01:31] | '_#0r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#0r, '_#1r}
[01:01:31] | '_#1r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#1r}
[01:01:31] | '_#2r    | U0 | {bb2[0..=5], bb3[0..=1]}
[01:01:31] | '_#3r    | U0 | {bb2[1..=5], bb3[0..=1]}
[01:01:31] | '_#4r    | U0 | {bb2[4..=5], bb3[0..=1]}
[01:01:31] | Inference Constraints
[01:01:31] | Inference Constraints
[01:01:31] | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:01:31] | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:01:31] | '_#2r live at {bb2[0]}
[01:01:31] | '_#3r live at {bb2[1..=3]}
[01:01:31] | '_#4r live at {bb2[4..=5], bb3[0..=1]}
[01:01:31] | '_#2r: '_#3r due to Interesting(bb2[0])
[01:01:31] | '_#3r: '_#4r due to Interesting(bb2[3])
[01:01:31] fn main() -> (){
[01:01:31]     let mut _0: ();
[01:01:31]     scope 1 {
[01:01:31]         scope 3 {
[01:01:31]             scope 5 {
[01:01:31]             scope 6 {
[01:01:31]             scope 6 {
[01:01:31]                 let _6: &'_#4r usize;
[01:01:31]         }
[01:01:31]         scope 4 {
[01:01:31]         scope 4 {
[01:01:31]             let _2: &'_#3r usize;
[01:01:31]     }
[01:01:31]     scope 2 {
[01:01:31]     scope 2 {
[01:01:31]         let mut _1: [usize; 3];
[01:01:31]     let mut _3: usize;
[01:01:31]     let mut _4: usize;
[01:01:31]     let mut _4: usize;
[01:01:31]     let mut _5: bool;
[01:01:31]     let mut _7: bool;
[01:01:31]     let mut _8: usize;
[01:01:31]     let mut _9: bool;
[01:01:31]     bb0: {                              
[01:01:31]         StorageLive(_1);
[01:01:31]         _1 = [const 1usize, const 2usize, const 3usize];
[01:01:31]         FakeRead(ForLet, _1);
[01:01:31]         StorageLive(_2);
[01:01:31]         StorageLive(_3);
[01:01:31]         _3 = const 0usize;
[01:01:31]         _4 = Len(_1);
[01:01:31]         _5 = Lt(_3, _4);
[01:01:31]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:01:31]     bb1: {
[01:01:31]         resume;
[01:01:31]     }
[01:01:31]     }
[01:01:31]     bb2: {                              
[01:01:31]         _2 = &'_#2r _1[_3];
[01:01:31]         FakeRead(ForLet, _2);
[01:01:31]         StorageLive(_6);
[01:01:31]         _6 = _2;
[01:01:31]         FakeRead(ForLet, _6);
[01:01:31]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:01:31]     }
[01:01:31]     bb3: {                              
[01:01:31]         StorageLive(_8);
[01:01:31]         _8 = (*_6);
[01:01:31]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:01:31]     }
[01:01:31]     bb4: {                              
[01:01:31]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:01:31]     }
[01:01:31]     bb5: {                              
[01:01:31]         StorageDead(_8);
[01:01:31]         _0 = ();
[01:01:31]         goto -> bb7;
[01:01:31]     }
[01:01:31]     bb6: {                              
[01:01:31]         _0 = ();
[01:01:31]         goto -> bb7;
[01:01:31]     }
[01:01:31]     bb7: {                              
[01:01:31]         nop;
[01:01:31]         StorageDead(_6);
[01:01:31]         nop;
[01:01:31]         StorageDead(_2);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
[01:01:31] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:31] 
[01:01:31] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:01:31] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:01:31] thread '[mir-opt] mir-opt/storage_ranges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:31] Current block: None
[01:01:31] Actual Line: "        FakeRead(ForLet, _1);"
[01:01:31] Expected Line: "        StorageLive(_3);"
[01:01:31] Test Name: rustc.main.TypeckMir.before.mir
[01:01:31] ... (elided)
[01:01:31]     bb0: {
[01:01:31]     bb0: {
[01:01:31]         StorageLive(_1);
[01:01:31]         _1 = const 0i32;
[01:01:31]         StorageLive(_3);
[01:01:31]         StorageLive(_4);
[01:01:31]         StorageLive(_5);
[01:01:31]         _5 = _1;
[01:01:31]         _4 = std::option::Option<i32>::Some(move _5,);
[01:01:31]         StorageDead(_5);
[01:01:31]         _3 = &_4;
[01:01:31]         _2 = ();
[01:01:31]         StorageDead(_4);
[01:01:31]         StorageDead(_3);
[01:01:31]         StorageLive(_6);
[01:01:31]         _6 = const 1i32;
[01:01:31]         _0 = ();
[01:01:31]         StorageDead(_6);
[01:01:31]         StorageDead(_1);
[01:01:31]         return;
---
[01:01:31] test result: FAILED. 23 passed; 22 failed; 0 ignored; 0 measured; 0 filtered out
[01:01:31] 
[01:01:31] 
[01:01:31] 
[01:01:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:31] 
[01:01:31] 
[01:01:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:31] Build completed unsuccessfully in 0:16:12
[01:01:31] Build completed unsuccessfully in 0:16:12
[01:01:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b0c2774
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
