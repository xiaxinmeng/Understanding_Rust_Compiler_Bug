plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 166 tests
....F..F.........F.....FFF....F..FF.i.....FFF.F..FF....FFFF....F........FFFF.FFFFFFiF.F..F..FFF.FFFF 100/166
FFF..FFFFFFF.F...FF.F..FF..F....F.i....FF..FF....F......FF...FFFF.
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
39         _5 = foo(move _6) -> bb1;        // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
40                                          // mir::Constant
41                                          // + span: $DIR/array-index-is-temporary.rs:16:21: 16:24
-                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(0)) }
44 
45     bb1: {


thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array_index_is_temporary.main.SimplifyCfg-elaborate-drops.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3375:25

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
36           _0 = core::slice::<impl [&i32]>::as_ptr(move _1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
37                                            // mir::Constant
38                                            // + span: $DIR/const-promotion-extern-static.rs:9:36: 9:42
-                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(Scalar(0)) }
41   
42       bb1: {


thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/box_expr.rs stdout ----
---- [mir-opt] mir-opt/box_expr.rs stdout ----
22         _4 = alloc::alloc::exchange_malloc(move _2, move _3) -> bb1; // scope 2 at $DIR/box_expr.rs:7:13: 7:25
23                                          // mir::Constant
24                                          // + span: $DIR/box_expr.rs:7:13: 7:25
-                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(0)) }
27 
28     bb1: {


31         (*_5) = S::new() -> [return: bb2, unwind: bb8]; // scope 0 at $DIR/box_expr.rs:7:17: 7:25
32                                          // mir::Constant
33                                          // + span: $DIR/box_expr.rs:7:17: 7:23
-                                          // + literal: Const { ty: fn() -> S {S::new}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn() -> S {S::new}, val: Value(Scalar(0)) }
36 
37     bb2: {


47         _6 = std::mem::drop::<Box<S>>(move _7) -> [return: bb4, unwind: bb6]; // scope 1 at $DIR/box_expr.rs:8:5: 8:12
48                                          // mir::Constant
49                                          // + span: $DIR/box_expr.rs:8:5: 8:9
-                                          // + literal: Const { ty: fn(std::boxed::Box<S>) {std::mem::drop::<std::boxed::Box<S>>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(std::boxed::Box<S>) {std::mem::drop::<std::boxed::Box<S>>}, val: Value(Scalar(0)) }
52 
53     bb4: {


thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/box_expr.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
33           _4 = read(move _5) -> bb1;       // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
34                                            // mir::Constant
35                                            // + span: $DIR/const_prop_fails_gracefully.rs:8:5: 8:9
-                                            // + literal: Const { ty: fn(usize) {read}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(usize) {read}, val: Value(Scalar(0)) }
38   
39       bb1: {


thread '[mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
28 +         _6 = alloc::alloc::exchange_malloc(const 4_usize, const 4_usize) -> bb1; // scope 2 at $DIR/boxes.rs:12:14: 12:22
29                                            // mir::Constant
30                                            // + span: $DIR/boxes.rs:12:14: 12:22
-                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(0)) }
33   
34       bb1: {


thread '[mir-opt] mir-opt/const_prop/boxes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/boxes.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
19           begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
20                                            // mir::Constant
21                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(0)) }
23                                            // ty::Const
24                                            // + ty: &str
25                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })

thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/issue-66971.rs stdout ----
---- [mir-opt] mir-opt/const_prop/issue-66971.rs stdout ----
18           _1 = encode(move _2) -> bb1;     // scope 0 at $DIR/issue-66971.rs:16:5: 16:23
19                                            // mir::Constant
20                                            // + span: $DIR/issue-66971.rs:16:5: 16:11
-                                            // + literal: Const { ty: fn(((), u8, u8)) {encode}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(((), u8, u8)) {encode}, val: Value(Scalar(0)) }
23   
24       bb1: {


thread '[mir-opt] mir-opt/const_prop/issue-66971.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_66971.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
19           _1 = test(move _2) -> bb1;       // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
20                                            // mir::Constant
21                                            // + span: $DIR/issue-67019.rs:11:5: 11:9
-                                            // + literal: Const { ty: fn(((u8, u8),)) {test}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(((u8, u8),)) {test}, val: Value(Scalar(0)) }
24   
25       bb1: {


thread '[mir-opt] mir-opt/const_prop/issue-67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/invalid_constant.rs stdout ----
30       bb2: {
30       bb2: {
31 -         _4 = ((_2 as Some).0: std::option::Option<()>); // scope 1 at $DIR/invalid_constant.rs:16:5: 16:12
32 -         _1 = _4;                         // scope 2 at $DIR/invalid_constant.rs:16:5: 16:12
- +         _4 = const Scalar(0x02): Option::<()>; // scope 1 at $DIR/invalid_constant.rs:16:5: 16:12
+ +         _4 = const Scalar(2): Option::<()>; // scope 1 at $DIR/invalid_constant.rs:16:5: 16:12
34 +                                          // ty::Const
35 +                                          // + ty: std::option::Option<()>
- +                                          // + val: Value(Scalar(0x02))
+ +                                          // + val: Value(Scalar(2))
37 +                                          // mir::Constant
38 +                                          // + span: $DIR/invalid_constant.rs:16:5: 16:12
- +                                          // + literal: Const { ty: std::option::Option<()>, val: Value(Scalar(0x02)) }
- +         _1 = const Scalar(0x02): Option::<()>; // scope 2 at $DIR/invalid_constant.rs:16:5: 16:12
+ +                                          // + literal: Const { ty: std::option::Option<()>, val: Value(Scalar(2)) }
+ +         _1 = const Scalar(2): Option::<()>; // scope 2 at $DIR/invalid_constant.rs:16:5: 16:12
41 +                                          // ty::Const
42 +                                          // + ty: std::option::Option<()>
- +                                          // + val: Value(Scalar(0x02))
+ +                                          // + val: Value(Scalar(2))
44 +                                          // mir::Constant
45 +                                          // + span: $DIR/invalid_constant.rs:16:5: 16:12
- +                                          // + literal: Const { ty: std::option::Option<()>, val: Value(Scalar(0x02)) }
+ +                                          // + literal: Const { ty: std::option::Option<()>, val: Value(Scalar(2)) }
47           goto -> bb1;                     // scope 0 at $DIR/invalid_constant.rs:10:20: 10:21
49   


thread '[mir-opt] mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs stdout ----
---- [mir-opt] mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs stdout ----
17           _1 = foo() -> bb1;               // scope 0 at $DIR/mutable_variable_aggregate_partial_read.rs:5:29: 5:34
18                                            // mir::Constant
19                                            // + span: $DIR/mutable_variable_aggregate_partial_read.rs:5:29: 5:32
-                                            // + literal: Const { ty: fn() -> (i32, i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> (i32, i32) {foo}, val: Value(Scalar(0)) }
22   
23       bb1: {


thread '[mir-opt] mir-opt/const_prop/mutable_variable_aggregate_partial_read.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_aggregate_partial_read.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/mutable_variable_unprop_assign.rs stdout ----
---- [mir-opt] mir-opt/const_prop/mutable_variable_unprop_assign.rs stdout ----
26           _1 = foo() -> bb1;               // scope 0 at $DIR/mutable_variable_unprop_assign.rs:5:13: 5:18
27                                            // mir::Constant
28                                            // + span: $DIR/mutable_variable_unprop_assign.rs:5:13: 5:16
-                                            // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(0)) }
31   
32       bb1: {


thread '[mir-opt] mir-opt/const_prop/mutable_variable_unprop_assign.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_unprop_assign.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/reify_fn_ptr.rs stdout ----
---- [mir-opt] mir-opt/const_prop/reify_fn_ptr.rs stdout ----
16           _3 = main as fn() (Pointer(ReifyFnPointer)); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:17
17                                            // mir::Constant
18                                            // + span: $DIR/reify_fn_ptr.rs:4:13: 4:17
-                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(0)) }
20           _2 = move _3 as usize (Misc);    // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:26
21           StorageDead(_3);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:25: 4:26
22           _1 = move _2 as *const fn() (Misc); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:41

thread '[mir-opt] mir-opt/const_prop/reify_fn_ptr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/reify_fn_ptr.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
---- [mir-opt] mir-opt/const_prop/scalar_literal_propagation.rs stdout ----
21 +         _2 = consume(const 1_u32) -> bb1; // scope 1 at $DIR/scalar_literal_propagation.rs:4:5: 4:15
22                                            // mir::Constant
23                                            // + span: $DIR/scalar_literal_propagation.rs:4:5: 4:12
-                                            // + literal: Const { ty: fn(u32) {consume}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(u32) {consume}, val: Value(Scalar(0)) }
26   
27       bb1: {


thread '[mir-opt] mir-opt/const_prop/scalar_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/scalar_literal_propagation.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
---- [mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
21           _2 = consume(move _3) -> bb1;    // scope 1 at $DIR/tuple_literal_propagation.rs:5:5: 5:15
22                                            // mir::Constant
23                                            // + span: $DIR/tuple_literal_propagation.rs:5:5: 5:12
-                                            // + literal: Const { ty: fn((u32, u32)) {consume}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn((u32, u32)) {consume}, val: Value(Scalar(0)) }
26   
27       bb1: {


thread '[mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/tuple_literal_propagation.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
16           _0 = foo(const -1_i32) -> bb3;   // scope 0 at $DIR/switch_int.rs:9:14: 9:21
17                                            // mir::Constant
18                                            // + span: $DIR/switch_int.rs:9:14: 9:17
-                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(0)) }
21   
22       bb2: {


23           _0 = foo(const 0_i32) -> bb3;    // scope 0 at $DIR/switch_int.rs:8:14: 8:20
24                                            // mir::Constant
25                                            // + span: $DIR/switch_int.rs:8:14: 8:17
-                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(i32) {foo}, val: Value(Scalar(0)) }
28   
29       bb3: {


thread '[mir-opt] mir-opt/const_prop/switch_int.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/switch_int.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/dest-prop/cycle.rs stdout ----
---- [mir-opt] mir-opt/dest-prop/cycle.rs stdout ----
32 +         _4 = val() -> bb1;               // scope 0 at $DIR/cycle.rs:9:17: 9:22
33                                            // mir::Constant
34                                            // + span: $DIR/cycle.rs:9:17: 9:20
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(0)) }
37   
38       bb1: {


thread '[mir-opt] mir-opt/dest-prop/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/cycle.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/dest-prop/union.rs stdout ----
---- [mir-opt] mir-opt/dest-prop/union.rs stdout ----
22           _2 = val() -> bb1;               // scope 0 at $DIR/union.rs:13:23: 13:28
23                                            // mir::Constant
24                                            // + span: $DIR/union.rs:13:23: 13:26
-                                            // + literal: Const { ty: fn() -> u32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> u32 {val}, val: Value(Scalar(0)) }
27   
28       bb1: {


thread '[mir-opt] mir-opt/dest-prop/union.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/union.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/dest-prop/branch.rs stdout ----
---- [mir-opt] mir-opt/dest-prop/branch.rs stdout ----
22 +         _2 = val() -> bb1;               // scope 0 at $DIR/branch.rs:13:13: 13:18
23                                            // mir::Constant
24                                            // + span: $DIR/branch.rs:13:13: 13:16
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(0)) }
27   
28       bb1: {


32           _3 = cond() -> bb2;              // scope 1 at $DIR/branch.rs:15:16: 15:22
33                                            // mir::Constant
34                                            // + span: $DIR/branch.rs:15:16: 15:20
-                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(0)) }
37   
38       bb2: {


50           _4 = val() -> bb5;               // scope 1 at $DIR/branch.rs:18:9: 18:14
51                                            // mir::Constant
52                                            // + span: $DIR/branch.rs:18:9: 18:12
-                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> i32 {val}, val: Value(Scalar(0)) }
55   
56       bb5: {


thread '[mir-opt] mir-opt/dest-prop/branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/branch.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/deduplicate_blocks.rs stdout ----
---- [mir-opt] mir-opt/deduplicate_blocks.rs stdout ----
27 +         _2 = transmute::<&str, &[u8]>(move _8) -> bb12; // scope 2 at $DIR/deduplicate_blocks.rs:3:11: 3:23
28                                            // mir::Constant
29                                            // + span: $DIR/deduplicate_blocks.rs:3:11: 3:23
-                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {std::intrinsics::transmute::<&str, &[u8]>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&str) -> &[u8] {std::intrinsics::transmute::<&str, &[u8]>}, val: Value(Scalar(0)) }
32   
33       bb1: {


thread '[mir-opt] mir-opt/deduplicate_blocks.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deduplicate_blocks.is_line_doc_comment_2.DeduplicateBlocks.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/dest-prop/copy_propagation_arg.rs stdout ----
---- [mir-opt] mir-opt/dest-prop/copy_propagation_arg.rs stdout ----
16 +         _1 = dummy(move _3) -> bb1;      // scope 0 at $DIR/copy_propagation_arg.rs:11:9: 11:17
17                                            // mir::Constant
18                                            // + span: $DIR/copy_propagation_arg.rs:11:9: 11:14
-                                            // + literal: Const { ty: fn(u8) -> u8 {dummy}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn(u8) -> u8 {dummy}, val: Value(Scalar(0)) }
21   
22       bb1: {


thread '[mir-opt] mir-opt/dest-prop/copy_propagation_arg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/copy_propagation_arg.foo.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/generator-storage-dead-unwind.rs stdout ----
---- [mir-opt] mir-opt/generator-storage-dead-unwind.rs stdout ----
38         _7 = take::<Foo>(move _8) -> [return: bb2, unwind: bb9]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:9: 26:16
39                                          // mir::Constant
40                                          // + span: $DIR/generator-storage-dead-unwind.rs:26:9: 26:13
-                                          // + literal: Const { ty: fn(Foo) {take::<Foo>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(Foo) {take::<Foo>}, val: Value(Scalar(0)) }
43 
44     bb2: {


50         _9 = take::<Bar>(move _10) -> [return: bb3, unwind: bb8]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:9: 27:16
51                                          // mir::Constant
52                                          // + span: $DIR/generator-storage-dead-unwind.rs:27:9: 27:13
-                                          // + literal: Const { ty: fn(Bar) {take::<Bar>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(Bar) {take::<Bar>}, val: Value(Scalar(0)) }
55 
56     bb3: {


thread '[mir-opt] mir-opt/generator-storage-dead-unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_storage_dead_unwind.main-{closure#0}.StateTransform.before.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] mir-opt/inline/cycle.rs stdout ----
16           _2 = <impl Fn() as Fn<()>>::call(move _3, move _4) -> [return: bb1, unwind: bb3]; // scope 0 at $DIR/cycle.rs:6:5: 6:8
17                                            // mir::Constant
18                                            // + span: $DIR/cycle.rs:6:5: 6:6
-                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl Fn(), ()) -> <impl Fn() as std::ops::FnOnce<()>>::Output {<impl Fn() as std::ops::Fn<()>>::call}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl Fn(), ()) -> <impl Fn() as std::ops::FnOnce<()>>::Output {<impl Fn() as std::ops::Fn<()>>::call}, val: Value(Scalar(0)) }
21   
22       bb1: {


thread '[mir-opt] mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/cycle.f.Inline.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
41           _4 = Formatter::sign_plus(move _5) -> bb1; // scope 0 at $DIR/funky_arms.rs:15:22: 15:37
42                                            // mir::Constant
43                                            // + span: $DIR/funky_arms.rs:15:26: 15:35
-                                            // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> bool {std::fmt::Formatter::sign_plus}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> bool {std::fmt::Formatter::sign_plus}, val: Value(Scalar(0)) }
46   
47       bb1: {


67           _7 = Formatter::precision(move _8) -> bb5; // scope 2 at $DIR/funky_arms.rs:24:30: 24:45
68                                            // mir::Constant
69                                            // + span: $DIR/funky_arms.rs:24:34: 24:43
-                                            // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> std::option::Option<usize> {std::fmt::Formatter::precision}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> fn(&'r std::fmt::Formatter) -> std::option::Option<usize> {std::fmt::Formatter::precision}, val: Value(Scalar(0)) }
72   
73       bb5: {


98           _0 = float_to_exponential_common_exact::<T>(move _11, move _12, move _13, move _14, move _17) -> bb7; // scope 2 at $DIR/funky_arms.rs:26:9: 26:87
99                                            // mir::Constant
100                                            // + span: $DIR/funky_arms.rs:26:9: 26:42
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, u32, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, u32, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(Scalar(0)) }
103   
104       bb7: {


123           _0 = float_to_exponential_common_shortest::<T>(move _18, move _19, move _20, move _21) -> bb9; // scope 2 at $DIR/funky_arms.rs:28:9: 28:68
124                                            // mir::Constant
125                                            // + span: $DIR/funky_arms.rs:28:9: 28:45
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut std::fmt::Formatter<'s>, &'t0 T, core::num::flt2dec::Sign, bool) -> std::result::Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(Scalar(0)) }
128   
129       bb9: {


thread '[mir-opt] mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
54         _8 = callee() -> bb4;            // scope 1 at $DIR/generator-tiny.rs:23:13: 23:21
55                                          // mir::Constant
56                                          // + span: $DIR/generator-tiny.rs:23:13: 23:19
-                                          // + literal: Const { ty: fn() {callee}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn() {callee}, val: Value(Scalar(0)) }
59 
60     bb4: {


thread '[mir-opt] mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_tiny.main-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-any-operand.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-any-operand.rs stdout ----
21         _1 = foo;                        // scope 0 at $DIR/inline-any-operand.rs:11:13: 11:16
22                                          // mir::Constant
23                                          // + span: $DIR/inline-any-operand.rs:11:13: 11:16
-                                          // + literal: Const { ty: fn(i32, i32) -> bool {foo}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(i32, i32) -> bool {foo}, val: Value(Scalar(0)) }
25         StorageLive(_2);                 // scope 1 at $DIR/inline-any-operand.rs:12:5: 12:6
26         _2 = _1;                         // scope 1 at $DIR/inline-any-operand.rs:12:5: 12:6
27         StorageLive(_3);                 // scope 1 at $DIR/inline-any-operand.rs:12:5: 12:13

thread '[mir-opt] mir-opt/inline/inline-any-operand.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_any_operand.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-instruction-set.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-instruction-set.rs stdout ----
14           _1 = instruction_set_a32() -> bb1; // scope 0 at $DIR/inline-instruction-set.rs:42:5: 42:26
15                                            // mir::Constant
16                                            // + span: $DIR/inline-instruction-set.rs:42:5: 42:24
-                                            // + literal: Const { ty: fn() {instruction_set_a32}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() {instruction_set_a32}, val: Value(Scalar(0)) }
19   
20       bb1: {


23 -         _2 = instruction_set_t32() -> bb2; // scope 0 at $DIR/inline-instruction-set.rs:43:5: 43:26
24 -                                          // mir::Constant
25 -                                          // + span: $DIR/inline-instruction-set.rs:43:5: 43:24
- -                                          // + literal: Const { ty: fn() {instruction_set_t32}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() {instruction_set_t32}, val: Value(Scalar(0)) }
28 - 
29 -     bb2: {


33 +         _3 = instruction_set_default() -> bb2; // scope 0 at $DIR/inline-instruction-set.rs:46:5: 46:30
34                                            // mir::Constant
35                                            // + span: $DIR/inline-instruction-set.rs:46:5: 46:28
-                                            // + literal: Const { ty: fn() {instruction_set_default}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() {instruction_set_default}, val: Value(Scalar(0)) }
38   
39 -     bb3: {


thread '[mir-opt] mir-opt/inline/inline-instruction-set.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_instruction_set.t32.Inline.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-cycle-generic.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-cycle-generic.rs stdout ----
15 +         _1 = <A as Call>::call() -> bb1; // scope 2 at $DIR/inline-cycle-generic.rs:9:5: 9:24
16                                            // mir::Constant
17 -                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
- -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(0)) }
19 +                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:24
- +                                          // + literal: Const { ty: fn() {<A as Call>::call}, val: Value(Scalar(<ZST>)) }
+ +                                          // + literal: Const { ty: fn() {<A as Call>::call}, val: Value(Scalar(0)) }
22   
23       bb1: {


thread '[mir-opt] mir-opt/inline/inline-cycle-generic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle_generic.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
14 -         _0 = <fn(A, B) as Clone>::clone(move _2) -> bb1; // scope 0 at $DIR/inline-shims.rs:6:5: 6:14
15 -                                          // mir::Constant
16 -                                          // + span: $DIR/inline-shims.rs:6:7: 6:12
- -                                          // + literal: Const { ty: for<'r> fn(&'r fn(A, B)) -> fn(A, B) {<fn(A, B) as std::clone::Clone>::clone}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: for<'r> fn(&'r fn(A, B)) -> fn(A, B) {<fn(A, B) as std::clone::Clone>::clone}, val: Value(Scalar(0)) }
19 - 
20 -     bb1: {


thread '[mir-opt] mir-opt/inline/inline-shims.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_shims.clone.Inline.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-specialization.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-specialization.rs stdout ----
15 -         _1 = <Vec<()> as Foo>::bar() -> bb1; // scope 0 at $DIR/inline-specialization.rs:5:13: 5:38
16 -                                          // mir::Constant
17 -                                          // + span: $DIR/inline-specialization.rs:5:13: 5:36
- -                                          // + literal: Const { ty: fn() -> u32 {<std::vec::Vec<()> as Foo>::bar}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() -> u32 {<std::vec::Vec<()> as Foo>::bar}, val: Value(Scalar(0)) }
20 - 
21 -     bb1: {


thread '[mir-opt] mir-opt/inline/inline-specialization.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_specialization.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
25           _4 = alloc::alloc::exchange_malloc(move _2, move _3) -> bb1; // scope 2 at $DIR/inline-into-box-place.rs:8:29: 8:43
26                                            // mir::Constant
27                                            // + span: $DIR/inline-into-box-place.rs:8:29: 8:43
-                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(0)) }
30   
31       bb1: {

41                                            // mir::Constant
41                                            // mir::Constant
42 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
43 -                                          // + user_ty: UserType(1)
- -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Scalar(0)) }
46 - 
47 -     bb2: {


72 -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> bb4; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
73 -                                          // mir::Constant
74 -                                          // + span: $DIR/inline-into-box-place.rs:8:42: 8:43
