plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:36] 
[00:56:36] running 51 tests
[00:56:37] ERROR 2018-07-04T02:19:07Z: compiletest::runtest: Some("    bb6: {")
[00:56:37] ERROR 2018-07-04T02:19:08Z: compiletest::runtest: Some("    bb4: {")
[00:56:41] ERROR 2018-07-04T02:19:11Z: compiletest::runtest: Some("    bb4: {")
[00:56:41] ERROR 2018-07-04T02:19:11Z: compiletest::runtest: Some("    bb4: {")
[00:56:41] ERROR 2018-07-04T02:19:11Z: compiletest::runtest: Some("    bb4: {")
[00:56:41] ERROR 2018-07-04T02:19:11Z: compiletest::runtest: Some("    bb4: {")
[00:56:43] ERROR 2018-07-04T02:19:13Z: compiletest::runtest: Some("   bb4: {")
[00:56:43] ERROR 2018-07-04T02:19:14Z: compiletest::runtest: Some("    bb2: {")
[00:56:44] ERROR 2018-07-04T02:19:14Z: compiletest::runtest: None
[00:56:51] ERROR 2018-07-04T02:19:21Z: compiletest::runtest: Some("    bb2: {")
[00:56:54] ERROR 2018-07-04T02:19:24Z: compiletest::runtest: None
[00:56:58] .FF.........FFFF..F..F..F.............F.....F......
[00:56:58] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:58] 
[00:56:58] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[00:56:58] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[00:56:58] thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:56:58] Current block:     bb6: {
[00:56:58] Actual Line: "        StorageDead(_6);"
[00:56:58] Expected Line: "        _0 = ();"
[00:56:58] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[00:56:58] Expected:
[00:56:58] ... (elided)
[00:56:58]     bb0: {
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = const false;
[00:56:58]         StorageLive(_2);
[00:56:58]         StorageLive(_3);
[00:56:58]         _3 = _1;
[00:56:58]         _2 = move _3;
[00:56:58]         StorageDead(_3);
[00:56:58]         StorageLive(_4);
[00:56:58]         UserAssertTy(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }, _4);
[00:56:58]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[00:56:58]         StorageLive(_5);
[00:56:58]         StorageLive(_6);
[00:56:58]         _6 = move _4;
[00:56:58]         replace(_5 <-move _6) -> [return: bb2, unwind: bb5];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     bb2: {
[00:56:58]     bb2: {
[00:56:58]         drop(_6) -> [return: bb6, unwind: bb4];
[00:56:58]     bb3: {
[00:56:58]         drop(_4) -> bb1;
[00:56:58]     }
[00:56:58]     bb4: {
[00:56:58]     bb4: {
[00:56:58]         drop(_5) -> bb3;
[00:56:58]     }
[00:56:58]     bb5: {
[00:56:58]         drop(_6) -> bb4;
[00:56:58]     }
[00:56:58]     bb6: {
[00:56:58]         StorageDead(_6);
[00:56:58]         _0 = ();
[00:56:58]         drop(_5) -> [return: bb7, unwind: bb3];
[00:56:58]     bb7: {
[00:56:58]     bb7: {
[00:56:58]         StorageDead(_5);
[00:56:58]         drop(_4) -> [return: bb8, unwind: bb1];
[00:56:58]     bb8: {
[00:56:58]     bb8: {
[00:56:58]         StorageDead(_4);
[00:56:58]         StorageDead(_2);
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] Actual:
[00:56:58] fn main() -> (){
[00:56:58] fn main() -> (){
[00:56:58]     let mut _0: ();
[00:56:58]     scope 1 {
[00:56:58]         scope 3 {
[00:56:58]             scope 5 {
[00:56:58]                 scope 7 {
[00:56:58]                 scope 8 {
[00:56:58]                 scope 8 {
[00:56:58]                     let _5: std::option::Option<std::boxed::Box<u32>>;
[00:56:58]             }
[00:56:58]             scope 6 {
[00:56:58]             scope 6 {
[00:56:58]                 let _4: std::option::Option<std::boxed::Box<u32>>;
[00:56:58]         }
[00:56:58]         scope 4 {
[00:56:58]         scope 4 {
[00:56:58]             let _2: bool;
[00:56:58]     }
[00:56:58]     scope 2 {
[00:56:58]     scope 2 {
[00:56:58]         let _1: bool;
[00:56:58]     }
[00:56:58]     let mut _3: bool;
[00:56:58]     let mut _6: std::option::Option<std::boxed::Box<u32>>;
[00:56:58]     bb0: {                              
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = const false;
[00:56:58]         StorageLive(_2);
[00:56:58]         StorageLive(_3);
[00:56:58]         _3 = _1;
[00:56:58]         _2 = move _3;
[00:56:58]         StorageDead(_3);
[00:56:58]         StorageLive(_4);
[00:56:58]         UserAssertTy(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }, _4);
[00:56:58]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[00:56:58]         StorageLive(_5);
[00:56:58]         StorageLive(_6);
[00:56:58]         _6 = move _4;
[00:56:58]         replace(_5 <- move _6) -> [return: bb2, unwind: bb5];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb2: {                              
[00:56:58]         drop(_6) -> [return: bb6, unwind: bb4];
[00:56:58]     bb3: {
[00:56:58]         drop(_4) -> bb1;
[00:56:58]     }
[00:56:58]     bb4: {
[00:56:58]     bb4: {
[00:56:58]         drop(_5) -> bb3;
[00:56:58]     }
[00:56:58]     bb5: {
[00:56:58]         drop(_6) -> bb4;
[00:56:58]     }
[00:56:58]     bb6: {                              
[00:56:58]         StorageDead(_6);
[00:56:58]         StorageDead(_6);
[00:56:58]                resume;
[00:56:58]     bb2: {
[00:56:58]         _1 = move _2;
[00:56:58]         drop(_2) -> bb4;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb3: {
[00:56:58]         drop(_2) -> bb1;
[00:56:58]     }
[00:56:58]     bb4: {
[00:56:58]         StorageDead(_2);
[00:56:58]         StorageLive(_4);
[00:56:58]         _4 = move _1;
[00:56:58]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[00:56:58]     bb5: {
[00:56:58]     bb5: {
[00:56:58]         drop(_4) -> [return: bb8, unwind: bb6];
[00:56:58]     bb6: {
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     }
[00:56:58]     bb7: {
[00:56:58]     bb7: {
[00:56:58]         drop(_4) -> bb6;
[00:56:58]     }
[00:56:58]     bb8: {
[00:56:58]         StorageDead(_4);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> bb9;
[00:56:58]     bb9: {
[00:56:58]     bb9: {
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] }
[00:56:58] Actual:
[00:56:58] fn main() -> (){
[00:56:58] fn main() -> (){
[00:56:58]     let mut _0: ();
[00:56:58]     scope 1 {
[00:56:58]     scope 2 {
[00:56:58]     scope 2 {
[00:56:58]         let _1: std::boxed::Box<S>;
[00:56:58]     }
[00:56:58]     let mut _2: std::boxed::Box<S>;
[00:56:58]     let mut _3: ();
[00:56:58]     let mut _4: std::boxed::Box<S>;
[00:56:58]     bb0: {                              
[00:56:58]         StorageLive(_1);
[00:56:58]         StorageLive(_2);
[00:56:58]         _2 = Box(S);
[00:56:58]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb2: {                              
[00:56:58]         _1 = move _2;
[00:56:58]         drop(_2) -> bb4;
[00:56:58]     bb3: {
[00:56:58]         drop(_2) -> bb1;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb4: {                              
[00:56:58]         StorageDead(_2);
[00:56:58]         StorageDead(_2);
[00:56:58]         StorageLive(_4);
[00:56:58]         _4 = move _1;
[00:56:58]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[00:56:58]     }
[00:56:58]     bb5: {                              
[00:56:58]         drop(_4) -> [return: bb8, unwind: bb6];
[00:56:58]     bb6: {
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     }
[00:56:58]     bb7: {
[00:56:58]     bb7: {
[00:56:58]         drop(_4) -> bb6;
[00:56:58]     }
[00:56:58]     bb8: {                              
[00:56:58]         StorageDead(_4);
[00:56:58]         StorageDead(_4);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> bb9;
[00:56:58]     }
[00:56:58]     bb9: {                              
[00:56:58]         StorageDead(_1);
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] }', tools/compiletest/src/runtest.rs:2816:13
[00:56:58] 
[00:56:58] ---- [mir-opt] mir-opt/end_region_4.rs stdout ----
[00:56:58] ---- [mir-opt] mir-opt/end_region_4.rs stdout ----
[00:56:58] thread '[mir-opt] mir-opt/end_region_4.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:56:58] Current block:     bb4: {
[00:56:58] Actual Line: "        StorageDead(_1);"
[00:56:58] Expected Line: "        return;"
[00:56:58] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[00:56:58] Expected:
[00:56:58] ... (elided)
[00:56:58]     let mut _0: ();
[00:56:58] ... (elided)
[00:56:58]     let _6: &'26_4rs i32;
[00:56:58] ... (elided)
[00:56:58]     let _3: &'26_2rs i32;
[00:56:58] ... (elided)
[00:56:58]     let _2: i32;
[00:56:58] ... (elided)
[00:56:58]     let _1: D;
[00:56:58] ... (elided)
[00:56:58]     let mut _4: ();
[00:56:58]     let mut _5: i32;
[00:56:58]     bb0: {
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = D::{{constructor}}(const 0i32,);
[00:56:58]         StorageLive(_2);
[00:56:58]         _2 = const 0i32;
[00:56:58]         StorageLive(_3);
[00:56:58]         _3 = &'26_2rs _2;
[00:56:58]         StorageLive(_5);
[00:56:58]         _5 = (*_3);
[00:56:58]         _4 = const foo(move _5) -> [return: bb2, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     bb2: {
[00:56:58]     bb2: {
[00:56:58]         StorageDead(_5);
[00:56:58]         StorageLive(_6);
[00:56:58]         _6 = &'26_4rs _2;
[00:56:58]         _0 = ();
[00:56:58]         EndRegion('26_4rs);
[00:56:58]         StorageDead(_6);
[00:56:58]         EndRegion('26_2rs);
[00:56:58]         StorageDead(_3);
[00:56:58]         StorageDead(_2);
[00:56:58]         drop(_1) -> [return: bb4, unwind: bb1];
[00:56:58]     bb3: {
[00:56:58]     bb3: {
[00:56:58]         EndRegion('26_2rs);
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     bb4: {
[00:56:58]     bb4: {
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] Actual:
[00:56:58] fn main() -> (){
[00:56:58] fn main() -> (){
[00:56:58]     let mut _0: ();
[00:56:58]     scope 1 {
[00:56:58]         scope 3 {
[00:56:58]             scope 5 {
[00:56:58]                 scope 7 {
[00:56:58]                 scope 8 {
[00:56:58]                 scope 8 {
[00:56:58]                     let _6: &'26_4rs i32;
[00:56:58]             }
[00:56:58]             scope 6 {
[00:56:58]             scope 6 {
[00:56:58]                 let _3: &'26_2rs i32;
[00:56:58]         }
[00:56:58]         scope 4 {
[00:56:58]             let _2: i32;
[00:56:58]         }
[00:56:58]         }
[00:56:58]     }
[00:56:58]     scope 2 {
[00:56:58]         let _1: D;
[00:56:58]     }
[00:56:58]     let mut _4: ();
[00:56:58]     let mut _5: i32;
[00:56:58]     bb0: {                              
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = D::{{constructor}}(const 0i32,);
[00:56:58]         StorageLive(_2);
[00:56:58]         _2 = const 0i32;
[00:56:58]         StorageLive(_3);
[00:56:58]         _3 = &'26_2rs _2;
[00:56:58]         StorageLive(_5);
[00:56:58]         _5 = (*_3);
[00:56:58]         _4 = const foo(move _5) -> [return: bb2, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb2: {                              
[00:56:58]         StorageDead(_5);
[00:56:58]         StorageLive(_6);
[00:56:58]         _6 = &'26_4rs _2;
[00:56:58]         _0 = ();
[00:56:58]         EndRegion('26_4rs);
[00:56:58]         StorageDead(_6);
[00:56:58]         EndRegion('26_2rs);
[00:56:58]         StorageDead(_3);
[00:56:58]         StorageDead(_2);
[00:56:58]         drop(_1) -> [return: bb4, unwind: bb1];
[00:56:58]     }       StorageDead(_3);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> [return: bb4, unwind: bb1];
[00:56:58]     bb3: {
[00:56:58]     bb3: {
[00:56:58]         EndRegion('14s);
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     bb4: {
[00:56:58]     bb4: {
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] }
[00:56:58] Actual:
[00:56:58] fn main() -> (){
[00:56:58] fn main() -> (){
[00:56:58]     let mut _0: ();
[00:56:58]     scope 1 {
[00:56:58]     scope 2 {
[00:56:58]         let _1: D;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     let mut _2: ();
[00:56:58]     let mut _3: [closure@NodeId(18) d:&'14s D];
[00:56:58]     let mut _4: &'14s D;
[00:56:58]     bb0: {                              
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = D::{{constructor}}(const 0i32,);
[00:56:58]         StorageLive(_3);
[00:56:58]         StorageLive(_4);
[00:56:58]         _4 = &'14s _1;
[00:56:58]         _3 = [closure@NodeId(18)] { d: move _4 };
[00:56:58]         StorageDead(_4);
[00:56:58]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb2: {                              
[00:56:58]         EndRegion('14s);
[00:56:58]         StorageDead(_3);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> [return: bb4, unwind: bb1];
[00:56:58]     bb3: {
[00:56:58]     bb3: {
[00:56:58]         EndRegion('14s);
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     }
[00:56:58]     bb4: {                              
[00:56:58]         StorageDead(_1);
[00:56:58]     bb4: {
[00:56:58]     bb4: {
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] Actual:
[00:56:58] fn main() -> (){
[00:56:58] fn main() -> (){
[00:56:58]     let mut _0: ();
[00:56:58]     scope 1 {
[00:56:58]     scope 2 {
[00:56:58]         let _1: D;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     let mut _2: ();
[00:56:58]     let mut _3: [closure@NodeId(22) d:&'19s D];
[00:56:58]     let mut _4: &'19s D;
[00:56:58]     bb0: {                              
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = D::{{constructor}}(const 0i32,);
[00:56:58]         StorageLive(_3);
[00:56:58]         StorageLive(_4);
[00:56:58]         _4 = &'19s _1;
[00:56:58]         _3 = [closure@NodeId(22)] { d: move _4 };
[00:56:58]         StorageDead(_4);
[00:56:58]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     }
[00:56:58]     bb2: {                              
[00:56:58]         EndRegion('19s);
[00:56:58]         StorageDead(_3);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> [return: bb4, unwind: bb1];
[00:56:58]     bb3: {
[00:56:58]     bb3: {
[00:56:58]         EndRegion('19s);
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     }
[00:56:58]     bb4: {                              
[00:56:58]         StorageDead(_1);
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] }', tools/compiletest/src/runtest.rs:2816:13
[00:56:58] 
[00:56:58] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[00:56:58] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[00:56:58] thread '[mir-opt] mir-opt/end_region_7.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:56:58] Current block:     bb4: {
[00:56:58] Actual Line: "        StorageDead(_4);"
[00:56:58] Expected Line: "        _2 = const foo(move _3) -> [return: bb5, unwind: bb3];"
[00:56:58] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[00:56:58] Expected:
[00:56:58] ... (elided)
[00:56:58] fn main() -> () {
[00:56:58]     let mut _0: ();
[00:56:58] ... (elided)
[00:56:58]     let _1: D;
[00:56:58] ... (elided)
[00:56:58]     let mut _2: ();
[00:56:58]     let mut _3: [closure@NodeId(22) d:D];
[00:56:58]     let mut _4: D;
[00:56:58]     bb0: {
[00:56:58]         StorageLive(_1);
[00:56:58]         _1 = D::{{constructor}}(const 0i32,);
[00:56:58]         StorageLive(_3);
[00:56:58]         StorageLive(_4);
[00:56:58]         _4 = move _1;
[00:56:58]         _3 = [closure@NodeId(22)] { d: move _4 };
[00:56:58]         drop(_4) -> [return: bb4, unwind: bb3];
[00:56:58]     bb1: {
[00:56:58]         resume;
[00:56:58]     }
[00:56:58]     bb2: {
[00:56:58]     bb2: {
[00:56:58]         drop(_1) -> bb1;
[00:56:58]     }
[00:56:58]     bb3: {
[00:56:58]         drop(_3) -> bb2;
[00:56:58]     }
[00:56:58]     bb4: {
[00:56:58]         StorageDead(_4);
[00:56:58]         _2 = const foo(move _3) -> [return: bb5, unwind: bb3];
[00:56:58]     bb5: {
[00:56:58]     bb5: {
[00:56:58]         drop(_3) -> [return: bb6, unwind: bb2];
[00:56:58]     bb6: {
[00:56:58]     bb6: {
[00:56:58]         StorageDead(_3);
[00:56:58]         _0 = ();
[00:56:58]         drop(_1) -> [return: bb7, unwind: bb1];
[00:56:58]     58]     bb7: {                              
[00:56:58]         StorageDead(_1);
[00:56:58]         StorageDead(_1);
[00:56:58]         return;
[00:56:58] }', tools/compiletest/src/runtest.rs:2816:13
[00:56:58] 
[00:56:58] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[00:56:58] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[00:56:58] thread '[mir-opt] mir-opt/end_region_8.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:56:58] Current block:    bb4: {
[00:56:58] Actual Line: "        StorageDead(_1);"
[00:56:58] Expected Line: "       return;"
[00:56:58] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[00:56:58] Expected:
[00:56:58] ... (elided)
[00:56:58] fn main() -> () {
[00:56:58]    let mut _0: ();
[00:56:58] ... (elided)
[00:56:58]    let _2: &'21_1rs D;
[00:56:58] ... (elided)
[00:56:58]    let _1: D;
[00:56:58] ... (elided)
[00:56:58]    let mut _3: ();
[00:56:58]    let mut _4: [closure@NodeId(22) r:&'19s D];
[00:56:58]    let mut _5: &'21_1rs D;
[00:56:58]    bb0: {
[00:56:58]        StorageLive(_1);
[00:56:58]        _1 = D::{{constructor}}(const 0i32,);
[00:56:58]        StorageLive(_2);
[00:56:58]        _2 = &'21_1rs _1;
[00:56:58]        StorageLive(_4);
[00:56:58]        StorageLive(_5);
[00:56:58]        _5 = _2;
[00:56:58]        _4 = [closure@NodeId(22)] { r: move _5 };
---
[00:56:58] test result: FAILED. 40 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:58] 
[00:56:58] 
[00:56:58] 
[00:56:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:58] 
[00:56:58] 
[00:56:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:58] Build completed unsuccessfully in 0:15:01
[00:56:58] Build completed unsuccessfully in 0:15:01
[00:56:58] make: *** [check] Error 1
[00:56:58] Makefile:58: recipe for target 'check' failed
107692 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103928 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
103924 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
100572 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
