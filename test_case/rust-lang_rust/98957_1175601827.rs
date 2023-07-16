plain
    Finished release [optimized] target(s) in 0.19s
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 180 tests
i...F...................................i.......................................i....... 88/180
........i........F...........F.F..............F............i............................ 176/180
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/array-index-is-temporary.rs stdout ----
---- [mir-opt] src/test/mir-opt/array-index-is-temporary.rs stdout ----
36         StorageLive(_5);                 // scope 3 at $DIR/array-index-is-temporary.rs:16:12: 16:29
37         StorageLive(_6);                 // scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
38         _6 = _3;                         // scope 4 at $DIR/array-index-is-temporary.rs:16:25: 16:26
-         _5 = foo(move _6) -> bb1;        // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
+         _5 = Zst: unsafe fn(*mut usize) -> u32 {foo}(move _6) -> bb1; // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
40                                          // mir::Constant
41                                          // + span: $DIR/array-index-is-temporary.rs:16:21: 16:24
-                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(<ZST>) }
44 
45     bb1: {


thread '[mir-opt] src/test/mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array_index_is_temporary.main.SimplifyCfg-elaborate-drops.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3466:25

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
25           StorageLive(_1);                 // scope 0 at $DIR/inline-into-box-place.rs:8:9: 8:11
26           _2 = SizeOf(std::vec::Vec<u32>); // scope 2 at $DIR/inline-into-box-place.rs:8:29: 8:43
27           _3 = AlignOf(std::vec::Vec<u32>); // scope 2 at $DIR/inline-into-box-place.rs:8:29: 8:43
-           _4 = alloc::alloc::exchange_malloc(move _2, move _3) -> bb1; // scope 2 at $DIR/inline-into-box-place.rs:8:29: 8:43
+           _4 = Zst: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}(move _2, move _3) -> bb1; // scope 2 at $DIR/inline-into-box-place.rs:8:29: 8:43
29                                            // mir::Constant
30                                            // + span: $DIR/inline-into-box-place.rs:8:29: 8:43
-                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(<ZST>) }
33   
34       bb1: {


36           _5 = ShallowInitBox(move _4, std::vec::Vec<u32>); // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
37           StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
38           _7 = (((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
- -         (*_7) = Vec::<u32>::new() -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
+ -         (*_7) = Zst: fn() -> Vec<u32> {Vec::<u32>::new}() -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
40 +         StorageLive(_9);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
41 +         _9 = &mut (*_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
42 +         StorageLive(_10);                // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
44                                            // mir::Constant
44                                            // mir::Constant
45 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
46 -                                          // + user_ty: UserType(1)
- -                                          // + literal: Const { ty: fn() -> Vec<u32> {Vec::<u32>::new}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() -> Vec<u32> {Vec::<u32>::new}, val: Value(<ZST>) }
49 - 
50 -     bb2: {

72   
72   
73 -     bb4 (cleanup): {
74 -         StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
- -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> bb5; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
+ -         _6 = Zst: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> bb5; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
76 -                                          // mir::Constant
77 -                                          // + span: $DIR/inline-into-box-place.rs:8:42: 8:43
- -                                          // + literal: Const { ty: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}, val: Value(<ZST>) }
80 - 
80 - 
81 -     bb5 (cleanup): {

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/issue-72181.rs stdout ----
22 
23     bb0: {
23     bb0: {
24         StorageLive(_1);                 // scope 0 at $DIR/issue-72181.rs:24:13: 24:34
-         _1 = std::mem::size_of::<Foo>() -> [return: bb1, unwind: bb3]; // scope 0 at $DIR/issue-72181.rs:24:13: 24:34
+         _1 = Zst: fn() -> usize {std::mem::size_of::<Foo>}() -> [return: bb1, unwind: bb3]; // scope 0 at $DIR/issue-72181.rs:24:13: 24:34
26                                          // mir::Constant
27                                          // + span: $DIR/issue-72181.rs:24:13: 24:32
-                                          // + literal: Const { ty: fn() -> usize {std::mem::size_of::<Foo>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn() -> usize {std::mem::size_of::<Foo>}, val: Value(<ZST>) }
30 
31     bb1: {


thread '[mir-opt] src/test/mir-opt/issue-72181.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181.main.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
132           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
133           Deinit(_27);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
134           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _21 = core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _21 = Zst: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
136                                            // mir::Constant
137                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(<ZST>) }
139                                            // mir::Constant
140                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
141                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
70         StorageLive(_8);                 // bb2[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
71         StorageLive(_9);                 // bb2[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
72         _9 = (*_6);                      // bb2[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _8 = ConstValue(Scalar(<ZST>): fn(usize) -> bool {use_x})(move _9) -> [return: bb3, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+         _8 = ConstValue(Zst: fn(usize) -> bool {use_x})(move _9) -> [return: bb3, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
74                                          // mir::Constant
75                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
-                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(<ZST>) }
78 
79     bb3: {


80         StorageDead(_9);                 // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
81         StorageDead(_8);                 // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
-         _0 = const ConstValue(Scalar(<ZST>): ()); // bb3[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
+         _0 = const ConstValue(Zst: ());  // bb3[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
83         goto -> bb6;                     // bb3[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
85 

86     bb4: {
86     bb4: {
87         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-         _10 = ConstValue(Scalar(<ZST>): fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x00000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+         _10 = ConstValue(Zst: fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x00000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
89                                          // mir::Constant
90                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
-                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(<ZST>) }
93 
94     bb5: {


95         StorageDead(_10);                // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
-         _0 = const ConstValue(Scalar(<ZST>): ()); // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
+         _0 = const ConstValue(Zst: ());  // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
97         goto -> bb6;                     // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
99 


thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
31 
32     bb6: {
32     bb6: {
33         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _3 = <Vec<i32> as Drop>::drop(move _2) -> [return: bb5, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         _3 = Zst: for<'r> fn(&'r mut Vec<i32>) {<Vec<i32> as Drop>::drop}(move _2) -> [return: bb5, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
35                                          // mir::Constant
36                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'r> fn(&'r mut Vec<i32>) {<Vec<i32> as Drop>::drop}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: for<'r> fn(&'r mut Vec<i32>) {<Vec<i32> as Drop>::drop}, val: Value(<ZST>) }
39 }
40 


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.32bit.mir', src/tools/compiletest/src/runtest.rs:3466:25

failures:
    [mir-opt] src/test/mir-opt/array-index-is-temporary.rs
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
