plain
[00:54:42] ....................................................................................................
[00:54:44] ....................................................i...............................................
[00:54:47] ....................................................................................................
[00:54:50] ....................................................................................................
[00:54:53] .iiiiiiiii..........................................................................................
[00:54:59] ....................................................................................................
[00:55:02] .................................................................................i..................
[00:55:05] ....................................................................................................
[00:55:08] ..................................i.i...ii..........................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:53] 
[01:01:53] running 45 tests
[01:01:54] ERROR 2018-09-14T17:30:58Z: compiletest::runtest: None
[01:01:54] ERROR 2018-09-14T17:30:59Z: compiletest::runtest: Some("    bb4: {")
[01:01:55] ERROR 2018-09-14T17:31:00Z: compiletest::runtest: None
[01:01:55] ERROR 2018-09-14T17:31:00Z: compiletest::runtest: Some("    bb2: {")
[01:01:56] ERROR 2018-09-14T17:31:00Z: compiletest::runtest: Some("    bb2: {")
[01:01:57] ERROR 2018-09-14T17:31:02Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:57] ERROR 2018-09-14T17:31:02Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:57] ERROR 2018-09-14T17:31:02Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:57] ERROR 2018-09-14T17:31:02Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:01:58] ERROR 2018-09-14T17:31:02Z: compiletest::runtest: Some("    bb0: {")
[01:01:58] ERROR 2018-09-14T17:31:03Z: compiletest::runtest: Some("    bb4: {")
[01:01:59] ERROR 2018-09-14T17:31:03Z: compiletest::runtest: Some("       _1 = D::{{constructor}}(const 0i32,);")
[01:01:59] ERROR 2018-09-14T17:31:04Z: compiletest::runtest: None
[01:01:59] ERROR 2018-09-14T17:31:04Z: compiletest::runtest: None
[01:02:00] ERROR 2018-09-14T17:31:05Z: compiletest::runtest: Some("    bb14: {")
[01:02:00] ERROR 2018-09-14T17:31:05Z: compiletest::runtest: Some("   bb5: { // The loop body (body_block)")
[01:02:05] ERROR 2018-09-14T17:31:09Z: compiletest::runtest: None
[01:02:06] ERROR 2018-09-14T17:31:11Z: compiletest::runtest: Some("        Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);")
[01:02:07] ERROR 2018-09-14T17:31:11Z: compiletest::runtest: None
[01:02:07] ERROR 2018-09-14T17:31:12Z: compiletest::runtest: Some("    bb0: {")
[01:02:12] .FF...F.FF..FFFFFFFFF....F.F..F....F.FFF.....
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block: None
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "       StorageLive(_2);"
[01:02:12] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:02:12] ... (elided)
[01:02:12]    bb0: {
[01:02:12]    bb0: {
[01:02:12]        StorageLive(_1);
[01:02:12]        _1 = const false;
[01:02:12]        StorageLive(_2);
[01:02:12]        StorageLive(_3);
[01:02:12]        _3 = _1;
[01:02:12]        _2 = move _3;
[01:02:12]        StorageDead(_3);
[01:02:12]        StorageLive(_4);
[01:02:12]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:02:12]        AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:02:12]        StorageLive(_5);
[01:02:12]        StorageLive(_6);
[01:02:12]        _6 = move _4;
[01:02:12]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:02:12] ... (elided)
[01:02:12]    bb2: {
[01:02:12]    bb2: {
[01:02:12]        drop(_6) -> [return: bb6, unwind: bb4];
[01:02:12] ... (elided)
[01:02:12]    bb5: {
[01:02:12]        drop(_6) -> bb4;
[01:02:12]    }
[01:02:12]    }
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]             scope 5 {
[01:02:12]                 scope 7 {
[01:02:12]                 scope 8 {
[01:02:12]                 scope 8 {
[01:02:12]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:02:12]             }
[01:02:12]             scope 6 {
[01:02:12]             scope 6 {
[01:02:12]                 let _4: std::option::Option<std::boxed::Box<u32>> as Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> };
[01:02:12]         }
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _2: bool;
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _1: bool;
[01:02:12]     }
[01:02:12]     let mut _3: bool;
[01:02:12]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = const false;
[01:02:12]         ReadForMatch(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = _1;
[01:02:12]         _2 = move _3;
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageLive(_4);
[01:02:12]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:02:12]         AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:02:12]         ReadForMatch(_4);
[01:02:12]         StorageLive(_5);
[01:02:12]         StorageLive(_6);
[01:02:12]         _6 = move _4;
[01:02:12]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:02:12]     bb1: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         drop(_6) -> [return: bb6, unwind: bb4];
[01:02:12]     bb3: {
[01:02:12]         drop(_4) -> bb1;
[01:02:12]     }
[01:02:12]     bb4: {
[01:02:12]     bb4: {
[01:02:12]         drop(_5) -> bb3;
[01:02:12]     }
[01:02:12]     bb5: {
[01:02:12]         drop(_6) -> bb4;
[01:02:12]     }
[01:02:12]     bb6: {                              
[01:02:12]         StorageDead(_6);
[01:02:12]         _0 = ();
[01:02:12]         drop(_5) -> [return: bb7, unwind: bb3];
[01:02:12]     }
[01:02:12]     bb7: {                              
[01:02:12]         StorageDead(_5);
[01:02:12]         drop(_4) -> [return: bb8, unwind: bb1];
[01:02:12]     }
[01:02:12]     bb8: {                              
[01:02:12]         StorageDead(_4);
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:     bb4: {
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "        StorageLive(_4);"
[01:02:12] Test Name: rustc.main.ElaborateDrops.before.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _1: std::boxed::Box<S>;
[01:02:12]     }
[01:02:12]     let mut _2: std::boxed::Box<S>;
[01:02:12]     let mut _3: ();
[01:02:12]     let mut _4: std::boxed::Box<S>;
[01:02:12]     bb0: {
[01:02:12]         StorageLive(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = Box(S);
[01:02:12]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:02:12]     bb1: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     bb2: {
[01:02:12]     bb2: {
[01:02:12]         _1 = move _2;
[01:02:12]         drop(_2) -> bb4;
[01:02:12]     }
[01:02:12]     bb3: {
[01:02:12]         drop(_2) -> bb1;
[01:02:12]     }
[01:02:12]     bb4: {
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageLive(_4);
[01:02:12]         _4 = move _1;
[01:02:12]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:02:12]     bb5: {
[01:02:12]     bb5: {
[01:02:12]         drop(_4) -> [return: bb8, unwind: bb6];
[01:02:12]     bb6: {
[01:02:12]         drop(_1) -> bb1;
[01:02:12]     }
[01:02:12]     bb7: {
[01:02:12]     bb7: {
[01:02:12]         drop(_4) -> bb6;
[01:02:12]     }
[01:02:12]     bb8: {
[01:02:12]         StorageDead(_4);
[01:02:12]         _0 = ();
[01:02:12]         drop(_1) -> bb9;
[01:02:12]     bb9: {
[01:02:12]     bb9: {
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _1: std::boxed::Box<S>;
[01:02:12]     }
[01:02:12]     let mut _2: std::boxed::Box<S>;
[01:02:12]     let mut _3: ();
[01:02:12]     let mut _4: std::boxed::Box<S>;
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = Box(S);
[01:02:12]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:02:12]     bb1: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         _1 = move _2;
[01:02:12]         drop(_2) -> bb4;
[01:02:12]     bb3: {
[01:02:12]         drop(_2) -> bb1;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb4: {                              
[01:02:12]         StorageDead(_2);
[01:02:12]         ReadForMatch(_1);
[01:02:12]         StorageLive(_4);
[01:02:12]         _4 = move _1;
[01:02:12]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:02:12]     }
[01:02:12]     bb5: {                              
[01:02:12]         drop(_4) -> [return: bb8, unwind: bb6];
[01:02:12]     bb6: {
[01:02:12]         drop(_1) -> bb1;
[01:02:12]     }
[01:02:12]     bb7: {
[01:02:12]     bb7: {
[01:02:12]         drop(_4) -> bb6;
[01:02:12]     }
[01:02:12]     bb8: {                              
[01:02:12]         StorageDead(_4);
[01:02:12]         _0 = ();
[01:02:12]         drop(_1) -> bb9;
[01:02:12]     }
[01:02:12]     bb9: {                              
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/end_region_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block: None
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "        StorageLive(_2);"
[01:02:12] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]     let mut _0: ();
[01:02:12] ... (elided)
[01:02:12]     let _2: &'10_1rs i32;
[01:02:12] ... (elided)
[01:02:12]     let _1: i32;
[01:02:12] ... (elided)
[01:02:12]     bb0: {
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = const 3i32;
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = &'10_1rs _1;
[01:02:12]         _0 = ();
[01:02:12]         EndRegion('10_1rs);
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _2: &'10_1rs i32;
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]         let _1: i32;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = const 3i32;
[01:02:12]         ReadForMatch(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = &'10_1rs _1;
[01:02:12]         ReadForMatch(_2);
[01:02:12]         _0 = ();
[01:02:12]         EndRegion('10_1rs);
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/end_region_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:     bb2: {
[01:02:12] Actual Line: "        ReadForMatch(_2);"
[01:02:12] Expected Line: "        StorageLive(_3);"
[01:02:12] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]     let mut _0: ();
[01:02:12] ... (elided)
[01:02:12]     let _7: &'23_3rs bool;
[01:02:12] ... (elided)
[01:02:12]     let _3: &'23_1rs bool;
[01:02:12] ... (elided)
[01:02:12]     let _2: bool;
[01:02:12] ... (elided)
[01:02:12]     let mut _4: ();
[01:02:12]     let mut _5: bool;
[01:02:12] ... (elided)
[01:02:12]     bb0: {
[01:02:12]         goto -> bb1;
[01:02:12]     bb1: {
[01:02:12]     bb1: {
[01:02:12]          falseUnwind -> [real: bb2, cleanup: bb3];
[01:02:12]     bb2: {
[01:02:12]     bb2: {
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = const true;
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = &'23_1rs _2;
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = _2;
[01:02:12]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:02:12]     bb3: {
[01:02:12] ... (elided)
[01:02:12]     }
[01:02:12]     bb4: {
[01:02:12]     bb4: {
[01:02:12]         _0 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         EndRegion('23_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageDead(_2);
[01:02:12]         return;
[01:02:12]     bb5: {
[01:02:12]         _4 = ();
[01:02:12]         _4 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         StorageLive(_7);
[01:02:12]         _7 = &'23_3rs _2;
[01:02:12]         _1 = ();
[01:02:12]         EndRegion('23_3rs);
[01:02:12]         StorageDead(_7);
[01:02:12]         EndRegion('23_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb1;
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]             scope 5 {
[01:02:12]             scope 6 {
[01:02:12]             scope 6 {
[01:02:12]                 let _7: &'23_3rs bool;
[01:02:12]         }
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _3: &'23_1rs bool;
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _2: bool;
[01:02:12]     }
[01:02:12]     let mut _1: ();
[01:02:12]     let mut _4: ();
[01:02:12]     let mut _5: bool;
[01:02:12]     let mut _6: !;
[01:02:12]     bb0: {                              
[01:02:12]         goto -> bb1;
[01:02:12]     }
[01:02:12]     bb1: {                              
[01:02:12]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         StorageLive(_2);
[01:02:12]         _2 = const true;
[01:02:12]         ReadForMatch(_2);
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = &'23_1rs _2;
[01:02:12]         ReadForMatch(_3);
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = _2;
[01:02:12]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:02:12]     bb3: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb4: {                              
[01:02:12]         _0 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         EndRegion('23_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageDead(_2);
[01:02:12]         return;
[01:02:12]     }
[01:02:12]     bb5: {                              
[01:02:12]         _4 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         StorageLive(_7);
[01:02:12]         _7 = &'23_3rs _2;
[01:02:12]         ReadForMatch(_7);
[01:02:12]         _1 = ();
[01:02:12]         EndRegion('23_3rs);
[01:02:12]         StorageDead(_7);
[01:02:12]         EndRegion('23_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb1;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/end_region_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:     bb2: {
[01:02:12] Actual Line: "        ReadForMatch(_3);"
[01:02:12] Expected Line: "        StorageLive(_5);"
[01:02:12] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]     let mut _0: ();
[01:02:12] ... (elided)
[01:02:12]     let _7: &'26_3rs bool;
[01:02:12] ... (elided)
[01:02:12]     let _3: &'26_1rs bool;
[01:02:12] ... (elided)
[01:02:12]     let mut _1: bool;
[01:02:12] ... (elided)
[01:02:12]     let mut _2: ();
[01:02:12]     let mut _4: ();
[01:02:12]     let mut _5: bool;
[01:02:12]     let mut _6: !;
[01:02:12]     bb0: {
[01:02:12]         StorageLive(_1);
[01:02:12]         goto -> bb1;
[01:02:12]     bb1: {
[01:02:12]     bb1: {
[01:02:12]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:02:12]     bb2: {
[01:02:12]         _1 = const true;
[01:02:12]         _1 = const true;
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = &'26_1rs _1;
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = _1;
[01:02:12]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:02:12]     bb3: {
[01:02:12] ... (elided)
[01:02:12]     }
[01:02:12]     bb4: {
[01:02:12]     bb4: {
[01:02:12]         _0 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         EndRegion('26_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12]     bb5: {
[01:02:12]         _4 = ();
[01:02:12]         _4 = ();
[01:02:12]         StorageDead(_5);
[01:02:12]         StorageLive(_7);
[01:02:12]         _7 = &'26_3rs _1;
[01:02:12]         _2 = ();
[01:02:12]         EndRegion('26_3rs);
[01:02:12]         StorageDead(_7);
[01:02:12]         EndRegion('26_1rs);
[01:02:12]         StorageDead(_3);
[01:02:12]         goto -> bb1;
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]             scope 5 {
[01:02:12]             scope 6 {
[01:02:12]             scope 6 {
[01:02:12]                 let _7: &'26_3rs bool;
[01:02:12]         }
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _3: &'26_1rs bool;
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let mut _1: bool;
[01:02:12]     }
---
[01:02:12]     }
[01:02:12]     bb13: {
[01:02:12]         goto -> bb6;
[01:02:12]     }
[01:02:12]     bb14: {
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageLive(_7);
[01:02:12]         _7 = &_2;
[01:02:12]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:02:12]     bb15: {
[01:02:12]         goto -> bb16;
[01:02:12]     }
[01:02:12]     bb16: {
---
[01:02:12]     }
[01:02:12]     bb19: {
[01:02:12]         goto -> bb20;
[01:02:12]     }
[01:02:12]     bb20: {
[01:02:12]         StorageDead(_3);
[01:02:12]         goto -> bb21;
[01:02:12]     bb21: {
[01:02:12]         goto -> bb22;
[01:02:12]     }
[01:02:12]     bb22: {
[01:02:12]     bb22: {
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb23;
[01:02:12]     bb23: {
[01:02:12]         goto -> bb24;
[01:02:12]     }
[01:02:12]     bb24: {
---
[01:02:12]     bb26: {
[01:02:12]         _5 = ();
[01:02:12]         unreachable;
[01:02:12]     }
[01:02:12]     bb27: {
[01:02:12]         StorageDead(_5);
[01:02:12]         goto -> bb14;
[01:02:12]     bb28: {
[01:02:12]     bb28: {
[01:02:12]         StorageDead(_7);
[01:02:12]         _1 = ();
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb1;
[01:02:12]     bb29: {
[01:02:12]         return;
[01:02:12]     }
[01:02:12] }
[01:02:12] }
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]         let _2: i32;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     let mut _1: ();
[01:02:12]     let mut _3: bool;
[01:02:12]     let mut _4: u8;
[01:02:12]     let mut _5: !;
[01:02:12]     let mut _6: ();
[01:02:12]     let mut _7: &i32;
[01:02:12]     bb0: {                              
[01:02:12]         goto -> bb1;
[01:02:12]     }
[01:02:12]     bb1: {                              
[01:02:12]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         goto -> bb29;
[01:02:12]     }
[01:02:12]     bb3: {                              
[01:02:12]         StorageLive(_2);
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = const true;
[01:02:12]         _4 = discriminant(_3);
[01:02:12]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[01:02:12]     bb4: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb5: {                              
[01:02:12]         _2 = const 4i32;
[01:02:12]         goto -> bb14;
[01:02:12]     }
[01:02:12]     bb6: {                              
[01:02:12]         _0 = ();
[01:02:12]         goto -> bb15;
[01:02:12]     }
[01:02:12]     bb7: {                              
[01:02:12]         falseEdges -> [real: bb12, imaginary: bb8];
[01:02:12]     }
[01:02:12]     bb8: {                              
[01:02:12]         falseEdges -> [real: bb13, imaginary: bb9];
[01:02:12]     }
[01:02:12]     bb9: {                              
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb10: {                             
[01:02:12]         goto -> bb8;
[01:02:12]     }
[01:02:12]     bb11: {                             
[01:02:12]         goto -> bb7;
[01:02:12]     }
[01:02:12]     bb12: {                             
[01:02:12]         goto -> bb5;
[01:02:12]     }
[01:02:12]     bb13: {                             
[01:02:12]         goto -> bb6;
[01:02:12]     }
[01:02:12]     bb14: {                             
[01:02:12]         ReadForMatch(_2);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageLive(_7);
[01:02:12]         _7 = &_2;
[01:02:12]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:02:12]     }
[01:02:12]     bb15: {                             
[01:02:12]         goto -> bb16;
[01:02:12]     }
[01:02:12]     bb16: {                             
[01:02:12]         goto -> bb17;
[01:02:12]     }
[01:02:12]     bb17: {                             
[01:02:12]         goto -> bb18;
[01:02:12]     }
[01:02:12]     bb18: {                             
[01:02:12]         goto -> bb19;
[01:02:12]     }
[01:02:12]     bb19: {                             
[01:02:12]         goto -> bb20;
[01:02:12]     }
[01:02:12]     bb20: {                             
[01:02:12]         StorageDead(_3);
[01:02:12]         goto -> bb21;
[01:02:12]     }
[01:02:12]     bb21: {                             
[01:02:12]         goto -> bb22;
[01:02:12]     }
[01:02:12]     bb22: {                             
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb23;
[01:02:12]     }
[01:02:12]     bb23: {                             
[01:02:12]         goto -> bb24;
[01:02:12]     }
[01:02:12]     bb24: {                             
[01:02:12]         goto -> bb25;
[01:02:12]     }
[01:02:12]     bb25: {                             
[01:02:12]         goto -> bb2;
[01:02:12]     }
[01:02:12]     bb26: {                             
[01:02:12]         _5 = ();
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb27: {                             
[01:02:12]         StorageDead(_5);
[01:02:12]         goto -> bb14;
[01:02:12]     }
[01:02:12]     bb28: {                             
[01:02:12]         StorageDead(_7);
[01:02:12]         _1 = ();
[01:02:12]         StorageDead(_2);
[01:02:12]         goto -> bb1;
[01:02:12]     }
[01:02:12]     bb29: {                             
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:    bb5: { // The loop body (body_block)
[01:02:12] Actual Line: "        ReadForMatch(_5);"
[01:02:12] Expected Line: "       StorageDead(_5);"
[01:02:12] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]    bb1: { // The cleanup block
[01:02:12]        resume;
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12]    bb3: { // Entry into the loop
[01:02:12]        _1 = ();
[01:02:12]        goto -> bb4;
[01:02:12]    }
[01:02:12]    bb4: { // The loop_block
[01:02:12]        falseUnwind -> [real: bb5, cleanup: bb1];
[01:02:12]    }
[01:02:12]    bb5: { // The loop body (body_block)
[01:02:12]        StorageLive(_5);
[01:02:12]        _5 = const 1i32;
[01:02:12]        StorageDead(_5);
[01:02:12]        goto -> bb4;
[01:02:12] ... (elided)
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]         let _5: i32;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     let mut _1: ();
[01:02:12]     let mut _2: !;
[01:02:12]     let mut _3: !;
[01:02:12]     let mut _4: ();
[01:02:12]     bb0: {                              
[01:02:12]         switchInt(const true) -> [false: bb3, otherwise: bb2];
[01:02:12]     bb1: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         _0 = ();
[01:02:12]         return;
[01:02:12]     }
[01:02:12]     bb3: {                              
[01:02:12]         _1 = ();
[01:02:12]         goto -> bb4;
[01:02:12]     }
[01:02:12]     bb4: {                              
[01:02:12]         falseUnwind -> [real: bb5, cleanup: bb1];
[01:02:12]     }
[01:02:12]     bb5: {                              
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = const 1i32;
[01:02:12]         ReadForMatch(_5);
[01:02:12]         StorageDead(_5);
[01:02:12]         goto -> bb4;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:02:12] Expected Line: "| \'_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}"
[01:02:12] Test Name: rustc.main.nll.0.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12] | '_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}
[01:02:12] | '_#3r    | U0 | {bb2[1..=3], bb3[0..=1]}
[01:02:12] | '_#4r    | U0 | {bb2[3], bb3[0..=1]}
[01:02:12] Actual:
[01:02:12] | Free Region Mapping
[01:02:12] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:02:12] | '_#1r    | Local    | ['_#1r]
[01:02:12] | Inferred Region Values
[01:02:12] | Inferred Region Values
[01:02:12] | '_#0r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#0r, '_#1r}
[01:02:12] | '_#1r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#1r}
[01:02:12] | '_#2r    | U0 | {bb2[0..=5], bb3[0..=1]}
[01:02:12] | '_#3r    | U0 | {bb2[1..=5], bb3[0..=1]}
[01:02:12] | '_#4r    | U0 | {bb2[4..=5], bb3[0..=1]}
[01:02:12] | Inference Constraints
[01:02:12] | Inference Constraints
[01:02:12] | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:02:12] | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:02:12] | '_#2r live at {bb2[0]}
[01:02:12] | '_#3r live at {bb2[1..=3]}
[01:02:12] | '_#4r live at {bb2[4..=5], bb3[0..=1]}
[01:02:12] | '_#2r: '_#3r due to Interesting(bb2[0])
[01:02:12] | '_#3r: '_#4r due to Interesting(bb2[3])
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]             scope 5 {
[01:02:12]             scope 6 {
[01:02:12]             scope 6 {
[01:02:12]                 let _6: &'_#4r usize;
[01:02:12]         }
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _2: &'_#3r usize;
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let mut _1: [usize; 3];
[01:02:12]     let mut _3: usize;
[01:02:12]     let mut _4: usize;
[01:02:12]     let mut _4: usize;
[01:02:12]     let mut _5: bool;
[01:02:12]     let mut _7: bool;
[01:02:12]     let mut _8: usize;
[01:02:12]     let mut _9: bool;
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = [const 1usize, const 2usize, const 3usize];
[01:02:12]         ReadForMatch(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = const 0usize;
[01:02:12]         _4 = Len(_1);
[01:02:12]         _5 = Lt(_3, _4);
[01:02:12]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:02:12]     bb1: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         _2 = &'_#2r _1[_3];
[01:02:12]         ReadForMatch(_2);
[01:02:12]         StorageLive(_6);
[01:02:12]         _6 = _2;
[01:02:12]         ReadForMatch(_6);
[01:02:12]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:02:12]     }
[01:02:12]     bb3: {                              
[01:02:12]         StorageLive(_8);
[01:02:12]         _8 = (*_6);
[01:02:12]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:02:12]     }
[01:02:12]     bb4: {                              
[01:02:12]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:02:12]     }
[01:02:12]     bb5: {                              
[01:02:12]         StorageDead(_8);
[01:02:12]         _0 = ();
[01:02:12]         goto -> bb7;
[01:02:12]     }
[01:02:12]     bb6: {                              
[01:02:12]         _0 = ();
[01:02:12]         goto -> bb7;
[01:02:12]     }
[01:02:12]     bb7: {                              
[01:02:12]         nop;
[01:02:12]         StorageDead(_6);
[01:02:12]         nop;
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/storage_ranges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block: None
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "        StorageLive(_3);"
[01:02:12] Test Name: rustc.main.TypeckMir.before.mir
[01:02:12] ... (elided)
[01:02:12]     bb0: {
[01:02:12]     bb0: {
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = const 0i32;
[01:02:12]         StorageLive(_3);
[01:02:12]         StorageLive(_4);
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = _1;
[01:02:12]         _4 = std::option::Option<i32>::Some(move _5,);
[01:02:12]         StorageDead(_5);
[01:02:12]         _3 = &_4;
[01:02:12]         _2 = ();
[01:02:12]         StorageDead(_4);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageLive(_6);
[01:02:12]         _6 = const 1i32;
[01:02:12]         _0 = ();
[01:02:12]         StorageDead(_6);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]         scope 4 {
[01:02:12]         scope 4 {
[01:02:12]             let _3: &std::option::Option<i32>;
[01:02:12]         scope 5 {
[01:02:12]         }
[01:02:12]         scope 6 {
[01:02:12]             let _6: i32;
[01:02:12]             let _6: i32;
[01:02:12]         }
[01:02:12]     }
[01:02:12]     scope 2 {
[01:02:12]         let _1: i32;
[01:02:12]     }
[01:02:12]     let mut _2: ();
[01:02:12]     let _4: std::option::Option<i32>;
[01:02:12]     let mut _5: i32;
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         _1 = const 0i32;
[01:02:12]         ReadForMatch(_1);
[01:02:12]         StorageLive(_3);
[01:02:12]         StorageLive(_4);
[01:02:12]         StorageLive(_5);
[01:02:12]         _5 = _1;
[01:02:12]         _4 = std::option::Option<i32>::Some(move _5,);
[01:02:12]         StorageDead(_5);
[01:02:12]         _3 = &_4;
[01:02:12]         ReadForMatch(_3);
[01:02:12]         _2 = ();
[01:02:12]         StorageDead(_4);
[01:02:12]         StorageDead(_3);
[01:02:12]         StorageLive(_6);
[01:02:12]         _6 = const 1i32;
[01:02:12]         ReadForMatch(_6);
[01:02:12]         _0 = ();
[01:02:12]         StorageDead(_6);
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:02:12] Actual Line: "        ReadForMatch(_3);"
[01:02:12] Expected Line: "        _0 = (*_3);"
[01:02:12] Test Name: rustc.main-{{closure}}.EraseRegions.after.mir
[01:02:12] ... (elided)
[01:02:12] ... (elided)
[01:02:12] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {
[01:02:12] ... (elided)
[01:02:12]     bb0: {
[01:02:12]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:02:12]         StorageLive(_3);
[01:02:12]         Validate(Suspend(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 })), [(*_2): i32]);
[01:02:12]         _3 = &ReErased (*_2);
[01:02:12]         Validate(Acquire, [(*_3): i32/ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 }) (imm)]);
[01:02:12]         _0 = (*_3);
[01:02:12]         EndRegion(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0 }));
[01:02:12]         StorageDead(_3);
[01:02:12]         return;
[01:02:12] }
[01:02:12] Actual:
[01:02:12] Actual:
[01:02:12] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32{
[01:02:12]     let mut _0: i32;
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _3: &ReErased i32;
[01:02:12]     }
[01:02:12]     bb0: {                              
[01:02:12]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:02:12]         StorageLive(_3);
[01:02:12]         Validate(Suspend(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0})), [(*_2): i32]);
[01:02:12]         _3 = &ReErased (*_2);
[01:02:12]         Validate(Acquire, [(*_3): i32/ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0}) (imm)]);
[01:02:12]         ReadForMatch(_3);
[01:02:12]         _0 = (*_3);
[01:02:12]         EndRegion(ReScope(Remainder { block: ItemLocalId(25), first_statement_index: 0}));
[01:02:12]         StorageDead(_3);
[01:02:12]         return;
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:02:12] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/validate_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block: None
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "        _0 = ();"
[01:02:12] Test Name: rustc.main.EraseRegions.after.mir
[01:02:12] ... (elided)
[01:02:12] fn main() -> () {
[01:02:12] ... (elided)
[01:02:12]     bb1: {
[01:02:12]     bb1: {
[01:02:12]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);
[01:02:12]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>]);
[01:02:12]         _1 = move _2 as std::boxed::Box<[i32]> (Unsize);
[01:02:12]         Validate(Acquire, [_1: std::boxed::Box<[i32]>]);
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_3);
[01:02:12]         _0 = ();
[01:02:12]         Validate(Release, [_1: std::boxed::Box<[i32]>]);
[01:02:12]         drop(_1) -> [return: bb2, unwind: bb3];
[01:02:12] ... (elided)
[01:02:12] }
[01:02:12] Actual:
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]     scope 2 {
[01:02:12]     scope 2 {
[01:02:12]         let _1: std::boxed::Box<[i32]> as Canonical { variables: [], value: std::boxed::Box<[i32]> };
[01:02:12]     }
[01:02:12]     let mut _2: std::boxed::Box<[i32; 3]>;
[01:02:12]     let mut _3: [i32; 3];
[01:02:12]     bb0: {                              
[01:02:12]         StorageLive(_1);
[01:02:12]         StorageLive(_2);
[01:02:12]         StorageLive(_3);
[01:02:12]         _3 = [const 1i32, const 2i32, const 3i32];
[01:02:12]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>, _3: [i32; 3]]);
[01:02:12]         _2 = const <std::boxed::Box<T>>::new(move _3) -> bb1;
[01:02:12]     }
[01:02:12]     bb1: {                              
[01:02:12]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);
[01:02:12]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>]);
[01:02:12]         _1 = move _2 as std::boxed::Box<[i32]> (Unsize);
[01:02:12]         Validate(Acquire, [_1: std::boxed::Box<[i32]>]);
[01:02:12]         StorageDead(_2);
[01:02:12]         StorageDead(_3);
[01:02:12]         ReadForMatch(_1);
[01:02:12]         _0 = ();
[01:02:12]         Validate(Release, [_1: std::boxed::Box<[i32]>]);
[01:02:12]         drop(_1) -> [return: bb2, unwind: bb3];
[01:02:12]     }
[01:02:12]     bb2: {                              
[01:02:12]         StorageDead(_1);
[01:02:12]         return;
[01:02:12]     bb3: {
[01:02:12]         resume;
[01:02:12]     }
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] }', tools/compiletest/src/runtest.rs:2860:13
[01:02:12] 
[01:02:12] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:02:12] thread '[mir-opt] mir-opt/validate_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:12] Current block:     bb0: {
[01:02:12] Actual Line: "        ReadForMatch(_1);"
[01:02:12] Expected Line: "        StorageLive(_2);"
[01:02:12] Test Name: rustc.main.EraseRegions.after.mir
[01:02:12] ... (elided)
[01:02:12] fn main() -> (){
[01:02:12] fn main() -> (){
[01:02:12]     let mut _0: ();
[01:02:12]     scope 1 {
[01:02:12]         scope 3 {
[01:02:12]         scope 4 {
---
[01:02:12] test result: FAILED. 24 passed; 21 failed; 0 ignored; 0 measured; 0 filtered out
[01:02:12] 
[01:02:12] 
[01:02:12] 
[01:02:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:12] 
[01:02:12] 
[01:02:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:12] Build completed unsuccessfully in 0:16:17
[01:02:12] Build completed unsuccessfully in 0:16:17
[01:02:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dadbeb0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:131555da:start=1536946279479802146,finish=1536946279485492550,duration=5690404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00d3e74d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10f1dcc6
travis_time:start:10f1dcc6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:268335a6
$ dmesg | grep -i kill
