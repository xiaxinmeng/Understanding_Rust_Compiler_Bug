plain
 finished in 0.625 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
.....................................F.............F...FF.F.....................i.......  88/255
........................................F...............Fi.FFFF......iiiFF.F.....F....F. 176/255
.......F..F.......iii.iiiii...F.....ii...F...Fi..F...FFF.........F.......F.....
failures:

---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
81           _18 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
82           StorageLive(_11);                // scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10
83           _11 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:22
+           _0 = const ();                   // scope 0 at $DIR/const_debuginfo.rs:+0:11: +14:2
84           StorageDead(_11);                // scope 8 at $DIR/const_debuginfo.rs:+14:1: +14:2
85           StorageDead(_10);                // scope 6 at $DIR/const_debuginfo.rs:+14:1: +14:2
86           StorageDead(_14);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2

thread '[mir-opt] tests/mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs stdout ----
14       }
15   
15   
Build completed unsuccessfully in 0:13:15
16       bb1: {
+           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
17           _2 = begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
18                                            // mir::Constant
19                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
24       }
25   
26       bb2: {
26       bb2: {
+           _0 = const ();                   // scope 0 at $DIR/control_flow_simplification.rs:+3:6: +3:6
27           StorageDead(_1);                 // scope 0 at $DIR/control_flow_simplification.rs:+3:5: +3:6
28           return;                          // scope 0 at $DIR/control_flow_simplification.rs:+4:2: +4:2


thread '[mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/issue_66971.rs:+0:11: +0:11
6       let _1: ();                          // in scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
7       let mut _2: ((), u8, u8);            // in scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+       let mut _3: ();                      // in scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
9       bb0: {
9       bb0: {
+           StorageLive(_1);                 // scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
10           StorageLive(_2);                 // scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
-           _2 = (const (), const 0_u8, const 0_u8); // scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+           StorageLive(_3);                 // scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
+           _3 = ();                         // scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
+           _2 = (move _3, const 0_u8, const 0_u8); // scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+           StorageDead(_3);                 // scope 0 at $DIR/issue_66971.rs:+1:21: +1:22
12           _1 = encode(move _2) -> bb1;     // scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
13                                            // mir::Constant
14                                            // + span: $DIR/issue_66971.rs:18:5: 18:11
17   
18       bb1: {
18       bb1: {
19           StorageDead(_2);                 // scope 0 at $DIR/issue_66971.rs:+1:22: +1:23
+           StorageDead(_1);                 // scope 0 at $DIR/issue_66971.rs:+1:23: +1:24
+           _0 = const ();                   // scope 0 at $DIR/issue_66971.rs:+0:11: +2:2
20           return;                          // scope 0 at $DIR/issue_66971.rs:+2:2: +2:2
22   }


thread '[mir-opt] tests/mir-opt/const_prop/issue_66971.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/issue_66971.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
8       let mut _3: (u8, u8);                // in scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
10       bb0: {
10       bb0: {
+           StorageLive(_1);                 // scope 0 at $DIR/issue_67019.rs:+1:5: +1:20
11           StorageLive(_2);                 // scope 0 at $DIR/issue_67019.rs:+1:10: +1:19
12           StorageLive(_3);                 // scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
13 -         _3 = (const 1_u8, const 2_u8);   // scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
22   
23       bb1: {
23       bb1: {
24           StorageDead(_2);                 // scope 0 at $DIR/issue_67019.rs:+1:19: +1:20
+           StorageDead(_1);                 // scope 0 at $DIR/issue_67019.rs:+1:20: +1:21
+           _0 = const ();                   // scope 0 at $DIR/issue_67019.rs:+0:11: +2:2
25           return;                          // scope 0 at $DIR/issue_67019.rs:+2:2: +2:2
27   }


thread '[mir-opt] tests/mir-opt/const_prop/issue_67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/invalid_constant.rs' panicked at 'the mir dump file for invalid_constant.main.RemoveZsts.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/invalid_constant.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/dest-prop/union.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/union.rs stdout ----
28           StorageDead(_2);                 // scope 0 at $DIR/union.rs:+5:29: +5:30
29           StorageLive(_3);                 // scope 1 at $DIR/union.rs:+7:10: +7:26
30           StorageDead(_3);                 // scope 1 at $DIR/union.rs:+7:26: +7:27
+           _0 = const ();                   // scope 0 at $DIR/union.rs:+0:11: +8:2
31           StorageDead(_1);                 // scope 0 at $DIR/union.rs:+8:1: +8:2
32           return;                          // scope 0 at $DIR/union.rs:+8:2: +8:2


thread '[mir-opt] tests/mir-opt/dest-prop/union.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dest-prop/union.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
7 +     scope 1 (inlined foo) {              // at $DIR/asm_unwind.rs:21:5: 21:10
8 +         let _2: D;                       // in scope 1 at $DIR/asm_unwind.rs:15:9: 15:11
9 +         scope 2 {
- +             debug _d => const D;         // in scope 2 at $DIR/asm_unwind.rs:15:9: 15:11
+ +             debug _d => _2;              // in scope 2 at $DIR/asm_unwind.rs:15:9: 15:11
11 +             scope 3 {
13 +         }

19 -                                          // mir::Constant
19 -                                          // mir::Constant
20 -                                          // + span: $DIR/asm_unwind.rs:21:5: 21:8
21 -                                          // + literal: Const { ty: fn() {foo}, val: Value(<ZST>) }
- +         StorageLive(_2);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
- +         asm!("", options(MAY_UNWIND)) -> [return: bb2, unwind: bb3]; // scope 3 at $DIR/asm_unwind.rs:16:14: 16:54
+ +         StorageLive(_2);                 // scope 1 at $DIR/asm_unwind.rs:15:9: 15:11
+ +         _2 = D;                          // scope 1 at $DIR/asm_unwind.rs:15:14: 15:15
+ +         asm!("", options(MAY_UNWIND)) -> [return: bb1, unwind: bb3]; // scope 3 at $DIR/asm_unwind.rs:16:14: 16:54
25   
26       bb1: {


- +         StorageDead(_2);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
-           StorageDead(_1);                 // scope 0 at $DIR/asm_unwind.rs:+1:10: +1:11
-           _0 = const ();                   // scope 0 at $DIR/asm_unwind.rs:+0:15: +2:2
-           return;                          // scope 0 at $DIR/asm_unwind.rs:+2:2: +2:2
+ +         _1 = const ();                   // scope 1 at $DIR/asm_unwind.rs:14:10: 17:2
+ +         drop(_2) -> bb2;                 // scope 1 at $DIR/asm_unwind.rs:17:1: 17:2
32 + 
33 +     bb2: {


- +         drop(_2) -> bb1;                 // scope 1 at $DIR/asm_unwind.rs:17:1: 17:2
+ +         StorageDead(_2);                 // scope 1 at $DIR/asm_unwind.rs:17:1: 17:2
+           StorageDead(_1);                 // scope 0 at $DIR/asm_unwind.rs:+1:10: +1:11
+           _0 = const ();                   // scope 0 at $DIR/asm_unwind.rs:+0:15: +2:2
+           return;                          // scope 0 at $DIR/asm_unwind.rs:+2:2: +2:2
36 + 
36 + 
37 +     bb3 (cleanup): {

thread '[mir-opt] tests/mir-opt/inline/asm_unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/asm_unwind.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:+0:8: +0:8
6       let _1: ();                          // in scope 0 at $DIR/cycle.rs:+1:5: +1:12
7 +     let mut _2: fn() {main};             // in scope 0 at $DIR/cycle.rs:+1:5: +1:12
- +     let mut _5: ();                      // in scope 0 at $DIR/cycle.rs:6:5: 6:8
9 +     scope 1 (inlined f::<fn() {main}>) { // at $DIR/cycle.rs:12:5: 12:12
10 +         debug g => _2;                   // in scope 1 at $DIR/cycle.rs:5:6: 5:7
11 +         let _3: ();                      // in scope 1 at $DIR/cycle.rs:6:5: 6:8

12 +         let mut _4: &fn() {main};        // in scope 1 at $DIR/cycle.rs:6:5: 6:6
+ +         let mut _5: ();                  // in scope 1 at $DIR/cycle.rs:6:5: 6:8
13 +         scope 2 (inlined <fn() {main} as Fn<()>>::call - shim(fn() {main})) { // at $DIR/cycle.rs:6:5: 6:8
15 +     }

25 -                                          // mir::Constant
25 -                                          // mir::Constant
26                                            // + span: $DIR/cycle.rs:12:7: 12:11
27                                            // + literal: Const { ty: fn() {main}, val: Value(<ZST>) }
- +         StorageLive(_3);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
+ +         StorageLive(_3);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
29 +         StorageLive(_4);                 // scope 1 at $DIR/cycle.rs:6:5: 6:6
30 +         _4 = &_2;                        // scope 1 at $DIR/cycle.rs:6:5: 6:6
31 +         StorageLive(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8

- +         _5 = const ();                   // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         _5 = ();                         // scope 1 at $DIR/cycle.rs:6:5: 6:8
33 +         _3 = move (*_4)() -> [return: bb4, unwind: bb2]; // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
35   

36       bb1: {
36       bb1: {
- +         StorageDead(_3);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
38 +         StorageDead(_2);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
39           StorageDead(_1);                 // scope 0 at $DIR/cycle.rs:+1:12: +1:13
40           _0 = const ();                   // scope 0 at $DIR/cycle.rs:+0:8: +2:2
50 +     }
51 + 
52 +     bb4: {
52 +     bb4: {
- +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:6:7: 6:8
54 +         StorageDead(_4);                 // scope 1 at $DIR/cycle.rs:6:7: 6:8
+ +         StorageDead(_3);                 // scope 1 at $DIR/cycle.rs:6:8: 6:9
+ +         _1 = const ();                   // scope 1 at $DIR/cycle.rs:5:20: 7:2
55 +         drop(_2) -> bb1;                 // scope 1 at $DIR/cycle.rs:7:1: 7:2
57   }


thread '[mir-opt] tests/mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/cycle.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
30 -     }
31 - 
32 -     bb2: {
32 -     bb2: {
+ +         _2 = const ();                   // scope 1 at $DIR/inline_instruction_set.rs:36:26: 36:28
33           StorageDead(_2);                 // scope 0 at $DIR/inline_instruction_set.rs:+2:26: +2:27
34           StorageLive(_3);                 // scope 0 at $DIR/inline_instruction_set.rs:+3:5: +3:30
35 -         _3 = instruction_set_default() -> [return: bb3, unwind unreachable]; // scope 0 at $DIR/inline_instruction_set.rs:+3:5: +3:30
39 -     }
40 - 
41 -     bb3: {
41 -     bb3: {
+ +         _3 = const ();                   // scope 2 at $DIR/inline_instruction_set.rs:39:30: 39:32
42           StorageDead(_3);                 // scope 0 at $DIR/inline_instruction_set.rs:+3:30: +3:31
43           StorageLive(_4);                 // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
44 -         _4 = inline_always_and_using_inline_asm() -> [return: bb4, unwind unreachable]; // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41

thread '[mir-opt] tests/mir-opt/inline/inline_instruction_set.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_instruction_set.t32.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/exponential_runtime.rs stdout ----
18       bb0: {
18       bb0: {
19           StorageLive(_1);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
20 -         _1 = <() as G>::call() -> bb1;   // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageLive(_2);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageLive(_3);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageLive(_4);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageLive(_5);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         StorageLive(_6);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         StorageLive(_7);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         _5 = <() as E>::call() -> bb4;   // scope 2 at $DIR/exponential_runtime.rs:62:9: 62:25
+ +         StorageLive(_2);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
+ +         StorageLive(_5);                 // scope 2 at $DIR/exponential_runtime.rs:62:9: 62:25
+ +         _5 = <() as E>::call() -> bb3;   // scope 2 at $DIR/exponential_runtime.rs:62:9: 62:25
28                                            // mir::Constant
29 -                                          // + span: $DIR/exponential_runtime.rs:87:5: 87:20
30 -                                          // + literal: Const { ty: fn() {<() as G>::call}, val: Value(<ZST>) }
33       }
34   
35       bb1: {
35       bb1: {
- +         StorageDead(_4);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageDead(_3);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
- +         StorageDead(_2);                 // scope 0 at $DIR/exponential_runtime.rs:+1:5: +1:22
+ +         StorageDead(_3);                 // scope 1 at $DIR/exponential_runtime.rs:75:25: 75:26
+ +         StorageLive(_4);                 // scope 1 at $DIR/exponential_runtime.rs:76:9: 76:25
+ +         _4 = <() as F>::call() -> bb2;   // scope 1 at $DIR/exponential_runtime.rs:76:9: 76:25
+ +                                          // mir::Constant
+ +                                          // + span: $DIR/exponential_runtime.rs:76:9: 76:23
+ +                                          // + literal: Const { ty: fn() {<() as F>::call}, val: Value(<ZST>) }
+ +     }
+ +     bb2: {
+ +     bb2: {
+ +         StorageDead(_4);                 // scope 1 at $DIR/exponential_runtime.rs:76:25: 76:26
+ +         _1 = const ();                   // scope 1 at $DIR/exponential_runtime.rs:73:15: 77:6
39           StorageDead(_1);                 // scope 0 at $DIR/exponential_runtime.rs:+1:22: +1:23
40           _0 = const ();                   // scope 0 at $DIR/exponential_runtime.rs:+0:11: +2:2
41           return;                          // scope 0 at $DIR/exponential_runtime.rs:+2:2: +2:2
42 +     }
43 + 
- +     bb2: {
- +     bb2: {
- +         StorageDead(_7);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         StorageDead(_6);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         StorageDead(_5);                 // scope 1 at $DIR/exponential_runtime.rs:74:9: 74:25
- +         _3 = <() as F>::call() -> bb3;   // scope 1 at $DIR/exponential_runtime.rs:75:9: 75:25
- +                                          // mir::Constant
- +                                          // + span: $DIR/exponential_runtime.rs:75:9: 75:23
- +                                          // + literal: Const { ty: fn() {<() as F>::call}, val: Value(<ZST>) }
- +     }
54 +     bb3: {
54 +     bb3: {
- +         _4 = <() as F>::call() -> bb1;   // scope 1 at $DIR/exponential_runtime.rs:76:9: 76:25
+ +         StorageDead(_5);                 // scope 2 at $DIR/exponential_runtime.rs:62:25: 62:26
+ +         StorageLive(_6);                 // scope 2 at $DIR/exponential_runtime.rs:63:9: 63:25
+ +         _6 = <() as E>::call() -> bb4;   // scope 2 at $DIR/exponential_runtime.rs:63:9: 63:25
56 +                                          // mir::Constant
- +                                          // + span: $DIR/exponential_runtime.rs:76:9: 76:23
- +                                          // + literal: Const { ty: fn() {<() as F>::call}, val: Value(<ZST>) }
+ +                                          // + span: $DIR/exponential_runtime.rs:63:9: 63:23
+ +                                          // + literal: Const { ty: fn() {<() as E>::call}, val: Value(<ZST>) }
60 + 
61 +     bb4: {


- +         _6 = <() as E>::call() -> bb5;   // scope 2 at $DIR/exponential_runtime.rs:63:9: 63:25
+ +         StorageDead(_6);                 // scope 2 at $DIR/exponential_runtime.rs:63:25: 63:26
+ +         StorageLive(_7);                 // scope 2 at $DIR/exponential_runtime.rs:64:9: 64:25
+ +         _7 = <() as E>::call() -> bb5;   // scope 2 at $DIR/exponential_runtime.rs:64:9: 64:25
63 +                                          // mir::Constant
- +                                          // + span: $DIR/exponential_runtime.rs:63:9: 63:23
+ +                                          // + span: $DIR/exponential_runtime.rs:64:9: 64:23
65 +                                          // + literal: Const { ty: fn() {<() as E>::call}, val: Value(<ZST>) }
67 + 

68 +     bb5: {
68 +     bb5: {
- +         _7 = <() as E>::call() -> bb2;   // scope 2 at $DIR/exponential_runtime.rs:64:9: 64:25
+ +         StorageDead(_7);                 // scope 2 at $DIR/exponential_runtime.rs:64:25: 64:26
+ +         _2 = const ();                   // scope 2 at $DIR/exponential_runtime.rs:61:15: 65:6
+ +         StorageDead(_2);                 // scope 1 at $DIR/exponential_runtime.rs:74:25: 74:26
+ +         StorageLive(_3);                 // scope 1 at $DIR/exponential_runtime.rs:75:9: 75:25
+ +         _3 = <() as F>::call() -> bb1;   // scope 1 at $DIR/exponential_runtime.rs:75:9: 75:25
70 +                                          // mir::Constant
- +                                          // + span: $DIR/exponential_runtime.rs:64:9: 64:23
- +                                          // + literal: Const { ty: fn() {<() as E>::call}, val: Value(<ZST>) }
+ +                                          // + span: $DIR/exponential_runtime.rs:75:9: 75:23
+ +                                          // + literal: Const { ty: fn() {<() as F>::call}, val: Value(<ZST>) }
74   }
75   


thread '[mir-opt] tests/mir-opt/inline/exponential_runtime.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/exponential_runtime.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_compatibility.rs stdout ----
16 -     }
17 - 
18 -     bb1: {
18 -     bb1: {
+ +         _1 = const ();                   // scope 1 at $DIR/inline_compatibility.rs:34:32: 34:34
19           StorageDead(_1);                 // scope 0 at $DIR/inline_compatibility.rs:+1:21: +1:22
20           _0 = const ();                   // scope 0 at $DIR/inline_compatibility.rs:+0:40: +2:2
21           return;                          // scope 0 at $DIR/inline_compatibility.rs:+2:2: +2:2

thread '[mir-opt] tests/mir-opt/inline/inline_compatibility.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_compatibility.inlined_target_feature.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline_cycle.rs:+0:10: +0:10
6       let _1: ();                          // in scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
7 +     let mut _2: fn() {f};                // in scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
- +     let mut _4: ();                      // in scope 0 at $DIR/inline_cycle.rs:55:5: 55:8
9 +     scope 1 (inlined call::<fn() {f}>) { // at $DIR/inline_cycle.rs:50:5: 50:12
10 +         debug f => _2;                   // in scope 1 at $DIR/inline_cycle.rs:54:22: 54:23
11 +         let _3: ();                      // in scope 1 at $DIR/inline_cycle.rs:55:5: 55:8

+ +         let mut _4: ();                  // in scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
12 +         scope 2 (inlined <fn() {f} as FnOnce<()>>::call_once - shim(fn() {f})) { // at $DIR/inline_cycle.rs:55:5: 55:8
14 +     }

24 -                                          // mir::Constant
24 -                                          // mir::Constant
25                                            // + span: $DIR/inline_cycle.rs:50:10: 50:11
26                                            // + literal: Const { ty: fn() {f}, val: Value(<ZST>) }
- +         StorageLive(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
+ +         StorageLive(_3);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
28 +         StorageLive(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
- +         _4 = const ();                   // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
+ +         _4 = ();                         // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
30 +         _3 = move _2() -> bb1;           // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
32   

33       bb1: {
33       bb1: {
- +         StorageDead(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
- +         StorageDead(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
+ +         StorageDead(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:7: 55:8
+ +         StorageDead(_3);                 // scope 1 at $DIR/inline_cycle.rs:55:8: 55:9
+ +         _1 = const ();                   // scope 1 at $DIR/inline_cycle.rs:54:28: 56:2
36 +         StorageDead(_2);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
37           StorageDead(_1);                 // scope 0 at $DIR/inline_cycle.rs:+1:12: +1:13
38           _0 = const ();                   // scope 0 at $DIR/inline_cycle.rs:+0:10: +2:2

thread '[mir-opt] tests/mir-opt/inline/inline_cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_cycle.two.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_options.rs stdout ----
21     bb1: {
21     bb1: {
22         StorageDead(_1);                 // scope 0 at $DIR/inline_options.rs:+1:18: +1:19
23         StorageLive(_2);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageLive(_3);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageLive(_4);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageLive(_5);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         _3 = g() -> bb3;                 // scope 1 at $DIR/inline_options.rs:17:23: 17:26
+         StorageLive(_3);                 // scope 1 at $DIR/inline_options.rs:17:23: 17:26
+         _3 = g() -> bb2;                 // scope 1 at $DIR/inline_options.rs:17:23: 17:26
28                                          // mir::Constant
29                                          // + span: $DIR/inline_options.rs:17:23: 17:24
30                                          // + literal: Const { ty: fn() {g}, val: Value(<ZST>) }
31     }
32 
33     bb2: {
33     bb2: {
-         StorageDead(_5);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageDead(_4);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageDead(_3);                 // scope 0 at $DIR/inline_options.rs:+2:5: +2:21
-         StorageDead(_2);                 // scope 0 at $DIR/inline_options.rs:+2:21: +2:22
-         _0 = const ();                   // scope 0 at $DIR/inline_options.rs:+0:11: +3:2
-         return;                          // scope 0 at $DIR/inline_options.rs:+3:2: +3:2
- 
-     bb3: {
-     bb3: {
-         _4 = g() -> bb4;                 // scope 1 at $DIR/inline_options.rs:17:28: 17:31
+         StorageDead(_3);                 // scope 1 at $DIR/inline_options.rs:17:26: 17:27
+         StorageLive(_4);                 // scope 1 at $DIR/inline_options.rs:17:28: 17:31
+         _4 = g() -> bb3;                 // scope 1 at $DIR/inline_options.rs:17:28: 17:31
44                                          // mir::Constant
45                                          // + span: $DIR/inline_options.rs:17:28: 17:29
46                                          // + literal: Const { ty: fn() {g}, val: Value(<ZST>) }
47     }
48 
-     bb4: {
-     bb4: {
-         _5 = g() -> bb2;                 // scope 1 at $DIR/inline_options.rs:17:33: 17:36
+     bb3: {
+         StorageDead(_4);                 // scope 1 at $DIR/inline_options.rs:17:31: 17:32
+         StorageLive(_5);                 // scope 1 at $DIR/inline_options.rs:17:33: 17:36
+         _5 = g() -> bb4;                 // scope 1 at $DIR/inline_options.rs:17:33: 17:36
51                                          // mir::Constant
52                                          // + span: $DIR/inline_options.rs:17:33: 17:34
53                                          // + literal: Const { ty: fn() {g}, val: Value(<ZST>) }
+     }
+ 
+     bb4: {
+     bb4: {
+         StorageDead(_5);                 // scope 1 at $DIR/inline_options.rs:17:36: 17:37
+         _2 = const ();                   // scope 1 at $DIR/inline_options.rs:17:21: 17:39
+         StorageDead(_2);                 // scope 0 at $DIR/inline_options.rs:+2:21: +2:22
+         _0 = const ();                   // scope 0 at $DIR/inline_options.rs:+0:11: +3:2
+         return;                          // scope 0 at $DIR/inline_options.rs:+3:2: +3:2
55 }
56 


thread '[mir-opt] tests/mir-opt/inline/inline_options.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_options.main.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
34       bb2: {
34       bb2: {
35           StorageLive(_6);                 // scope 0 at $DIR/inline_diverging.rs:+4:9: +4:16
36 -         _6 = panic();                    // scope 0 at $DIR/inline_diverging.rs:+4:9: +4:16
- +         StorageLive(_7);                 // scope 0 at $DIR/inline_diverging.rs:+4:9: +4:16
+ +         StorageLive(_7);                 // scope 1 at $SRC_DIR/std/src/panic.rs:LL:COL
38 +         _7 = begin_panic::<&str>(const "explicit panic"); // scope 1 at $SRC_DIR/std/src/panic.rs:LL:COL
39                                            // mir::Constant
40 -                                          // + span: $DIR/inline_diverging.rs:16:9: 16:14

thread '[mir-opt] tests/mir-opt/inline/inline_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_diverging.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/intrinsic_asserts.rs stdout ----
---- [mir-opt] tests/mir-opt/intrinsic_asserts.rs stdout ----
8       let _3: ();                          // in scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
10       bb0: {
10       bb0: {
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
+           StorageLive(_1);                 // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
12 -         _1 = assert_inhabited::<()>() -> [return: bb1, unwind unreachable]; // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
13 -                                          // mir::Constant
14 -                                          // + span: $DIR/intrinsic_asserts.rs:7:5: 7:45
17       }
18   
19       bb1: {
19       bb1: {
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+1:47: +1:48
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
+           StorageDead(_1);                 // scope 0 at $DIR/intrinsic_asserts.rs:+1:47: +1:48
+           StorageLive(_2);                 // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
22 -         _2 = assert_zero_valid::<u8>() -> [return: bb2, unwind unreachable]; // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
23 -                                          // mir::Constant
24 -                                          // + span: $DIR/intrinsic_asserts.rs:8:5: 8:46
27       }
28   
29       bb2: {
29       bb2: {
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+2:48: +2:49
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
+           StorageDead(_2);                 // scope 0 at $DIR/intrinsic_asserts.rs:+2:48: +2:49
+           StorageLive(_3);                 // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
32 -         _3 = assert_mem_uninitialized_valid::<u8>() -> [return: bb3, unwind unreachable]; // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
33 -                                          // mir::Constant
34 -                                          // + span: $DIR/intrinsic_asserts.rs:9:5: 9:59
37       }
38   
39       bb3: {
39       bb3: {
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+3:61: +3:62
-           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+0:20: +4:2
+           StorageDead(_3);                 // scope 0 at $DIR/intrinsic_asserts.rs:+3:61: +3:62
+           _0 = const ();                   // scope 0 at $DIR/intrinsic_asserts.rs:+0:20: +4:2
42           return;                          // scope 0 at $DIR/intrinsic_asserts.rs:+4:2: +4:2
44   }


thread '[mir-opt] tests/mir-opt/intrinsic_asserts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/intrinsic_asserts.removable.InstSimplify.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/issue_76997_inline_scopes_parenting.rs stdout ----
10     scope 1 {
10     scope 1 {
11         debug f => _1;                   // in scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:9: +1:10
12         scope 2 (inlined main::{closure#0}) { // at $DIR/issue_76997_inline_scopes_parenting.rs:6:5: 6:10
-             debug x => const ();         // in scope 2 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:14: +1:15
+             debug x => _5;               // in scope 2 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:14: +1:15
14             scope 3 {
-                 debug y => const ();     // in scope 3 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:23: +1:24
+                 debug y => _5;           // in scope 3 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:23: +1:24
17         }
18     }


35         _3 = (move _4,);                 // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:5: +2:10
36         StorageLive(_5);                 // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:5: +2:10
37         _5 = move (_3.0: ());            // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:5: +2:10
+         _0 = _5;                         // scope 3 at $DIR/issue_76997_inline_scopes_parenting.rs:+1:30: +1:31
38         StorageDead(_5);                 // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:5: +2:10
39         StorageDead(_4);                 // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:9: +2:10
40         StorageDead(_3);                 // scope 1 at $DIR/issue_76997_inline_scopes_parenting.rs:+2:9: +2:10

thread '[mir-opt] tests/mir-opt/inline/issue_76997_inline_scopes_parenting.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/issue_76997_inline_scopes_parenting.main.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issue_76432.rs stdout ----
