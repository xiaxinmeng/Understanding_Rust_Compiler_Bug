plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:47] 
[01:00:47] running 45 tests
[01:00:47] ERROR 2018-09-13T16:54:44Z: compiletest::runtest: None
[01:00:47] ERROR 2018-09-13T16:54:44Z: compiletest::runtest: Some("    bb4: {")
[01:00:48] ERROR 2018-09-13T16:54:45Z: compiletest::runtest: None
[01:00:49] ERROR 2018-09-13T16:54:45Z: compiletest::runtest: Some("    bb2: {")
[01:00:49] ERROR 2018-09-13T16:54:45Z: compiletest::runtest: Some("    bb2: {")
[01:00:50] ERROR 2018-09-13T16:54:47Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:00:50] ERROR 2018-09-13T16:54:47Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:00:51] ERROR 2018-09-13T16:54:47Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:00:51] ERROR 2018-09-13T16:54:47Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:00:51] ERROR 2018-09-13T16:54:48Z: compiletest::runtest: Some("    bb0: {")
[01:00:51] ERROR 2018-09-13T16:54:48Z: compiletest::runtest: Some("    bb4: {")
[01:00:52] ERROR 2018-09-13T16:54:49Z: compiletest::runtest: Some("       _1 = D::{{constructor}}(const 0i32,);")
[01:00:52] ERROR 2018-09-13T16:54:49Z: compiletest::runtest: None
[01:00:52] ERROR 2018-09-13T16:54:49Z: compiletest::runtest: None
[01:00:53] ERROR 2018-09-13T16:54:50Z: compiletest::runtest: Some("    bb14: {")
[01:00:53] ERROR 2018-09-13T16:54:50Z: compiletest::runtest: Some("   bb5: { // The loop body (body_block)")
[01:00:58] ERROR 2018-09-13T16:54:54Z: compiletest::runtest: None
[01:00:59] ERROR 2018-09-13T16:54:56Z: compiletest::runtest: Some("        Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);")
[01:01:00] ERROR 2018-09-13T16:54:56Z: compiletest::runtest: None
[01:01:00] ERROR 2018-09-13T16:54:57Z: compiletest::runtest: Some("    bb0: {")
[01:01:05] .FF...F.FF..FFFFFFFFF.....FF..F....F.FFF.....
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block: None
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "       StorageLive(_2);"
[01:01:05] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:01:05] ... (elided)
[01:01:05]    bb0: {
[01:01:05]    bb0: {
[01:01:05]        StorageLive(_1);
[01:01:05]        _1 = const false;
[01:01:05]        StorageLive(_2);
[01:01:05]        StorageLive(_3);
[01:01:05]        _3 = _1;
[01:01:05]        _2 = move _3;
[01:01:05]        StorageDead(_3);
[01:01:05]        StorageLive(_4);
[01:01:05]        _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:01:05]        AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:01:05]        StorageLive(_5);
[01:01:05]        StorageLive(_6);
[01:01:05]        _6 = move _4;
[01:01:05]        replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:01:05] ... (elided)
[01:01:05]    bb2: {
[01:01:05]    bb2: {
[01:01:05]        drop(_6) -> [return: bb6, unwind: bb4];
[01:01:05] ... (elided)
[01:01:05]    bb5: {
[01:01:05]        drop(_6) -> bb4;
[01:01:05]    }
[01:01:05]    }
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]             scope 5 {
[01:01:05]                 scope 7 {
[01:01:05]                 scope 8 {
[01:01:05]                 scope 8 {
[01:01:05]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[01:01:05]             }
[01:01:05]             scope 6 {
[01:01:05]             scope 6 {
[01:01:05]                 let _4: std::option::Option<std::boxed::Box<u32>> as Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> };
[01:01:05]         }
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _2: bool;
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _1: bool;
[01:01:05]     }
[01:01:05]     let mut _3: bool;
[01:01:05]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = const false;
[01:01:05]         ReadForMatch(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = _1;
[01:01:05]         _2 = move _3;
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageLive(_4);
[01:01:05]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:01:05]         AscribeUserType(_4, o, Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> });
[01:01:05]         ReadForMatch(_4);
[01:01:05]         StorageLive(_5);
[01:01:05]         StorageLive(_6);
[01:01:05]         _6 = move _4;
[01:01:05]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[01:01:05]     bb1: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         drop(_6) -> [return: bb6, unwind: bb4];
[01:01:05]     bb3: {
[01:01:05]         drop(_4) -> bb1;
[01:01:05]     }
[01:01:05]     bb4: {
[01:01:05]     bb4: {
[01:01:05]         drop(_5) -> bb3;
[01:01:05]     }
[01:01:05]     bb5: {
[01:01:05]         drop(_6) -> bb4;
[01:01:05]     }
[01:01:05]     bb6: {                              
[01:01:05]         StorageDead(_6);
[01:01:05]         _0 = ();
[01:01:05]         drop(_5) -> [return: bb7, unwind: bb3];
[01:01:05]     }
[01:01:05]     bb7: {                              
[01:01:05]         StorageDead(_5);
[01:01:05]         drop(_4) -> [return: bb8, unwind: bb1];
[01:01:05]     }
[01:01:05]     bb8: {                              
[01:01:05]         StorageDead(_4);
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:     bb4: {
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "        StorageLive(_4);"
[01:01:05] Test Name: rustc.main.ElaborateDrops.before.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _1: std::boxed::Box<S>;
[01:01:05]     }
[01:01:05]     let mut _2: std::boxed::Box<S>;
[01:01:05]     let mut _3: ();
[01:01:05]     let mut _4: std::boxed::Box<S>;
[01:01:05]     bb0: {
[01:01:05]         StorageLive(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = Box(S);
[01:01:05]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:01:05]     bb1: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     bb2: {
[01:01:05]     bb2: {
[01:01:05]         _1 = move _2;
[01:01:05]         drop(_2) -> bb4;
[01:01:05]     }
[01:01:05]     bb3: {
[01:01:05]         drop(_2) -> bb1;
[01:01:05]     }
[01:01:05]     bb4: {
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageLive(_4);
[01:01:05]         _4 = move _1;
[01:01:05]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:01:05]     bb5: {
[01:01:05]     bb5: {
[01:01:05]         drop(_4) -> [return: bb8, unwind: bb6];
[01:01:05]     bb6: {
[01:01:05]         drop(_1) -> bb1;
[01:01:05]     }
[01:01:05]     bb7: {
[01:01:05]     bb7: {
[01:01:05]         drop(_4) -> bb6;
[01:01:05]     }
[01:01:05]     bb8: {
[01:01:05]         StorageDead(_4);
[01:01:05]         _0 = ();
[01:01:05]         drop(_1) -> bb9;
[01:01:05]     bb9: {
[01:01:05]     bb9: {
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _1: std::boxed::Box<S>;
[01:01:05]     }
[01:01:05]     let mut _2: std::boxed::Box<S>;
[01:01:05]     let mut _3: ();
[01:01:05]     let mut _4: std::boxed::Box<S>;
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = Box(S);
[01:01:05]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:01:05]     bb1: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         _1 = move _2;
[01:01:05]         drop(_2) -> bb4;
[01:01:05]     bb3: {
[01:01:05]         drop(_2) -> bb1;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb4: {                              
[01:01:05]         StorageDead(_2);
[01:01:05]         ReadForMatch(_1);
[01:01:05]         StorageLive(_4);
[01:01:05]         _4 = move _1;
[01:01:05]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:01:05]     }
[01:01:05]     bb5: {                              
[01:01:05]         drop(_4) -> [return: bb8, unwind: bb6];
[01:01:05]     bb6: {
[01:01:05]         drop(_1) -> bb1;
[01:01:05]     }
[01:01:05]     bb7: {
[01:01:05]     bb7: {
[01:01:05]         drop(_4) -> bb6;
[01:01:05]     }
[01:01:05]     bb8: {                              
[01:01:05]         StorageDead(_4);
[01:01:05]         _0 = ();
[01:01:05]         drop(_1) -> bb9;
[01:01:05]     }
[01:01:05]     bb9: {                              
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/end_region_1.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/end_region_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block: None
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "        StorageLive(_2);"
[01:01:05] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]     let mut _0: ();
[01:01:05] ... (elided)
[01:01:05]     let _2: &'10_1rs i32;
[01:01:05] ... (elided)
[01:01:05]     let _1: i32;
[01:01:05] ... (elided)
[01:01:05]     bb0: {
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = const 3i32;
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = &'10_1rs _1;
[01:01:05]         _0 = ();
[01:01:05]         EndRegion('10_1rs);
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _2: &'10_1rs i32;
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]         let _1: i32;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = const 3i32;
[01:01:05]         ReadForMatch(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = &'10_1rs _1;
[01:01:05]         ReadForMatch(_2);
[01:01:05]         _0 = ();
[01:01:05]         EndRegion('10_1rs);
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/end_region_2.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/end_region_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:     bb2: {
[01:01:05] Actual Line: "        ReadForMatch(_2);"
[01:01:05] Expected Line: "        StorageLive(_3);"
[01:01:05] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]     let mut _0: ();
[01:01:05] ... (elided)
[01:01:05]     let _7: &'23_3rs bool;
[01:01:05] ... (elided)
[01:01:05]     let _3: &'23_1rs bool;
[01:01:05] ... (elided)
[01:01:05]     let _2: bool;
[01:01:05] ... (elided)
[01:01:05]     let mut _4: ();
[01:01:05]     let mut _5: bool;
[01:01:05] ... (elided)
[01:01:05]     bb0: {
[01:01:05]         goto -> bb1;
[01:01:05]     bb1: {
[01:01:05]     bb1: {
[01:01:05]          falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:05]     bb2: {
[01:01:05]     bb2: {
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = const true;
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = &'23_1rs _2;
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = _2;
[01:01:05]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:05]     bb3: {
[01:01:05] ... (elided)
[01:01:05]     }
[01:01:05]     bb4: {
[01:01:05]     bb4: {
[01:01:05]         _0 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         EndRegion('23_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageDead(_2);
[01:01:05]         return;
[01:01:05]     bb5: {
[01:01:05]         _4 = ();
[01:01:05]         _4 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         StorageLive(_7);
[01:01:05]         _7 = &'23_3rs _2;
[01:01:05]         _1 = ();
[01:01:05]         EndRegion('23_3rs);
[01:01:05]         StorageDead(_7);
[01:01:05]         EndRegion('23_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb1;
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]             scope 5 {
[01:01:05]             scope 6 {
[01:01:05]             scope 6 {
[01:01:05]                 let _7: &'23_3rs bool;
[01:01:05]         }
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _3: &'23_1rs bool;
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _2: bool;
[01:01:05]     }
[01:01:05]     let mut _1: ();
[01:01:05]     let mut _4: ();
[01:01:05]     let mut _5: bool;
[01:01:05]     let mut _6: !;
[01:01:05]     bb0: {                              
[01:01:05]         goto -> bb1;
[01:01:05]     }
[01:01:05]     bb1: {                              
[01:01:05]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         StorageLive(_2);
[01:01:05]         _2 = const true;
[01:01:05]         ReadForMatch(_2);
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = &'23_1rs _2;
[01:01:05]         ReadForMatch(_3);
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = _2;
[01:01:05]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:05]     bb3: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb4: {                              
[01:01:05]         _0 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         EndRegion('23_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageDead(_2);
[01:01:05]         return;
[01:01:05]     }
[01:01:05]     bb5: {                              
[01:01:05]         _4 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         StorageLive(_7);
[01:01:05]         _7 = &'23_3rs _2;
[01:01:05]         ReadForMatch(_7);
[01:01:05]         _1 = ();
[01:01:05]         EndRegion('23_3rs);
[01:01:05]         StorageDead(_7);
[01:01:05]         EndRegion('23_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb1;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/end_region_3.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/end_region_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:     bb2: {
[01:01:05] Actual Line: "        ReadForMatch(_3);"
[01:01:05] Expected Line: "        StorageLive(_5);"
[01:01:05] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]     let mut _0: ();
[01:01:05] ... (elided)
[01:01:05]     let _7: &'26_3rs bool;
[01:01:05] ... (elided)
[01:01:05]     let _3: &'26_1rs bool;
[01:01:05] ... (elided)
[01:01:05]     let mut _1: bool;
[01:01:05] ... (elided)
[01:01:05]     let mut _2: ();
[01:01:05]     let mut _4: ();
[01:01:05]     let mut _5: bool;
[01:01:05]     let mut _6: !;
[01:01:05]     bb0: {
[01:01:05]         StorageLive(_1);
[01:01:05]         goto -> bb1;
[01:01:05]     bb1: {
[01:01:05]     bb1: {
[01:01:05]         falseUnwind -> [real: bb2, cleanup: bb3];
[01:01:05]     bb2: {
[01:01:05]         _1 = const true;
[01:01:05]         _1 = const true;
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = &'26_1rs _1;
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = _1;
[01:01:05]         switchInt(move _5) -> [false: bb5, otherwise: bb4];
[01:01:05]     bb3: {
[01:01:05] ... (elided)
[01:01:05]     }
[01:01:05]     bb4: {
[01:01:05]     bb4: {
[01:01:05]         _0 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         EndRegion('26_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05]     bb5: {
[01:01:05]         _4 = ();
[01:01:05]         _4 = ();
[01:01:05]         StorageDead(_5);
[01:01:05]         StorageLive(_7);
[01:01:05]         _7 = &'26_3rs _1;
[01:01:05]         _2 = ();
[01:01:05]         EndRegion('26_3rs);
[01:01:05]         StorageDead(_7);
[01:01:05]         EndRegion('26_1rs);
[01:01:05]         StorageDead(_3);
[01:01:05]         goto -> bb1;
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]             scope 5 {
[01:01:05]             scope 6 {
[01:01:05]             scope 6 {
[01:01:05]                 let _7: &'26_3rs bool;
[01:01:05]         }
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _3: &'26_1rs bool;
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let mut _1: bool;
[01:01:05]     }
---
[01:01:05]     }
[01:01:05]     bb13: {
[01:01:05]         goto -> bb6;
[01:01:05]     }
[01:01:05]     bb14: {
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageLive(_7);
[01:01:05]         _7 = &_2;
[01:01:05]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:01:05]     bb15: {
[01:01:05]         goto -> bb16;
[01:01:05]     }
[01:01:05]     bb16: {
---
[01:01:05]     }
[01:01:05]     bb19: {
[01:01:05]         goto -> bb20;
[01:01:05]     }
[01:01:05]     bb20: {
[01:01:05]         StorageDead(_3);
[01:01:05]         goto -> bb21;
[01:01:05]     bb21: {
[01:01:05]         goto -> bb22;
[01:01:05]     }
[01:01:05]     bb22: {
[01:01:05]     bb22: {
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb23;
[01:01:05]     bb23: {
[01:01:05]         goto -> bb24;
[01:01:05]     }
[01:01:05]     bb24: {
---
[01:01:05]     bb26: {
[01:01:05]         _5 = ();
[01:01:05]         unreachable;
[01:01:05]     }
[01:01:05]     bb27: {
[01:01:05]         StorageDead(_5);
[01:01:05]         goto -> bb14;
[01:01:05]     bb28: {
[01:01:05]     bb28: {
[01:01:05]         StorageDead(_7);
[01:01:05]         _1 = ();
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb1;
[01:01:05]     bb29: {
[01:01:05]         return;
[01:01:05]     }
[01:01:05] }
[01:01:05] }
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]         let _2: i32;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     let mut _1: ();
[01:01:05]     let mut _3: bool;
[01:01:05]     let mut _4: u8;
[01:01:05]     let mut _5: !;
[01:01:05]     let mut _6: ();
[01:01:05]     let mut _7: &i32;
[01:01:05]     bb0: {                              
[01:01:05]         goto -> bb1;
[01:01:05]     }
[01:01:05]     bb1: {                              
[01:01:05]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         goto -> bb29;
[01:01:05]     }
[01:01:05]     bb3: {                              
[01:01:05]         StorageLive(_2);
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = const true;
[01:01:05]         _4 = discriminant(_3);
[01:01:05]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[01:01:05]     bb4: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb5: {                              
[01:01:05]         _2 = const 4i32;
[01:01:05]         goto -> bb14;
[01:01:05]     }
[01:01:05]     bb6: {                              
[01:01:05]         _0 = ();
[01:01:05]         goto -> bb15;
[01:01:05]     }
[01:01:05]     bb7: {                              
[01:01:05]         falseEdges -> [real: bb12, imaginary: bb8];
[01:01:05]     }
[01:01:05]     bb8: {                              
[01:01:05]         falseEdges -> [real: bb13, imaginary: bb9];
[01:01:05]     }
[01:01:05]     bb9: {                              
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb10: {                             
[01:01:05]         goto -> bb8;
[01:01:05]     }
[01:01:05]     bb11: {                             
[01:01:05]         goto -> bb7;
[01:01:05]     }
[01:01:05]     bb12: {                             
[01:01:05]         goto -> bb5;
[01:01:05]     }
[01:01:05]     bb13: {                             
[01:01:05]         goto -> bb6;
[01:01:05]     }
[01:01:05]     bb14: {                             
[01:01:05]         ReadForMatch(_2);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageLive(_7);
[01:01:05]         _7 = &_2;
[01:01:05]         _6 = const std::mem::drop(move _7) -> [return: bb28, unwind: bb4];
[01:01:05]     }
[01:01:05]     bb15: {                             
[01:01:05]         goto -> bb16;
[01:01:05]     }
[01:01:05]     bb16: {                             
[01:01:05]         goto -> bb17;
[01:01:05]     }
[01:01:05]     bb17: {                             
[01:01:05]         goto -> bb18;
[01:01:05]     }
[01:01:05]     bb18: {                             
[01:01:05]         goto -> bb19;
[01:01:05]     }
[01:01:05]     bb19: {                             
[01:01:05]         goto -> bb20;
[01:01:05]     }
[01:01:05]     bb20: {                             
[01:01:05]         StorageDead(_3);
[01:01:05]         goto -> bb21;
[01:01:05]     }
[01:01:05]     bb21: {                             
[01:01:05]         goto -> bb22;
[01:01:05]     }
[01:01:05]     bb22: {                             
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb23;
[01:01:05]     }
[01:01:05]     bb23: {                             
[01:01:05]         goto -> bb24;
[01:01:05]     }
[01:01:05]     bb24: {                             
[01:01:05]         goto -> bb25;
[01:01:05]     }
[01:01:05]     bb25: {                             
[01:01:05]         goto -> bb2;
[01:01:05]     }
[01:01:05]     bb26: {                             
[01:01:05]         _5 = ();
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb27: {                             
[01:01:05]         StorageDead(_5);
[01:01:05]         goto -> bb14;
[01:01:05]     }
[01:01:05]     bb28: {                             
[01:01:05]         StorageDead(_7);
[01:01:05]         _1 = ();
[01:01:05]         StorageDead(_2);
[01:01:05]         goto -> bb1;
[01:01:05]     }
[01:01:05]     bb29: {                             
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:    bb5: { // The loop body (body_block)
[01:01:05] Actual Line: "        ReadForMatch(_5);"
[01:01:05] Expected Line: "       StorageDead(_5);"
[01:01:05] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]    bb1: { // The cleanup block
[01:01:05]        resume;
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05]    bb3: { // Entry into the loop
[01:01:05]        _1 = ();
[01:01:05]        goto -> bb4;
[01:01:05]    }
[01:01:05]    bb4: { // The loop_block
[01:01:05]        falseUnwind -> [real: bb5, cleanup: bb1];
[01:01:05]    }
[01:01:05]    bb5: { // The loop body (body_block)
[01:01:05]        StorageLive(_5);
[01:01:05]        _5 = const 1i32;
[01:01:05]        StorageDead(_5);
[01:01:05]        goto -> bb4;
[01:01:05] ... (elided)
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]         let _5: i32;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     let mut _1: ();
[01:01:05]     let mut _2: !;
[01:01:05]     let mut _3: !;
[01:01:05]     let mut _4: ();
[01:01:05]     bb0: {                              
[01:01:05]         switchInt(const true) -> [false: bb3, otherwise: bb2];
[01:01:05]     bb1: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         _0 = ();
[01:01:05]         return;
[01:01:05]     }
[01:01:05]     bb3: {                              
[01:01:05]         _1 = ();
[01:01:05]         goto -> bb4;
[01:01:05]     }
[01:01:05]     bb4: {                              
[01:01:05]         falseUnwind -> [real: bb5, cleanup: bb1];
[01:01:05]     }
[01:01:05]     bb5: {                              
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = const 1i32;
[01:01:05]         ReadForMatch(_5);
[01:01:05]         StorageDead(_5);
[01:01:05]         goto -> bb4;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:01:05] Expected Line: "| \'_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}"
[01:01:05] Test Name: rustc.main.nll.0.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05] | '_#2r    | U0 | {bb2[0..=3], bb3[0..=1]}
[01:01:05] | '_#3r    | U0 | {bb2[1..=3], bb3[0..=1]}
[01:01:05] | '_#4r    | U0 | {bb2[3], bb3[0..=1]}
[01:01:05] Actual:
[01:01:05] | Free Region Mapping
[01:01:05] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:01:05] | '_#1r    | Local    | ['_#1r]
[01:01:05] | Inferred Region Values
[01:01:05] | Inferred Region Values
[01:01:05] | '_#0r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#0r, '_#1r}
[01:01:05] | '_#1r    | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], '_#1r}
[01:01:05] | '_#2r    | U0 | {bb2[0..=5], bb3[0..=1]}
[01:01:05] | '_#3r    | U0 | {bb2[1..=5], bb3[0..=1]}
[01:01:05] | '_#4r    | U0 | {bb2[4..=5], bb3[0..=1]}
[01:01:05] | Inference Constraints
[01:01:05] | Inference Constraints
[01:01:05] | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:01:05] | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:01:05] | '_#2r live at {bb2[0]}
[01:01:05] | '_#3r live at {bb2[1..=3]}
[01:01:05] | '_#4r live at {bb2[4..=5], bb3[0..=1]}
[01:01:05] | '_#2r: '_#3r due to Interesting(bb2[0])
[01:01:05] | '_#3r: '_#4r due to Interesting(bb2[3])
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]             scope 5 {
[01:01:05]             scope 6 {
[01:01:05]             scope 6 {
[01:01:05]                 let _6: &'_#4r usize;
[01:01:05]         }
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _2: &'_#3r usize;
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let mut _1: [usize; 3];
[01:01:05]     let mut _3: usize;
[01:01:05]     let mut _4: usize;
[01:01:05]     let mut _4: usize;
[01:01:05]     let mut _5: bool;
[01:01:05]     let mut _7: bool;
[01:01:05]     let mut _8: usize;
[01:01:05]     let mut _9: bool;
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = [const 1usize, const 2usize, const 3usize];
[01:01:05]         ReadForMatch(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = const 0usize;
[01:01:05]         _4 = Len(_1);
[01:01:05]         _5 = Lt(_3, _4);
[01:01:05]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:01:05]     bb1: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         _2 = &'_#2r _1[_3];
[01:01:05]         ReadForMatch(_2);
[01:01:05]         StorageLive(_6);
[01:01:05]         _6 = _2;
[01:01:05]         ReadForMatch(_6);
[01:01:05]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:01:05]     }
[01:01:05]     bb3: {                              
[01:01:05]         StorageLive(_8);
[01:01:05]         _8 = (*_6);
[01:01:05]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:01:05]     }
[01:01:05]     bb4: {                              
[01:01:05]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:01:05]     }
[01:01:05]     bb5: {                              
[01:01:05]         StorageDead(_8);
[01:01:05]         _0 = ();
[01:01:05]         goto -> bb7;
[01:01:05]     }
[01:01:05]     bb6: {                              
[01:01:05]         _0 = ();
[01:01:05]         goto -> bb7;
[01:01:05]     }
[01:01:05]     bb7: {                              
[01:01:05]         nop;
[01:01:05]         StorageDead(_6);
[01:01:05]         nop;
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/storage_ranges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block: None
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "        StorageLive(_3);"
[01:01:05] Test Name: rustc.main.TypeckMir.before.mir
[01:01:05] ... (elided)
[01:01:05]     bb0: {
[01:01:05]     bb0: {
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = const 0i32;
[01:01:05]         StorageLive(_3);
[01:01:05]         StorageLive(_4);
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = _1;
[01:01:05]         _4 = std::option::Option<i32>::Some(move _5,);
[01:01:05]         StorageDead(_5);
[01:01:05]         _3 = &_4;
[01:01:05]         _2 = ();
[01:01:05]         StorageDead(_4);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageLive(_6);
[01:01:05]         _6 = const 1i32;
[01:01:05]         _0 = ();
[01:01:05]         StorageDead(_6);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]         scope 4 {
[01:01:05]         scope 4 {
[01:01:05]             let _3: &std::option::Option<i32>;
[01:01:05]         scope 5 {
[01:01:05]         }
[01:01:05]         scope 6 {
[01:01:05]             let _6: i32;
[01:01:05]             let _6: i32;
[01:01:05]         }
[01:01:05]     }
[01:01:05]     scope 2 {
[01:01:05]         let _1: i32;
[01:01:05]     }
[01:01:05]     let mut _2: ();
[01:01:05]     let _4: std::option::Option<i32>;
[01:01:05]     let mut _5: i32;
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         _1 = const 0i32;
[01:01:05]         ReadForMatch(_1);
[01:01:05]         StorageLive(_3);
[01:01:05]         StorageLive(_4);
[01:01:05]         StorageLive(_5);
[01:01:05]         _5 = _1;
[01:01:05]         _4 = std::option::Option<i32>::Some(move _5,);
[01:01:05]         StorageDead(_5);
[01:01:05]         _3 = &_4;
[01:01:05]         ReadForMatch(_3);
[01:01:05]         _2 = ();
[01:01:05]         StorageDead(_4);
[01:01:05]         StorageDead(_3);
[01:01:05]         StorageLive(_6);
[01:01:05]         _6 = const 1i32;
[01:01:05]         ReadForMatch(_6);
[01:01:05]         _0 = ();
[01:01:05]         StorageDead(_6);
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/validate_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:01:05] Actual Line: "        ReadForMatch(_3);"
[01:01:05] Expected Line: "        _0 = (*_3);"
[01:01:05] Test Name: rustc.main-{{closure}}.EraseRegions.after.mir
[01:01:05] ... (elided)
[01:01:05] ... (elided)
[01:01:05] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32 {
[01:01:05] ... (elided)
[01:01:05]     bb0: {
[01:01:05]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:01:05]         StorageLive(_3);
[01:01:05]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 }))), [(*_2): i32]);
[01:01:05]         _3 = &ReErased (*_2);
[01:01:05]         Validate(Acquire, [(*_3): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 })) (imm)]);
[01:01:05]         _0 = (*_3);
[01:01:05]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 })));
[01:01:05]         StorageDead(_3);
[01:01:05]         return;
[01:01:05] }
[01:01:05] Actual:
[01:01:05] Actual:
[01:01:05] fn main::{{closure}}(_1: &ReErased [closure@NodeId(50)], _2: &ReErased mut i32) -> i32{
[01:01:05]     let mut _0: i32;
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _3: &ReErased i32;
[01:01:05]     }
[01:01:05]     bb0: {                              
[01:01:05]         Validate(Acquire, [_1: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrEnv) [closure@NodeId(50)], _2: &ReFree(DefId(0/1:11 ~ validate_1[317d]::main[0]::{{closure}}[0]), BrAnon(0)) mut i32]);
[01:01:05]         StorageLive(_3);
[01:01:05]         Validate(Suspend(ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 }))), [(*_2): i32]);
[01:01:05]         _3 = &ReErased (*_2);
[01:01:05]         Validate(Acquire, [(*_3): i32/ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 })) (imm)]);
[01:01:05]         ReadForMatch(_3);
[01:01:05]         _0 = (*_3);
[01:01:05]         EndRegion(ReScope(Remainder(BlockRemainder { block: ItemLocalId(25), first_statement_index: 0 })));
[01:01:05]         StorageDead(_3);
[01:01:05]         return;
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:01:05] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/validate_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block: None
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "        _0 = ();"
[01:01:05] Test Name: rustc.main.EraseRegions.after.mir
[01:01:05] ... (elided)
[01:01:05] fn main() -> () {
[01:01:05] ... (elided)
[01:01:05]     bb1: {
[01:01:05]     bb1: {
[01:01:05]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:05]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:05]         _1 = move _2 as std::boxed::Box<[i32]> (Unsize);
[01:01:05]         Validate(Acquire, [_1: std::boxed::Box<[i32]>]);
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_3);
[01:01:05]         _0 = ();
[01:01:05]         Validate(Release, [_1: std::boxed::Box<[i32]>]);
[01:01:05]         drop(_1) -> [return: bb2, unwind: bb3];
[01:01:05] ... (elided)
[01:01:05] }
[01:01:05] Actual:
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]     scope 2 {
[01:01:05]     scope 2 {
[01:01:05]         let _1: std::boxed::Box<[i32]> as Canonical { variables: [], value: std::boxed::Box<[i32]> };
[01:01:05]     }
[01:01:05]     let mut _2: std::boxed::Box<[i32; 3]>;
[01:01:05]     let mut _3: [i32; 3];
[01:01:05]     bb0: {                              
[01:01:05]         StorageLive(_1);
[01:01:05]         StorageLive(_2);
[01:01:05]         StorageLive(_3);
[01:01:05]         _3 = [const 1i32, const 2i32, const 3i32];
[01:01:05]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>, _3: [i32; 3]]);
[01:01:05]         _2 = const <std::boxed::Box<T>>::new(move _3) -> bb1;
[01:01:05]     }
[01:01:05]     bb1: {                              
[01:01:05]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:05]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:05]         _1 = move _2 as std::boxed::Box<[i32]> (Unsize);
[01:01:05]         Validate(Acquire, [_1: std::boxed::Box<[i32]>]);
[01:01:05]         StorageDead(_2);
[01:01:05]         StorageDead(_3);
[01:01:05]         ReadForMatch(_1);
[01:01:05]         _0 = ();
[01:01:05]         Validate(Release, [_1: std::boxed::Box<[i32]>]);
[01:01:05]         drop(_1) -> [return: bb2, unwind: bb3];
[01:01:05]     }
[01:01:05]     bb2: {                              
[01:01:05]         StorageDead(_1);
[01:01:05]         return;
[01:01:05]     bb3: {
[01:01:05]         resume;
[01:01:05]     }
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] }', tools/compiletest/src/runtest.rs:2860:13
[01:01:05] 
[01:01:05] ---- [mir-opt] mir-opt/validate_3.rs stdout ----
[01:01:05] thread '[mir-opt] mir-opt/validate_3.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:05] Current block:     bb0: {
[01:01:05] Actual Line: "        ReadForMatch(_1);"
[01:01:05] Expected Line: "        StorageLive(_2);"
[01:01:05] Test Name: rustc.main.EraseRegions.after.mir
[01:01:05] ... (elided)
[01:01:05] fn main() -> (){
[01:01:05] fn main() -> (){
[01:01:05]     let mut _0: ();
[01:01:05]     scope 1 {
[01:01:05]         scope 3 {
[01:01:05]         scope 4 {
---
[01:01:05] test result: FAILED. 24 passed; 21 failed; 0 ignored; 0 measured; 0 filtered out
[01:01:05] 
[01:01:05] 
[01:01:05] 
[01:01:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:05] 
[01:01:05] 
[01:01:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:05] Build completed unsuccessfully in 0:15:52
[01:01:05] Build completed unsuccessfully in 0:15:52
[01:01:05] make: *** [check] Error 1
[01:01:05] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037a31cb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:027dadd3:start=1536857704288337583,finish=1536857704293313782,duration=4976199
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034b746c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:131a9d03
travis_time:start:131a9d03
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f892f22
$ dmesg | grep -i kill
