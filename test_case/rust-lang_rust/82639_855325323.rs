plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 160 tests
........F................F........i.......................F....................i...................F 100/160
...F.......F.F.............i.....F..F.....................F.
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/const_goto.rs stdout ----
---- [mir-opt] mir-opt/const_goto.rs stdout ----
4   fn issue_77355_opt(_1: Foo) -> u64 {
5       debug num => _1;                     // in scope 0 at $DIR/const_goto.rs:11:20: 11:23
6       let mut _0: u64;                     // return place in scope 0 at $DIR/const_goto.rs:11:33: 11:36
- -     let mut _2: bool;                    // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -     let mut _2: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
8 -     let mut _3: isize;                   // in scope 0 at $DIR/const_goto.rs:12:22: 12:28
9 +     let mut _2: isize;                   // in scope 0 at $DIR/const_goto.rs:12:22: 12:28

11       bb0: {
11       bb0: {
- -         StorageLive(_2);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -         StorageLive(_2);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
13 -         _3 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:12:22: 12:28
14 -         switchInt(move _3) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto.rs:12:22: 12:28
15 +         _2 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:12:22: 12:28
17       }
18   
19       bb1: {
19       bb1: {
- -         _2 = const false;                // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:321:18: 321:23
- -         goto -> bb3;                     // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -         _2 = const false;                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
22 +         _0 = const 42_u64;               // scope 0 at $DIR/const_goto.rs:12:53: 12:55
23 +         goto -> bb3;                     // scope 0 at $DIR/const_goto.rs:12:5: 12:57

25   
26       bb2: {
26       bb2: {
- -         _2 = const true;                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:320:48: 320:52
- -         goto -> bb3;                     // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -         _2 = const true;                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29 -     }
31 -     bb3: {


thread '[mir-opt] mir-opt/const_goto.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto.issue_77355_opt.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3561:25

---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
4   fn hello() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/control-flow-simplification.rs:11:14: 11:14
6       let mut _1: bool;                    // in scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
-       let mut _2: !;                       // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
+       let mut _2: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
9       bb0: {
9       bb0: {
10           StorageLive(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
15       }
16   
17       bb1: {
17       bb1: {
-           StorageLive(_2);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
-           begin_panic::<&str>(const "explicit panic"); // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
+           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+           begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
20                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:32
+                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
22                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
23                                            // ty::Const
24                                            // + ty: &str

25                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
26                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:33: 28:49
+                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
28                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
30   


thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/fn-ptr-shim.rs stdout ----
---- [mir-opt] mir-opt/fn-ptr-shim.rs stdout ----
1 // MIR for `std::ops::Fn::call` before AddMovesForPackedDrops
2 
3 fn std::ops::Fn::call(_1: *const fn(), _2: ()) -> <fn() as FnOnce<()>>::Output {
-     let mut _0: <fn() as std::ops::FnOnce<()>>::Output; // return place in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ops/function.rs:70:5: 70:67
+     let mut _0: <fn() as std::ops::FnOnce<()>>::Output; // return place in scope 0 at $SRC_DIR/core/src/ops/function.rs:LL:COL
6     bb0: {
6     bb0: {
-         _0 = move (*_1)() -> bb1;        // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ops/function.rs:70:5: 70:67
+         _0 = move (*_1)() -> bb1;        // scope 0 at $SRC_DIR/core/src/ops/function.rs:LL:COL
9 
10     bb1: {


-         return;                          // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ops/function.rs:70:5: 70:67
+         return;                          // scope 0 at $SRC_DIR/core/src/ops/function.rs:LL:COL
13 }
14 


thread '[mir-opt] mir-opt/fn-ptr-shim.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/fn_ptr_shim.core.ops-function-Fn-call.AddMovesForPackedDrops.before.mir', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
9       let _4: i32;                         // in scope 0 at $DIR/issue-73223.rs:3:14: 3:15
10       let mut _5: !;                       // in scope 0 at $DIR/issue-73223.rs:4:17: 4:23
11       let mut _7: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
-       let _8: ();                          // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:9: 47:10
-       let mut _9: (&i32, &i32);            // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:15: 37:32
-       let mut _10: &i32;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:16: 37:22
-       let mut _11: &i32;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
+       let _8: ();                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _9: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _10: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
16       let _12: i32;                        // in scope 0 at $DIR/issue-73223.rs:8:23: 8:24
-       let mut _15: bool;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:20: 39:46
-       let mut _16: bool;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:21: 39:46
-       let mut _17: i32;                    // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:22: 39:31
-       let mut _18: i32;                    // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:35: 39:45
-       let mut _19: !;                      // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:47: 45:18
-       let _21: !;                          // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:21: 44:114
-       let mut _22: core::panicking::AssertKind; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:54: 44:58
-       let mut _23: &i32;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-       let _24: &i32;                       // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-       let mut _25: &i32;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-       let _26: &i32;                       // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:85: 44:113
+       let mut _15: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _16: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _17: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _18: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _21: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _22: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _23: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _24: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _25: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _26: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29       scope 1 {
30           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
31           let _6: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
32           scope 3 {
32           scope 3 {
33               debug _prev => _6;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
-               let _13: &i32;               // in scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:14: 38:22
-               let _14: &i32;               // in scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:24: 38:33
-               let mut _28: &i32;           // in scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
+               let _13: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let _14: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _28: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
37               scope 4 {
-                   debug left_val => _13;   // in scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:14: 38:22
-                   debug right_val => _14;  // in scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:24: 38:33
-                   let _20: core::panicking::AssertKind; // in scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:40:25: 40:29
+                   debug left_val => _13;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug right_val => _14;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   let _20: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
41                   scope 5 {
-                       debug kind => _20;   // in scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:40:25: 40:29
+                       debug kind => _20;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
44               }
45           }


76           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
77           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
78           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
-           StorageLive(_8);                 // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:36:41: 48:6
-           StorageLive(_9);                 // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:15: 37:32
-           StorageLive(_10);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:16: 37:22
-           _10 = &_1;                       // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:16: 37:22
-           StorageLive(_11);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
-           _28 = const main::promoted[0];   // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
+           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85                                            // ty::Const
86                                            // + ty: &i32
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
88                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
90                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[2d0f]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
-           _11 = _28;                       // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:24: 37:31
-           (_9.0: &i32) = move _10;         // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:15: 37:32
-           (_9.1: &i32) = move _11;         // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:15: 37:32
-           StorageDead(_11);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:31: 37:32
-           StorageDead(_10);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:37:31: 37:32
-           StorageLive(_13);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:14: 38:22
-           _13 = (_9.0: &i32);              // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:14: 38:22
-           StorageLive(_14);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:24: 38:33
-           _14 = (_9.1: &i32);              // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:38:24: 38:33
-           StorageLive(_15);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:20: 39:46
-           StorageLive(_16);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:21: 39:46
-           StorageLive(_17);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:22: 39:31
-           _17 = (*_13);                    // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:22: 39:31
-           StorageLive(_18);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:35: 39:45
-           _18 = const 1_i32;               // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:35: 39:45
-           _16 = Eq(move _17, const 1_i32); // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:21: 39:46
-           StorageDead(_18);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:45: 39:46
-           StorageDead(_17);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:45: 39:46
-           _15 = Not(move _16);             // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:20: 39:46
-           StorageDead(_16);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:45: 39:46
-           switchInt(move _15) -> [false: bb4, otherwise: bb3]; // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:39:17: 45:18
+           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_13);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _13 = (_9.0: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _14 = (_9.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _17 = (*_13);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _18 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = Eq(move _17, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _15) -> [false: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
113   
114       bb3: {


-           StorageLive(_20);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:40:25: 40:29
-           discriminant(_20) = 0;           // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:40:32: 40:65
-           StorageLive(_21);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:21: 44:114
-           StorageLive(_22);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:54: 44:58
-           _22 = const core::panicking::AssertKind::Eq; // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:54: 44:58
+           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120                                            // ty::Const
121                                            // + ty: core::panicking::AssertKind
122                                            // + val: Value(Scalar(0x00))
123                                            // mir::Constant
123                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:54: 44:58
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
-           StorageLive(_23);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-           StorageLive(_24);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-           _24 = _13;                       // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-           _23 = _24;                       // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:60: 44:70
-           StorageLive(_25);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-           StorageLive(_26);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-           _26 = _14;                       // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-           _25 = _26;                       // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:72: 44:83
-           StorageLive(_27);                // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:85: 44:113
-           discriminant(_27) = 0;           // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:85: 44:113
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:21: 44:114
+           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_24);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _24 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _23 = _24;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_25);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_26);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _26 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _25 = _26;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
137                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:21: 44:53
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
139                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
140                                            // ty::Const
141                                            // + ty: core::panicking::AssertKind

142                                            // + val: Value(Scalar(0x00))
143                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:44:21: 44:114
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
145                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
147   

148       bb4: {
148       bb4: {
-           nop;                             // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:45:18: 45:18
-           StorageDead(_15);                // scope 4 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:45:17: 45:18
-           StorageDead(_14);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:46:13: 46:14
-           StorageDead(_13);                // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:46:13: 46:14
-           StorageDead(_9);                 // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:48:5: 48:6
-           StorageDead(_8);                 // scope 3 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:48:5: 48:6
+           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_13);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
155           nop;                             // scope 0 at $DIR/issue-73223.rs:1:11: 9:2
156           StorageDead(_6);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
157           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/issue_76432.rs stdout ----
---- [mir-opt] mir-opt/issue_76432.rs stdout ----
21       let mut _19: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:54: 9:68
22       let mut _20: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
23       let mut _21: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
-       let mut _22: !;                      // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/panic.rs:18:9: 18:39
+       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
25       scope 1 {
26           debug v => _2;                   // in scope 1 at $DIR/issue_76432.rs:7:9: 7:10
27           let _13: &T;                     // in scope 1 at $DIR/issue_76432.rs:9:10: 9:16
64       }
65   
66       bb1: {
66       bb1: {
-           StorageLive(_22);                // scope 1 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/panic.rs:18:9: 18:39
-           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/panic.rs:18:9: 18:39
+           StorageLive(_22);                // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
+           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
69                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/panic.rs:18:9: 18:33
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
71                                            // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
72                                            // ty::Const
73                                            // + ty: &str

74                                            // + val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 })
75                                            // mir::Constant
-                                            // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:585:24: 585:66
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
77                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 }) }
79   


thread '[mir-opt] mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
5     let mut _0: T;                       // return place in scope 0 at $DIR/no-drop-for-inactive-variant.rs:7:33: 7:34
6     let mut _2: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:9:9: 9:16
7     let _3: T;                           // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:9:14: 9:15
-     let mut _4: !;                       // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
+     let mut _4: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
9     let mut _5: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
10     let mut _6: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
11     let mut _7: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
19     }
20 
21     bb1: {
21     bb1: {
-         StorageLive(_4);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
-         begin_panic::<&str>(const "explicit panic") -> bb4; // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:50
+         StorageLive(_4);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+         begin_panic::<&str>(const "explicit panic") -> bb4; // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
24                                          // mir::Constant
-                                          // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:9: 28:32
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
26                                          // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
27                                          // ty::Const
28                                          // + ty: &str

29                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
30                                          // mir::Constant
-                                          // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/std/src/panic.rs:28:33: 28:49
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
32                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
34 


thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_drop_for_inactive_variant.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
13 -     }
14 - 
15 -     bb1: {
15 -     bb1: {
- -         goto -> bb3;                     // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
17 -     }
19 -     bb2: {


- -         goto -> bb3;                     // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/macros/mod.rs:319:9: 322:10
+ -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21 -     }
23 -     bb3: {


thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.foo.MatchBranchSimplification.64bit.diff', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/retag.rs stdout ----
---- [mir-opt] mir-opt/retag.rs stdout ----
1 // MIR for `std::ptr::drop_in_place` after SimplifyCfg-make_shim
2 
3 fn std::ptr::drop_in_place(_1: *mut Test) -> () {
-     let mut _0: ();                      // return place in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _2: &mut Test;               // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _3: ();                      // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _2: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
8     bb0: {
8     bb0: {
-         Retag([raw] _1);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-         _2 = &mut (*_1);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-         _3 = <Test as Drop>::drop(move _2) -> bb1; // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         Retag([raw] _1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         _3 = <Test as Drop>::drop(move _2) -> bb1; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
12                                          // mir::Constant
-                                          // + span: /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
14                                          // + literal: Const { ty: for<'r> fn(&'r mut Test) {<Test as std::ops::Drop>::drop}, val: Value(Scalar(<ZST>)) }
16 

17     bb1: {
17     bb1: {
-         return;                          // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
20 }
21 


thread '[mir-opt] mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.core.ptr-drop_in_place.Test.SimplifyCfg-make_shim.after.mir', src/tools/compiletest/src/runtest.rs:3561:25
---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
1 // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
3 fn std::ptr::drop_in_place(_1: *mut [String]) -> () {
-     let mut _0: ();                      // return place in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _2: usize;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _3: usize;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _4: usize;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _5: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _6: bool;                    // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _7: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _8: bool;                    // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _9: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _10: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _11: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _12: bool;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _13: *mut std::string::String; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _14: bool;                   // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-     let mut _15: *mut [std::string::String]; // in scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _3: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _4: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _5: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _6: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _7: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _9: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _10: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _11: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _13: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _14: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+     let mut _15: *mut [std::string::String]; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
20     bb0: {
20     bb0: {
-         goto -> bb15;                    // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         goto -> bb15;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
23 
24     bb1: {


-         return;                          // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
27 
27 
28     bb2 (cleanup): {

-         resume;                          // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         resume;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
31 
31 
32     bb3 (cleanup): {

-         _5 = &raw mut (*_1)[_4];         // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-         _4 = Add(move _4, const 1_usize); // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-         drop((*_5)) -> bb4;              // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         _5 = &raw mut (*_1)[_4];         // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         _4 = Add(move _4, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         drop((*_5)) -> bb4;              // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
37 
37 
38     bb4 (cleanup): {

-         _6 = Eq(_4, _3);                 // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
-         switchInt(move _6) -> [false: bb3, otherwise: bb2]; // scope 0 at /rustc/34b9932f5c0f519d6b9b9f95f21723142c5dc877/library/core/src/ptr/mod.rs:192:1: 192:56
+         _6 = Eq(_4, _3);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         switchInt(move _6) -> [false: bb3, otherwise: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
---
test result: FAILED. 147 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:57
