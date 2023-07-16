plain
[00:49:08] .................................................................................................... 2200/4635
[00:49:13] ....................................i............................................................... 2300/4635
[00:49:16] .................................................................................................... 2400/4635
[00:49:20] .................................................................................................... 2500/4635
[00:49:23] ..................................................iiiiiiiii......................................... 2600/4635
[00:49:30] .................................................................................................... 2800/4635
[00:49:33] .................................................................................................... 2900/4635
[00:49:37] ................................................................................i................... 3000/4635
[00:49:39] .................................................................................................... 3100/4635
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:05] 
[01:02:05] running 48 tests
[01:02:09] ERROR 2018-10-19T16:02:09Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:02:09] ERROR 2018-10-19T16:02:10Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:02:09] ERROR 2018-10-19T16:02:10Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:02:09] ERROR 2018-10-19T16:02:10Z: compiletest::runtest: Some("        _1 = D::{{constructor}}(const 0i32,);")
[01:02:10] ERROR 2018-10-19T16:02:11Z: compiletest::runtest: Some("       _1 = D::{{constructor}}(const 0i32,);")
[01:02:11] ERROR 2018-10-19T16:02:11Z: compiletest::runtest: Some("        _5 = S1::{{constructor}}(const \"ex1\",);")
[01:02:24] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:02:24] ............FFFF..F.F.............F.............
[01:02:24] 
[01:02:24] ---- [mir-opt] mir-opt/end_region_4.rs stdout ----
[01:02:24] ---- [mir-opt] mir-opt/end_region_4.rs stdout ----
[01:02:24] thread '[mir-opt] mir-opt/end_region_4.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:24] Current block:         _1 = D::{{constructor}}(const 0i32,);
[01:02:24] Actual Line: "        _1 = D(const 0i32,);"
[01:02:24] Expected Line: "        _1 = D::{{constructor}}(const 0i32,);"
[01:02:24] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:24] ... (elided)
[01:02:24] ... (elided)
[01:02:24]     let mut _0: ();
[01:02:24] ... (elided)
[01:02:24]     let _6: &'26_4rs i32;
[01:02:24] ... (elided)
[01:02:24]     let _3: &'26_2rs i32;
[01:02:24] ... (elided)
[01:02:24]     let _2: i32;
[01:02:24] ... (elided)
[01:02:24]     let _1: D;
[01:02:24] ... (elided)
[01:02:24]     let mut _4: ();
[01:02:24]     let mut _5: i32;
[01:02:24]     bb0: {
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D::{{constructor}}(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_2);
[01:02:24]         _2 = const 0i32;
[01:02:24]         FakeRead(ForLet, _2);
[01:02:24]         StorageLive(_3);
[01:02:24]         _3 = &'26_2rs _2;
[01:02:24]         FakeRead(ForLet, _3);
[01:02:24]         StorageLive(_5);
[01:02:24]         _5 = (*_3);
[01:02:24]         _4 = const foo(move _5) -> [return: bb2, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     bb2: {
[01:02:24]     bb2: {
[01:02:24]         StorageDead(_5);
[01:02:24]         StorageLive(_6);
[01:02:24]         _6 = &'26_4rs _2;
[01:02:24]         FakeRead(ForLet, _6);
[01:02:24]         _0 = ();
[01:02:24]         EndRegion('26_4rs);
[01:02:24]         StorageDead(_6);
[01:02:24]         EndRegion('26_2rs);
[01:02:24]         StorageDead(_3);
[01:02:24]         StorageDead(_2);
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('26_2rs);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] Actual:
[01:02:24] fn main() -> (){
[01:02:24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]         scope 3 {
[01:02:24]             scope 5 {
[01:02:24]                 scope 7 {
[01:02:24]                 scope 8 {
[01:02:24]                 scope 8 {
[01:02:24]                     let _6: &'26_4rs i32;
[01:02:24]             }
[01:02:24]      eDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('14s);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }
[01:02:24] Actual:
[01:02:24] fn main() -> (){
[01:02:24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]     scope 2 {
[01:02:24]         let _1: D;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     let mut _2: ();
[01:02:24]     let mut _3: [closure@NodeId(18) d:&'14s D];
[01:02:24]     let mut _4: &'14s D;
[01:02:24]     bb0: {                              
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_3);
[01:02:24]         StorageLive(_4);
[01:02:24]         _4 = &'14s _1;
[01:02:24]         _3 = [closure@NodeId(18)] { d: move _4 };
[01:02:24]         StorageDead(_4);
[01:02:24]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {                              
[01:02:24]         EndRegion('14s);
[01:02:24]         StorageDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('14s);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {                              
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }', tools/compiletest/src/runtest.rs:2939:13
[01:02:24] 
[01:02:24] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[01:02:24] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[01:02:24] thread '[mir-opt] mir-opt/end_region_6.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:24] Current block:         _1 = D::{{constructor}}(const 0i32,);
[01:02:24] Actual Line: "        _1 = D(const 0i32,);"
[01:02:24] Expected Line: "        _1 = D::{{constructor}}(const 0i32,);"
[01:02:24] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:24] ... (elided)
[01:02:24] fn main() -> () {
[01:02:24] fn main() -> () {
[01:02:24]     let mut _0: ();
[01:02:24] ... (elided)
[01:02:24]     let _1: D;
[01:02:24] ... (elided)
[01:02:24]     let mut _2: ();
[01:02:24]     let mut _3: [closure@NodeId(22) d:&'19s D];
[01:02:24]     let mut _4: &'19s D;
[01:02:24]     bb0: {
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D::{{constructor}}(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_3);
[01:02:24]         StorageLive(_4);
[01:02:24]         _4 = &'19s _1;
[01:02:24]         _3 = [closure@NodeId(22)] { d: move _4 };
[01:02:24]         StorageDead(_4);
[01:02:24]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     bb2: {
[01:02:24]     bb2: {
[01:02:24]         EndRegion('19s);
[01:02:24]         StorageDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('19s);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] Actual:
[01:02:24] fn main() -> (){
[01:02:24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]     scope 2 {
[01:02:24]         let _1: D;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     let mut _2: ();
[01:02:24]     let mut _3: [closure@NodeId(22) d:&'19s D];
[01:02:24]     let mut _4: &'19s D;
[01:02:24]     bb0: {                              
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_3);
[01:02:24]         StorageLive(_4);
[01:02:24]         _4 = &'19s _1;
[01:02:24]         _3 = [closure@NodeId(22)] { d: move _4 };
[01:02:24]         StorageDead(_4);
[01:02:24]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {                              
[01:02:24]         EndRegion('19s);
[01:02:24]         StorageDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('19s);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {                              
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }', tools/compiletest/src/runtest.rs:2939:13
[01:02:24] 
[01:02:24] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[01:02:24] ---- [mir-opt] mir-opt/end_region_7.rs stdout ----
[01:02:24] thread '[mir-opt] mir-opt/end_region_7.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:24] Current block:         _1 = D::{{constructor}}(const 0i32,);
[01:02:24] Actual Line: "        _1 = D(const 0i32,);"
[01:02:24] Expected Line: "        _1 = D::{{constructor}}(const 0i32,);"
[01:02:24] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:02:24] ... (elided)
[01:02:24] fn main() -> () {
[01:02:24] fn main() -> () {
[01:02:24]     let mut _0: ();
[01:02:24] ... (elided)
[01:02:24]     let _1: D;
[01:02:24] ... (elided)
[01:02:24]     let mut _2: ();
[01:02:24]     let mut _3: [closure@NodeId(22) d:D];
[01:02:24]     bb0: {
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D::{{constructor}}(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_3);
[01:02:24]         _3 = [closure@NodeId(22)] { d: move _1 };
[01:02:24]         _2 = const foo(move _3) -> [return: bb2, unwind: bb4];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     bb2: {
[01:02:24]     bb2: {
[01:02:24]         drop(_3) -> [return: bb5, unwind: bb3];
[01:02:24]     bb3: {
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         drop(_3) -> bb3;
[01:02:24]     }
[01:02:24]     bb5: {
[01:02:24]         StorageDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb6, unwind: bb1];
[01:02:24]     bb6: {
[01:02:24]     bb6: {
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }
[01:02:24] Actual:
[01:02:24] fn main() -> (){
[01:02:24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]     scope 2 {
[01:02:24]         let _1: D;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     let mut _2: ();
[01:02:24]     let mut _3: [closure@NodeId(22) d:D];
[01:02:24]     bb0: {                              
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_3);
[01:02:24]         _3 = [closure@NodeId(22)] { d: move _1 };
[01:02:24]         _2 = const foo(move _3) -> [return: bb2, unwind: bb4];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {                              
[01:02:24]         drop(_3) -> [return: bb5, unwind: bb3];
[01:02:24]     bb3: {
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         drop(_3) -> bb3;
[01:02:24]     }
[01:02:24]     bb5: {                              
[01:02:24]         StorageDead(_3);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb6, unwind: bb1];
[01:02:24]     }
[01:02:24]     bb6: {                              
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }', tools/compiletest/src/runtest.rs:2939:13
[01:02:24] 
[01:02:24] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[01:02:24] ---- [mir-opt] mir-opt/end_region_8.rs stdout ----
[01:02:24] thread '[mir-opt] mir-opt/end_region_8.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:24] Current block:        _1 = :24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]         scope 3 {
[01:02:24]         scope 4 {
[01:02:24]         scope 4 {
[01:02:24]             let _2: &'21_1rs D;
[01:02:24]     }
[01:02:24]     scope 2 {
[01:02:24]         let _1: D;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     let mut _3: ();
[01:02:24]     let mut _4: [closure@NodeId(22) r:&'19s D];
[01:02:24]     bb0: {                              
[01:02:24]         StorageLive(_1);
[01:02:24]         _1 = D(const 0i32,);
[01:02:24]         FakeRead(ForLet, _1);
[01:02:24]         StorageLive(_2);
[01:02:24]         _2 = &'21_1rs _1;
[01:02:24]         FakeRead(ForLet, _2);
[01:02:24]         StorageLive(_4);
[01:02:24]         _4 = [closure@NodeId(22)] { r: _2 };
[01:02:24]         _3 = const foo(move _4) -> [return: bb2, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {                              
[01:02:24]         EndRegion('19s);
[01:02:24]         StorageDead(_4);
[01:02:24]         _0 = ();
[01:02:24]         EndRegion('21_1rs);
[01:02:24]         StorageDead(_2);
[01:02:24]         drop(_1) -> [return: bb4, unwind: bb1];
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         EndRegion('19s);
[01:02:24]         EndRegion('21_1rs);
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {                              
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24] }', tools/compiletest/src/runtest.rs:2939:13
[01:02:24] 
[01:02:24] ---- [mir-opt] mir-opt/end_region_destruction_extents_1.rs stdout ----
[01:02:24] ---- [mir-opt] mir-opt/end_region_destruction_extents_1.rs stdout ----
[01:02:24] thread '[mir-opt] mir-opt/end_region_destruction_extents_1.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:02:24] Current block:         _5 = S1::{{constructor}}(const "ex1",);
[01:02:24] Actual Line: "        _5 = S1(const \"ex1\",);"
[01:02:24] Expected Line: "        _5 = S1::{{constructor}}(const \"ex1\",);"
[01:02:24] Test Name: rustc.main.QualifyAndPromoteConstants.before.mir
[01:02:24] ... (elided)
[01:02:24] fn main() -> () {
[01:02:24] fn main() -> () {
[01:02:24] let mut _0: ();
[01:02:24]     let mut _1: &'12ds S1;
[01:02:24]     let mut _2: D1<'12ds, '10s>;
[01:02:24]     let mut _3: &'12ds S1;
[01:02:24]     let mut _4: &'12ds S1;
[01:02:24]     let _5: S1;
[01:02:24]     let mut _6: &'10s S1;
[01:02:24]     let mut _7: &'10s S1;
[01:02:24]     let _8: S1;
[01:02:24]     bb0: {
[01:02:24]         StorageLive(_2);
[01:02:24]         StorageLive(_3);
[01:02:24]         StorageLive(_4);
[01:02:24]         StorageLive(_5);
[01:02:24]         _5 = S1::{{constructor}}(const "ex1",);
[01:02:24]         _4 = &'12ds _5;
[01:02:24]         _3 = &'12ds (*_4);
[01:02:24]         StorageLive(_6);
[01:02:24]         StorageLive(_7);
[01:02:24]         StorageLive(_8);
[01:02:24]         _8 = S1::{{constructor}}(const "dang1",);
[01:02:24]         _7 = &'10s _8;
[01:02:24]         _6 = &'10s (*_7);
[01:02:24]         _2 = D1<'12ds, '10s>::{{constructor}}(move _3, move _6);
[01:02:24]         EndRegion('10s);
[01:02:24]         StorageDead(_6);
[01:02:24]         StorageDead(_3);
[01:02:24]     }
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         (_1.0: Aligned) = move _4;
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     bb4: {
[01:02:24]     bb4: {
[01:02:24]         StorageDead(_6);
[01:02:24]         (_1.0: Aligned) = move _4;
[01:02:24]         StorageDead(_4);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb2, unwind: bb1];
[01:02:24] }
[01:02:24] Actual:
[01:02:24] fn main() -> (){
[01:02:24] fn main() -> (){
[01:02:24]     let mut _0: ();
[01:02:24]     scope 1 {
[01:02:24]     scope 2 {
[01:02:24]     scope 2 {
[01:02:24]         let mut _1: Packed;
[01:02:24]     let mut _2: Aligned;
[01:02:24]     let mut _2: Aligned;
[01:02:24]     let mut _3: Droppy;
[01:02:24]     let mut _4: Aligned;
[01:02:24]     let mut _5: Droppy;
[01:02:24]     let mut _6: Aligned;
[01:02:24]     bb0: {                              
[01:02:24]         StorageLive(_1);
[01:02:24]         StorageLive(_2);
[01:02:24]         StorageLive(_3);
[01:02:24]         _3 = Droppy(const 0usize,);
[01:02:24]         _2 = Aligned(move _3,);
[01:02:24]         StorageDead(_3);
[01:02:24]         _1 = Packed(move _2,);
[01:02:24]         StorageDead(_2);
[01:02:24]         StorageLive(_4);
[01:02:24]         StorageLive(_5);
[01:02:24]         _5 = Droppy(const 0usize,);
[01:02:24]         _4 = Aligned(move _5,);
[01:02:24]         StorageDead(_5);
[01:02:24]         StorageLive(_6);
[01:02:24]         _6 = move (_1.0: Aligned);
[01:02:24]         drop(_6) -> [return: bb4, unwind: bb3];
[01:02:24]     bb1: {
[01:02:24]         resume;
[01:02:24]     }
[01:02:24]     }
[01:02:24]     bb2: {                              
[01:02:24]         StorageDead(_1);
[01:02:24]         return;
[01:02:24]     bb3: {
[01:02:24]     bb3: {
[01:02:24]         (_1.0: Aligned) = move _4;
[01:02:24]         drop(_1) -> bb1;
[01:02:24]     }
[01:02:24]     bb4: {                              
[01:02:24]         StorageDead(_6);
[01:02:24]         (_1.0: Aligned) = move _4;
[01:02:24]         StorageDead(_4);
[01:02:24]         _0 = ();
[01:02:24]         drop(_1) -> [return: bb2, unwind: bb1];
[01:02:24] }', tools/compiletest/src/runtest.rs:2939:13
[01:02:24] 
[01:02:24] 
[01:02:24] failures:
