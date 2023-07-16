plain
 finished in 0.458 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 180 tests
i....FF........F..F......FF...FF..F....iF..F...F..FF.FF.......FFFFF.FF.....F....F..F.FFF 88/180
F..FFFFFiFF..FFF.F.FFF.FFFFF..FFFFFFF......FFFF....F.....FFi....FF...FFF............F..F 176/180
failures:

---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
30           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
31 -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
32           StorageDead(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
-           _0 = core::slice::<impl [&i32]>::as_ptr(move _1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+           _0 = ZST: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}(move _1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
34                                            // mir::Constant
35                                            // + span: $DIR/const-promotion-extern-static.rs:9:36: 9:42
-                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(ZST) }
38   
39       bb1: {


thread '[mir-opt] src/test/mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3466:25

---- [mir-opt] src/test/mir-opt/array-index-is-temporary.rs stdout ----
---- [mir-opt] src/test/mir-opt/array-index-is-temporary.rs stdout ----
36         StorageLive(_5);                 // scope 3 at $DIR/array-index-is-temporary.rs:16:12: 16:29
37         StorageLive(_6);                 // scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
38         _6 = _3;                         // scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
-         _5 = foo(move _6) -> bb1;        // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
+         _5 = ZST: unsafe fn(*mut usize) -> u32 {foo}(move _6) -> bb1; // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
40                                          // mir::Constant
41                                          // + span: $DIR/array-index-is-temporary.rs:16:21: 16:24
-                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(ZST) }
44 
45     bb1: {


thread '[mir-opt] src/test/mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array_index_is_temporary.main.SimplifyCfg-elaborate-drops.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/box_expr.rs stdout ----
---- [mir-opt] src/test/mir-opt/box_expr.rs stdout ----
19         StorageLive(_1);                 // scope 0 at $DIR/box_expr.rs:7:9: 7:10
20         _2 = SizeOf(S);                  // scope 2 at $DIR/box_expr.rs:7:13: 7:25
21         _3 = AlignOf(S);                 // scope 2 at $DIR/box_expr.rs:7:13: 7:25
-         _4 = alloc::alloc::exchange_malloc(move _2, move _3) -> bb1; // scope 2 at $DIR/box_expr.rs:7:13: 7:25
+         _4 = ZST: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}(move _2, move _3) -> bb1; // scope 2 at $DIR/box_expr.rs:7:13: 7:25
23                                          // mir::Constant
24                                          // + span: $DIR/box_expr.rs:7:13: 7:25
-                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(ZST) }
27 
28     bb1: {


29         StorageLive(_5);                 // scope 0 at $DIR/box_expr.rs:7:13: 7:25
30         _5 = ShallowInitBox(move _4, S); // scope 0 at $DIR/box_expr.rs:7:13: 7:25
-         (*_5) = S::new() -> [return: bb2, unwind: bb8]; // scope 0 at $DIR/box_expr.rs:7:17: 7:25
+         (*_5) = ZST: fn() -> S {S::new}() -> [return: bb2, unwind: bb8]; // scope 0 at $DIR/box_expr.rs:7:17: 7:25
32                                          // mir::Constant
33                                          // + span: $DIR/box_expr.rs:7:17: 7:23
-                                          // + literal: Const { ty: fn() -> S {S::new}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn() -> S {S::new}, val: Value(ZST) }
36 
37     bb2: {


44         StorageLive(_6);                 // scope 1 at $DIR/box_expr.rs:8:5: 8:12
45         StorageLive(_7);                 // scope 1 at $DIR/box_expr.rs:8:10: 8:11
46         _7 = move _1;                    // scope 1 at $DIR/box_expr.rs:8:10: 8:11
-         _6 = std::mem::drop::<Box<S>>(move _7) -> [return: bb4, unwind: bb6]; // scope 1 at $DIR/box_expr.rs:8:5: 8:12
+         _6 = ZST: fn(Box<S>) {std::mem::drop::<Box<S>>}(move _7) -> [return: bb4, unwind: bb6]; // scope 1 at $DIR/box_expr.rs:8:5: 8:12
48                                          // mir::Constant
49                                          // + span: $DIR/box_expr.rs:8:5: 8:9
-                                          // + literal: Const { ty: fn(Box<S>) {std::mem::drop::<Box<S>>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(Box<S>) {std::mem::drop::<Box<S>>}, val: Value(ZST) }
52 
53     bb4: {


thread '[mir-opt] src/test/mir-opt/box_expr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/box_expr.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3466:25
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [mir-opt] src/test/mir-opt/combine_clone_of_primitives.rs stdout ----
---- [mir-opt] src/test/mir-opt/combine_clone_of_primitives.rs stdout ----
21           _4 = &((*_1).0: T);              // scope 0 at $DIR/combine_clone_of_primitives.rs:8:5: 8:9
22 -         _3 = &(*_4);                     // scope 0 at $DIR/combine_clone_of_primitives.rs:8:5: 8:9
23 +         _3 = _4;                         // scope 0 at $DIR/combine_clone_of_primitives.rs:8:5: 8:9
-           _2 = <T as Clone>::clone(move _3) -> bb1; // scope 0 at $DIR/combine_clone_of_primitives.rs:8:5: 8:9
+           _2 = ZST: for<'r> fn(&'r T) -> T {<T as Clone>::clone}(move _3) -> bb1; // scope 0 at $DIR/combine_clone_of_primitives.rs:8:5: 8:9
25                                            // mir::Constant
26                                            // + span: $DIR/combine_clone_of_primitives.rs:8:5: 8:9
-                                            // + literal: Const { ty: for<'r> fn(&'r T) -> T {<T as Clone>::clone}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r T) -> T {<T as Clone>::clone}, val: Value(ZST) }
29   
30       bb1: {


34           StorageLive(_7);                 // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
35           _7 = &((*_1).1: u64);            // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
36 -         _6 = &(*_7);                     // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
- -         _5 = <u64 as Clone>::clone(move _6) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
+ -         _5 = ZST: for<'r> fn(&'r u64) -> u64 {<u64 as Clone>::clone}(move _6) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
38 -                                          // mir::Constant
39 -                                          // + span: $DIR/combine_clone_of_primitives.rs:9:5: 9:11
- -                                          // + literal: Const { ty: for<'r> fn(&'r u64) -> u64 {<u64 as Clone>::clone}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: for<'r> fn(&'r u64) -> u64 {<u64 as Clone>::clone}, val: Value(ZST) }
41 +         _6 = _7;                         // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
42 +         _5 = (*_6);                      // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11
43 +         goto -> bb2;                     // scope 0 at $DIR/combine_clone_of_primitives.rs:9:5: 9:11

50           StorageLive(_10);                // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
51           _10 = &((*_1).2: [f32; 3]);      // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
52 -         _9 = &(*_10);                    // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
- -         _8 = <[f32; 3] as Clone>::clone(move _9) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
+ -         _8 = ZST: for<'r> fn(&'r [f32; 3]) -> [f32; 3] {<[f32; 3] as Clone>::clone}(move _9) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
54 -                                          // mir::Constant
55 -                                          // + span: $DIR/combine_clone_of_primitives.rs:10:5: 10:16
- -                                          // + literal: Const { ty: for<'r> fn(&'r [f32; 3]) -> [f32; 3] {<[f32; 3] as Clone>::clone}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: for<'r> fn(&'r [f32; 3]) -> [f32; 3] {<[f32; 3] as Clone>::clone}, val: Value(ZST) }
57 +         _9 = _10;                        // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
58 +         _8 = (*_9);                      // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16
59 +         goto -> bb3;                     // scope 0 at $DIR/combine_clone_of_primitives.rs:10:5: 10:16

thread '[mir-opt] src/test/mir-opt/combine_clone_of_primitives.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/combine_clone_of_primitives.{impl#0}-clone.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
27           StorageLive(_4);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
28           StorageLive(_5);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:10: 8:11
29           _5 = _1;                         // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:10: 8:11
-           _4 = read(move _5) -> bb1;       // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
+           _4 = ZST: fn(usize) {read}(move _5) -> bb1; // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
31                                            // mir::Constant
32                                            // + span: $DIR/const_prop_fails_gracefully.rs:8:5: 8:9
-                                            // + literal: Const { ty: fn(usize) {read}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(usize) {read}, val: Value(ZST) }
35   
36       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/boxes.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/boxes.rs stdout ----
26           StorageLive(_3);                 // scope 0 at $DIR/boxes.rs:12:14: 12:22
27 -         _4 = SizeOf(i32);                // scope 2 at $DIR/boxes.rs:12:14: 12:22
28 -         _5 = AlignOf(i32);               // scope 2 at $DIR/boxes.rs:12:14: 12:22
- -         _6 = alloc::alloc::exchange_malloc(move _4, move _5) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
+ -         _6 = ZST: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}(move _4, move _5) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
30 +         _4 = const 4_usize;              // scope 2 at $DIR/boxes.rs:12:14: 12:22
31 +         _5 = const 4_usize;              // scope 2 at $DIR/boxes.rs:12:14: 12:22
- +         _6 = alloc::alloc::exchange_malloc(const 4_usize, const 4_usize) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
+ +         _6 = ZST: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}(const 4_usize, const 4_usize) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
33                                            // mir::Constant
34                                            // + span: $DIR/boxes.rs:12:14: 12:22
-                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(ZST) }
37   
38       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/boxes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/boxes.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/issue-66971.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/issue-66971.rs stdout ----
17           (_2.1: u8) = const 0_u8;         // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
18           (_2.2: u8) = const 0_u8;         // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
19           StorageDead(_3);                 // scope 0 at $DIR/issue-66971.rs:16:21: 16:22
-           _1 = encode(move _2) -> bb1;     // scope 0 at $DIR/issue-66971.rs:16:5: 16:23
+           _1 = ZST: fn(((), u8, u8)) {encode}(move _2) -> bb1; // scope 0 at $DIR/issue-66971.rs:16:5: 16:23
21                                            // mir::Constant
22                                            // + span: $DIR/issue-66971.rs:16:5: 16:11
-                                            // + literal: Const { ty: fn(((), u8, u8)) {encode}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(((), u8, u8)) {encode}, val: Value(ZST) }
25   
26       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/issue-66971.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_66971.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
16   
17       bb1: {
17       bb1: {
18           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
-           _2 = begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+           _2 = ZST: fn(&str) -> ! {begin_panic::<&str>}(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
20                                            // mir::Constant
21                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                            // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(ZST) }
23                                            // mir::Constant
24                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
25                                            // + literal: Const { ty: &str, val: Value(Slice(..)) }

thread '[mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/issue-67019.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/issue-67019.rs stdout ----
18 -         (_2.0: (u8, u8)) = move _3;      // scope 0 at $DIR/issue-67019.rs:11:10: 11:19
19 +         (_2.0: (u8, u8)) = const (1_u8, 2_u8); // scope 0 at $DIR/issue-67019.rs:11:10: 11:19
20           StorageDead(_3);                 // scope 0 at $DIR/issue-67019.rs:11:18: 11:19
-           _1 = test(move _2) -> bb1;       // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
+           _1 = ZST: fn(((u8, u8),)) {test}(move _2) -> bb1; // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
22                                            // mir::Constant
23                                            // + span: $DIR/issue-67019.rs:11:5: 11:9
-                                            // + literal: Const { ty: fn(((u8, u8),)) {test}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(((u8, u8),)) {test}, val: Value(ZST) }
26   
27       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/issue-67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs stdout ----
14   
15       bb0: {
15       bb0: {
16           StorageLive(_1);                 // scope 0 at $DIR/mutable_variable_aggregate_partial_read.rs:5:9: 5:14
-           _1 = foo() -> bb1;               // scope 0 at $DIR/mutable_variable_aggregate_partial_read.rs:5:29: 5:34
+           _1 = ZST: fn() -> (i32, i32) {foo}() -> bb1; // scope 0 at $DIR/mutable_variable_aggregate_partial_read.rs:5:29: 5:34
18                                            // mir::Constant
19                                            // + span: $DIR/mutable_variable_aggregate_partial_read.rs:5:29: 5:32
-                                            // + literal: Const { ty: fn() -> (i32, i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> (i32, i32) {foo}, val: Value(ZST) }
22   
23       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_aggregate_partial_read.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/mutable_variable_unprop_assign.rs stdout ----
23   
24       bb0: {
24       bb0: {
25           StorageLive(_1);                 // scope 0 at $DIR/mutable_variable_unprop_assign.rs:5:9: 5:10
-           _1 = foo() -> bb1;               // scope 0 at $DIR/mutable_variable_unprop_assign.rs:5:13: 5:18
+           _1 = ZST: fn() -> i32 {foo}() -> bb1; // scope 0 at $DIR/mutable_variable_unprop_assign.rs:5:13: 5:18
27                                            // mir::Constant
28                                            // + span: $DIR/mutable_variable_unprop_assign.rs:5:13: 5:16
-                                            // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {foo}, val: Value(ZST) }
31   
32       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/mutable_variable_unprop_assign.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_unprop_assign.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs stdout ----
13           StorageLive(_1);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:41
14           StorageLive(_2);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:26
15           StorageLive(_3);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:17
-           _3 = main as fn() (Pointer(ReifyFnPointer)); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:17
+           _3 = ZST: fn() {main} as fn() (Pointer(ReifyFnPointer)); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:17
17                                            // mir::Constant
18                                            // + span: $DIR/reify_fn_ptr.rs:4:13: 4:17
-                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() {main}, val: Value(ZST) }
20           _2 = move _3 as usize (PointerExposeAddress); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:26
21           StorageDead(_3);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:25: 4:26
22           _1 = move _2 as *const fn() (PointerFromExposedAddress); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:41

thread '[mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/reify_fn_ptr.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
16           StorageLive(_2);                 // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
17           StorageLive(_3);                 // scope 1 at $DIR/scalar_literal_propagation.rs:4:13: 4:14
18 -         _3 = _1;                         // scope 1 at $DIR/scalar_literal_propagation.rs:4:13: 4:14
- -         _2 = consume(move _3) -> bb1;    // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
+ -         _2 = ZST: fn(u32) {consume}(move _3) -> bb1; // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
20 +         _3 = const 1_u32;                // scope 1 at $DIR/scalar_literal_propagation.rs:4:13: 4:14
- +         _2 = consume(const 1_u32) -> bb1; // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
+ +         _2 = ZST: fn(u32) {consume}(const 1_u32) -> bb1; // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
22                                            // mir::Constant
23                                            // + span: $DIR/scalar_literal_propagation.rs:4:5: 4:12
-                                            // + literal: Const { ty: fn(u32) {consume}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(u32) {consume}, val: Value(ZST) }
26   
27       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/scalar_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/scalar_literal_propagation.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/switch_int.rs stdout ----
13       }
14   
15       bb1: {
15       bb1: {
-           _0 = foo(const -1_i32) -> bb3;   // scope 0 at $DIR/switch_int.rs:9:14: 9:21
+           _0 = ZST: fn(i32) {foo}(const -1_i32) -> bb3; // scope 0 at $DIR/switch_int.rs:9:14: 9:21
17                                            // mir::Constant
18                                            // + span: $DIR/switch_int.rs:9:14: 9:17
-                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(ZST) }
21   
22       bb2: {


-           _0 = foo(const 0_i32) -> bb3;    // scope 0 at $DIR/switch_int.rs:8:14: 8:20
+           _0 = ZST: fn(i32) {foo}(const 0_i32) -> bb3; // scope 0 at $DIR/switch_int.rs:8:14: 8:20
24                                            // mir::Constant
25                                            // + span: $DIR/switch_int.rs:8:14: 8:17
-                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(ZST) }
28   
29       bb3: {


thread '[mir-opt] src/test/mir-opt/const_prop/switch_int.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/switch_int.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
19           StorageLive(_3);                 // scope 1 at $DIR/tuple_literal_propagation.rs:5:13: 5:14
20 -         _3 = _1;                         // scope 1 at $DIR/tuple_literal_propagation.rs:5:13: 5:14
21 +         _3 = const (1_u32, 2_u32);       // scope 1 at $DIR/tuple_literal_propagation.rs:5:13: 5:14
-           _2 = consume(move _3) -> bb1;    // scope 1 at $DIR/tuple_literal_propagation.rs:5:5: 5:15
+           _2 = ZST: fn((u32, u32)) {consume}(move _3) -> bb1; // scope 1 at $DIR/tuple_literal_propagation.rs:5:5: 5:15
23                                            // mir::Constant
24                                            // + span: $DIR/tuple_literal_propagation.rs:5:5: 5:12
-                                            // + literal: Const { ty: fn((u32, u32)) {consume}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn((u32, u32)) {consume}, val: Value(ZST) }
27   
28       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/tuple_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/tuple_literal_propagation.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
25   
26       bb1: {
26       bb1: {
27           StorageLive(_5);                 // scope 0 at $DIR/cycle.rs:12:11: 12:17
-           _5 = cond() -> bb2;              // scope 0 at $DIR/cycle.rs:12:11: 12:17
+           _5 = ZST: fn() -> bool {cond}() -> bb2; // scope 0 at $DIR/cycle.rs:12:11: 12:17
29                                            // mir::Constant
30                                            // + span: $DIR/cycle.rs:12:11: 12:15
-                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(ZST) }
33   
34       bb2: {


thread '[mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dead-store-elimination/cycle.cycle.DeadStoreElimination.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/derefer_terminator_test.rs stdout ----
30   
31       bb0: {
31       bb0: {
32           StorageLive(_1);                 // scope 0 at $DIR/derefer_terminator_test.rs:3:9: 3:10
-           _1 = foo() -> bb1;               // scope 0 at $DIR/derefer_terminator_test.rs:3:13: 3:18
+           _1 = ZST: fn() -> bool {foo}() -> bb1; // scope 0 at $DIR/derefer_terminator_test.rs:3:13: 3:18
34                                            // mir::Constant
35                                            // + span: $DIR/derefer_terminator_test.rs:3:13: 3:16
-                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(ZST) }
38   
39       bb1: {


40           StorageLive(_2);                 // scope 1 at $DIR/derefer_terminator_test.rs:4:9: 4:10
-           _2 = foo() -> bb2;               // scope 1 at $DIR/derefer_terminator_test.rs:4:13: 4:18
+           _2 = ZST: fn() -> bool {foo}() -> bb2; // scope 1 at $DIR/derefer_terminator_test.rs:4:13: 4:18
42                                            // mir::Constant
43                                            // + span: $DIR/derefer_terminator_test.rs:4:13: 4:16
-                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(ZST) }
46   
47       bb2: {


thread '[mir-opt] src/test/mir-opt/derefer_terminator_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_terminator_test.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/deduplicate_blocks.rs stdout ----
---- [mir-opt] src/test/mir-opt/deduplicate_blocks.rs stdout ----
23           _3 = _1;                         // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
24           StorageLive(_8);                 // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
25           _8 = _3;                         // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
- -         _2 = transmute::<&str, &[u8]>(move _8) -> bb14; // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
- +         _2 = transmute::<&str, &[u8]>(move _8) -> bb12; // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
+ -         _2 = ZST: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {transmute::<&str, &[u8]>}(move _8) -> bb14; // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
+ +         _2 = ZST: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {transmute::<&str, &[u8]>}(move _8) -> bb12; // scope 2 at $SRC_DIR/core/src/str/mod.rs:LL:COL
28                                            // mir::Constant
29                                            // + span: $SRC_DIR/core/src/str/mod.rs:LL:COL
-                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {transmute::<&str, &[u8]>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {transmute::<&str, &[u8]>}, val: Value(ZST) }
32   
33       bb1: {


thread '[mir-opt] src/test/mir-opt/deduplicate_blocks.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deduplicate_blocks.is_line_doc_comment_2.DeduplicateBlocks.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/derefer_complex_case.rs stdout ----
---- [mir-opt] src/test/mir-opt/derefer_complex_case.rs stdout ----
33                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
34                                            // + literal: Const { ty: &[i32; 2], val: Unevaluated(main, [], Some(promoted[0])) }
35           _2 = &(*_14);                    // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
-           _1 = <&[i32; 2] as IntoIterator>::into_iter(move _2) -> bb1; // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
+           _1 = ZST: fn(&[i32; 2]) -> <&[i32; 2] as IntoIterator>::IntoIter {<&[i32; 2] as IntoIterator>::into_iter}(move _2) -> bb1; // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
37                                            // mir::Constant
38                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
-                                            // + literal: Const { ty: fn(&[i32; 2]) -> <&[i32; 2] as IntoIterator>::IntoIter {<&[i32; 2] as IntoIterator>::into_iter}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(&[i32; 2]) -> <&[i32; 2] as IntoIterator>::IntoIter {<&[i32; 2] as IntoIterator>::into_iter}, val: Value(ZST) }
41   
42       bb1: {


53           StorageLive(_9);                 // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
54           _9 = &mut _4;                    // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
55           _8 = &mut (*_9);                 // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
-           _7 = <std::slice::Iter<i32> as Iterator>::next(move _8) -> bb3; // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
+           _7 = ZST: for<'r> fn(&'r mut std::slice::Iter<i32>) -> Option<<std::slice::Iter<i32> as Iterator>::Item> {<std::slice::Iter<i32> as Iterator>::next}(move _8) -> bb3; // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
57                                            // mir::Constant
58                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
-                                            // + literal: Const { ty: for<'r> fn(&'r mut std::slice::Iter<i32>) -> Option<<std::slice::Iter<i32> as Iterator>::Item> {<std::slice::Iter<i32> as Iterator>::next}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r mut std::slice::Iter<i32>) -> Option<<std::slice::Iter<i32> as Iterator>::Item> {<std::slice::Iter<i32> as Iterator>::next}, val: Value(ZST) }
61   
62       bb3: {


74 +         StorageDead(_15);                // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
75           StorageLive(_13);                // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
76           _13 = _12;                       // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
-           _6 = std::mem::drop::<i32>(move _13) -> bb7; // scope 2 at $DIR/derefer_complex_case.rs:4:29: 4:38
+           _6 = ZST: fn(i32) {std::mem::drop::<i32>}(move _13) -> bb7; // scope 2 at $DIR/derefer_complex_case.rs:4:29: 4:38
78                                            // mir::Constant
79                                            // + span: $DIR/derefer_complex_case.rs:4:29: 4:33
-                                            // + literal: Const { ty: fn(i32) {std::mem::drop::<i32>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(i32) {std::mem::drop::<i32>}, val: Value(ZST) }
82   
83       bb5: {


thread '[mir-opt] src/test/mir-opt/derefer_complex_case.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_complex_case.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/dest-prop/cycle.rs stdout ----
24   
25       bb0: {
25       bb0: {
26           StorageLive(_1);                 // scope 0 at $DIR/cycle.rs:9:9: 9:14
-           _1 = val() -> bb1;               // scope 0 at $DIR/cycle.rs:9:17: 9:22
+           _1 = ZST: fn() -> i32 {val}() -> bb1; // scope 0 at $DIR/cycle.rs:9:17: 9:22
28                                            // mir::Constant
29                                            // + span: $DIR/cycle.rs:9:17: 9:20
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(ZST) }
32   
33       bb1: {


thread '[mir-opt] src/test/mir-opt/dest-prop/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/cycle.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/dest-prop/branch.rs stdout ----
16   
17       bb0: {
17       bb0: {
18           StorageLive(_1);                 // scope 0 at $DIR/branch.rs:13:9: 13:10
-           _1 = val() -> bb1;               // scope 0 at $DIR/branch.rs:13:13: 13:18
+           _1 = ZST: fn() -> i32 {val}() -> bb1; // scope 0 at $DIR/branch.rs:13:13: 13:18
20                                            // mir::Constant
21                                            // + span: $DIR/branch.rs:13:13: 13:16
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(ZST) }
24   
25       bb1: {


26           StorageLive(_2);                 // scope 1 at $DIR/branch.rs:15:9: 15:10
27           StorageLive(_3);                 // scope 1 at $DIR/branch.rs:15:16: 15:22
-           _3 = cond() -> bb2;              // scope 1 at $DIR/branch.rs:15:16: 15:22
+           _3 = ZST: fn() -> bool {cond}() -> bb2; // scope 1 at $DIR/branch.rs:15:16: 15:22
29                                            // mir::Constant
30                                            // + span: $DIR/branch.rs:15:16: 15:20
-                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(ZST) }
33   
34       bb2: {

42   
42   
43       bb4: {
44           StorageLive(_4);                 // scope 1 at $DIR/branch.rs:18:9: 18:14
-           _4 = val() -> bb5;               // scope 1 at $DIR/branch.rs:18:9: 18:14
+           _4 = ZST: fn() -> i32 {val}() -> bb5; // scope 1 at $DIR/branch.rs:18:9: 18:14
46                                            // mir::Constant
47                                            // + span: $DIR/branch.rs:18:9: 18:12
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(ZST) }
50   
51       bb5: {


thread '[mir-opt] src/test/mir-opt/dest-prop/branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/branch.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/dest-prop/union.rs stdout ----
19       bb0: {
19       bb0: {
20           StorageLive(_1);                 // scope 0 at $DIR/union.rs:13:9: 13:11
21           StorageLive(_2);                 // scope 0 at $DIR/union.rs:13:23: 13:28
-           _2 = val() -> bb1;               // scope 0 at $DIR/union.rs:13:23: 13:28
+           _2 = ZST: fn() -> u32 {val}() -> bb1; // scope 0 at $DIR/union.rs:13:23: 13:28
23                                            // mir::Constant
24                                            // + span: $DIR/union.rs:13:23: 13:26
-                                            // + literal: Const { ty: fn() -> u32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> u32 {val}, val: Value(ZST) }
27   
28       bb1: {


thread '[mir-opt] src/test/mir-opt/dest-prop/union.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/union.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/derefer_inline_test.rs stdout ----
---- [mir-opt] src/test/mir-opt/derefer_inline_test.rs stdout ----
16           StorageLive(_1);                 // scope 0 at $DIR/derefer_inline_test.rs:10:5: 10:12
17           _2 = SizeOf(std::boxed::Box<u32>); // scope 1 at $DIR/derefer_inline_test.rs:10:5: 10:12
18           _3 = AlignOf(std::boxed::Box<u32>); // scope 1 at $DIR/derefer_inline_test.rs:10:5: 10:12
-           _4 = alloc::alloc::exchange_malloc(move _2, move _3) -> bb1; // scope 1 at $DIR/derefer_inline_test.rs:10:5: 10:12
