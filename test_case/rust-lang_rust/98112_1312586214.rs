plain
 finished in 0.610 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 190 tests
...............................F......F..........iF............F.....................F.. 88/190
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
FF..F.....F...

---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
15           debug a => _1;                   // in scope 1 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:9: +1:10
16           scope 2 {
17               let _5: i32;                 // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:13: +3:15
+               let _10: *const ();          // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+               let _11: usize;              // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+               let _12: usize;              // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+               let _13: usize;              // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+               let _14: usize;              // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+               let _15: bool;               // in scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
18               scope 3 {
19                   debug _b => _5;          // in scope 3 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:13: +3:15

46   
47       bb1: {
47       bb1: {
48           _5 = (*_1)[_6];                  // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+           _10 = _1 as *const () (PtrToPtr); // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+           _11 = _10 as usize (PointerExposeAddress); // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ -         _12 = AlignOf(i32);              // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ -         _13 = Sub(_12, const 1_usize);   // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ -         _14 = BitAnd(_11, _13);          // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ +         _12 = const 4_usize;             // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ +         _13 = const 3_usize;             // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ +         _14 = BitAnd(_11, const 3_usize); // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+           _15 = Eq(_14, const 0_usize);    // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+           assert(_15, "attempt to dereference a misaligned pointer") -> bb2; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+   
+       bb2: {
+       bb2: {
49           StorageDead(_6);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:25: +3:26
50           nop;                             // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+2:5: +4:6
51           StorageDead(_5);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+4:5: +4:6

thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3447:21

---- [mir-opt] src/test/mir-opt/const_prop/boxes.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/boxes.rs stdout ----
12       let mut _7: std::boxed::Box<i32>;    // in scope 0 at $DIR/boxes.rs:+1:14: +1:22
13       let mut _8: *const i32;              // in scope 0 at $DIR/boxes.rs:+1:14: +1:22
14       let mut _9: *const i32;              // in scope 0 at $DIR/boxes.rs:+1:14: +1:22
+       let _10: *const ();                  // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _11: usize;                      // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _12: usize;                      // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _13: usize;                      // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _14: usize;                      // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _15: bool;                       // in scope 0 at $DIR/boxes.rs:+1:19: +1:21
+       let _16: *const ();                  // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
+       let _17: usize;                      // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
+       let _18: usize;                      // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
+       let _19: usize;                      // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
+       let _20: usize;                      // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
+       let _21: bool;                       // in scope 0 at $DIR/boxes.rs:+1:13: +1:22
15       scope 1 {
16           debug x => _1;                   // in scope 1 at $DIR/boxes.rs:+1:9: +1:10


37           _7 = ShallowInitBox(move _6, i32); // scope 0 at $DIR/boxes.rs:+1:14: +1:22
38           _8 = (((_7.0: std::ptr::Unique<i32>).0: std::ptr::NonNull<i32>).0: *const i32); // scope 0 at $DIR/boxes.rs:+1:19: +1:21
39           (*_8) = const 42_i32;            // scope 0 at $DIR/boxes.rs:+1:19: +1:21
-           _3 = move _7;                    // scope 0 at $DIR/boxes.rs:+1:14: +1:22
-           StorageDead(_7);                 // scope 0 at $DIR/boxes.rs:+1:21: +1:22
-           _9 = (((_3.0: std::ptr::Unique<i32>).0: std::ptr::NonNull<i32>).0: *const i32); // scope 0 at $DIR/boxes.rs:+1:13: +1:22
-           _2 = (*_9);                      // scope 0 at $DIR/boxes.rs:+1:13: +1:22
-           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/boxes.rs:+1:13: +1:26
-           StorageDead(_2);                 // scope 0 at $DIR/boxes.rs:+1:25: +1:26
-           drop(_3) -> [return: bb2, unwind: bb3]; // scope 0 at $DIR/boxes.rs:+1:26: +1:27
+           _10 = _8 as *const () (PtrToPtr); // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+           _11 = _10 as usize (PointerExposeAddress); // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+ -         _12 = AlignOf(i32);              // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+ -         _13 = Sub(_12, const 1_usize);   // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+ +         _12 = const 4_usize;             // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+ +         _13 = const 3_usize;             // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+           _14 = BitAnd(_11, _13);          // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+           _15 = Eq(_14, const 0_usize);    // scope 0 at $DIR/boxes.rs:+1:19: +1:21
+           assert(_15, "attempt to dereference a misaligned pointer") -> bb4; // scope 0 at $DIR/boxes.rs:+1:19: +1:21
48   
49       bb2: {

55   
55   
56       bb3 (cleanup): {
57           resume;                          // scope 0 at $DIR/boxes.rs:+0:1: +2:2
+   
+       bb4: {
+       bb4: {
+           _3 = move _7;                    // scope 0 at $DIR/boxes.rs:+1:14: +1:22
+           StorageDead(_7);                 // scope 0 at $DIR/boxes.rs:+1:21: +1:22
+           _9 = (((_3.0: std::ptr::Unique<i32>).0: std::ptr::NonNull<i32>).0: *const i32); // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           _2 = (*_9);                      // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           _16 = _9 as *const () (PtrToPtr); // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           _17 = _16 as usize (PointerExposeAddress); // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+ -         _18 = AlignOf(i32);              // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+ -         _19 = Sub(_18, const 1_usize);   // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+ +         _18 = const 4_usize;             // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+ +         _19 = const 3_usize;             // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           _20 = BitAnd(_17, _19);          // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           _21 = Eq(_20, const 0_usize);    // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+           assert(_21, "attempt to dereference a misaligned pointer") -> bb5; // scope 0 at $DIR/boxes.rs:+1:13: +1:22
+   
+       bb5: {
+       bb5: {
+           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/boxes.rs:+1:13: +1:26
+           StorageDead(_2);                 // scope 0 at $DIR/boxes.rs:+1:25: +1:26
+           drop(_3) -> [return: bb2, unwind: bb3]; // scope 0 at $DIR/boxes.rs:+1:26: +1:27
59   }
60   


thread '[mir-opt] src/test/mir-opt/const_prop/boxes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/boxes.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/const_prop/mutable_variable_no_prop.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/mutable_variable_no_prop.rs stdout ----
11           debug x => _1;                   // in scope 1 at $DIR/mutable_variable_no_prop.rs:+1:9: +1:14
12           let _5: u32;                     // in scope 1 at $DIR/mutable_variable_no_prop.rs:+5:9: +5:10
13           scope 2 {
+               let _6: *const ();           // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+               let _7: usize;               // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+               let _8: usize;               // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+               let _9: usize;               // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+               let _10: usize;              // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+               let _11: bool;               // in scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
15           scope 3 {
15           scope 3 {
16               debug y => _5;               // in scope 3 at $DIR/mutable_variable_no_prop.rs:+5:9: +5:10

28                                            // + span: $DIR/mutable_variable_no_prop.rs:10:13: 10:19
29                                            // + literal: Const { ty: *mut u32, val: Value(Scalar(alloc1)) }
30           _3 = (*_4);                      // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+           _6 = _4 as *const () (PtrToPtr); // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+           _7 = _6 as usize (PointerExposeAddress); // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ -         _8 = AlignOf(u32);               // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ -         _9 = Sub(_8, const 1_usize);     // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ -         _10 = BitAnd(_7, _9);            // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ +         _8 = const 4_usize;              // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ +         _9 = const 3_usize;              // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+ +         _10 = BitAnd(_7, const 3_usize); // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+           _11 = Eq(_10, const 0_usize);    // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+           assert(_11, "attempt to dereference a misaligned pointer") -> bb1; // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:13: +3:19
+   
+       bb1: {
+       bb1: {
31           _1 = move _3;                    // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:9: +3:19
32           StorageDead(_3);                 // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:18: +3:19
33           StorageDead(_4);                 // scope 2 at $DIR/mutable_variable_no_prop.rs:+3:19: +3:20

thread '[mir-opt] src/test/mir-opt/const_prop/mutable_variable_no_prop.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_no_prop.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/const_prop_miscompile.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop_miscompile.rs stdout ----
11           debug v => _1;                   // in scope 1 at $DIR/const_prop_miscompile.rs:+1:9: +1:14
12           let _4: bool;                    // in scope 1 at $DIR/const_prop_miscompile.rs:+5:9: +5:10
13           scope 2 {
+               let _6: *const ();           // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+               let _7: usize;               // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+               let _8: usize;               // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+               let _9: usize;               // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+               let _10: usize;              // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+               let _11: bool;               // in scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
15           scope 3 {
15           scope 3 {
16               debug y => _4;               // in scope 3 at $DIR/const_prop_miscompile.rs:+5:9: +5:10

25           StorageLive(_3);                 // scope 2 at $DIR/const_prop_miscompile.rs:+3:10: +3:22
26           _3 = &raw mut (_1.0: i32);       // scope 2 at $DIR/const_prop_miscompile.rs:+3:10: +3:22
27           (*_3) = const 5_i32;             // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+           _6 = _3 as *const () (PtrToPtr); // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+           _7 = _6 as usize (PointerExposeAddress); // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ -         _8 = AlignOf(i32);               // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ -         _9 = Sub(_8, const 1_usize);     // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ -         _10 = BitAnd(_7, _9);            // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ +         _8 = const 4_usize;              // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ +         _9 = const 3_usize;              // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+ +         _10 = BitAnd(_7, const 3_usize); // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+           _11 = Eq(_10, const 0_usize);    // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+           assert(_11, "attempt to dereference a misaligned pointer") -> bb1; // scope 2 at $DIR/const_prop_miscompile.rs:+3:9: +3:26
+   
+       bb1: {
+       bb1: {
28           StorageDead(_3);                 // scope 2 at $DIR/const_prop_miscompile.rs:+3:26: +3:27
29           nop;                             // scope 2 at $DIR/const_prop_miscompile.rs:+2:5: +4:6
30           StorageDead(_2);                 // scope 1 at $DIR/const_prop_miscompile.rs:+4:5: +4:6

thread '[mir-opt] src/test/mir-opt/const_prop_miscompile.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop_miscompile.bar.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_soundness.rs stdout ----
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_soundness.rs stdout ----
8       let mut _3: isize;                   // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+3:9: +3:16
9       let mut _4: isize;                   // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:13: +4:20
10       let _5: i32;                         // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _6: *const ();                   // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _7: usize;                       // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _8: usize;                       // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _9: usize;                       // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _10: usize;                      // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+       let _11: bool;                       // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
11       scope 1 {
12           debug v => _5;                   // in scope 1 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19

35       bb4: {
35       bb4: {
36           StorageLive(_5);                 // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
37           _5 = (((*_2) as Some).0: i32);   // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
-           _0 = _5;                         // scope 1 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
-           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
-           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
+           _6 = _2 as *const () (PtrToPtr); // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           _7 = _6 as usize (PointerExposeAddress); // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           _8 = AlignOf(std::option::Option<i32>); // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           _9 = Sub(_8, const 1_usize);     // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           _10 = BitAnd(_7, _9);            // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           _11 = Eq(_10, const 0_usize);    // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
+           assert(_11, "attempt to dereference a misaligned pointer") -> bb6; // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:18: +4:19
42   
43       bb5: {


44           return;                          // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+9:2: +9:2
+   
+       bb6: {
+       bb6: {
+           _0 = _5;                         // scope 1 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
+           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
+           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_soundness.rs:+4:24: +4:25
46   }
47   


thread '[mir-opt] src/test/mir-opt/early_otherwise_branch_soundness.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_soundness.no_deref_ptr.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/simplify_locals.rs stdout ----
---- [mir-opt] src/test/mir-opt/simplify_locals.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/simplify_locals.rs:+0:9: +0:9
6 -     let _1: u32;                         // in scope 0 at $DIR/simplify_locals.rs:+2:14: +2:15
7 -     let mut _2: *mut u32;                // in scope 0 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +     let mut _1: *mut u32;                // in scope 0 at $DIR/simplify_locals.rs:+2:14: +2:15
8       scope 1 {
+ -         let _3: *const ();               // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         let _2: *const ();               // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         let _3: usize;                   // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+           let _4: usize;                   // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+           let _5: usize;                   // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+           let _6: usize;                   // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         let _7: usize;                   // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         let _8: bool;                    // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         let _7: bool;                    // in scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
10   
11       bb0: {


13 -         StorageLive(_2);                 // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
14 -         _2 = &/*tls*/ mut X;             // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
15 -         _1 = (*_2);                      // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _3 = _2 as *const () (PtrToPtr); // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _4 = _3 as usize (PointerExposeAddress); // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _5 = AlignOf(u32);               // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _6 = Sub(_5, const 1_usize);     // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _7 = BitAnd(_4, _6);             // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         _8 = Eq(_7, const 0_usize);      // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ -         assert(_8, "attempt to dereference a misaligned pointer") -> bb1; // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         StorageLive(_1);                 // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _1 = &/*tls*/ mut X;             // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _2 = _1 as *const () (PtrToPtr); // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _3 = _2 as usize (PointerExposeAddress); // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _4 = AlignOf(u32);               // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _5 = Sub(_4, const 1_usize);     // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _6 = BitAnd(_3, _5);             // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         _7 = Eq(_6, const 0_usize);      // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+ +         assert(_7, "attempt to dereference a misaligned pointer") -> bb1; // scope 1 at $DIR/simplify_locals.rs:+2:14: +2:15
+   
+       bb1: {
+       bb1: {
16 -         StorageDead(_2);                 // scope 0 at $DIR/simplify_locals.rs:+2:17: +2:18
- -         StorageDead(_1);                 // scope 0 at $DIR/simplify_locals.rs:+2:17: +2:18
+           StorageDead(_1);                 // scope 0 at $DIR/simplify_locals.rs:+2:17: +2:18
18           _0 = const ();                   // scope 0 at $DIR/simplify_locals.rs:+0:9: +3:2
19           return;                          // scope 0 at $DIR/simplify_locals.rs:+3:2: +3:2


thread '[mir-opt] src/test/mir-opt/simplify_locals.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals.t1.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/tls_access.rs stdout ----
---- [mir-opt] src/test/mir-opt/tls_access.rs stdout ----
6     let mut _3: *mut u8;                 // in scope 0 at $DIR/tls_access.rs:+3:9: +3:12
7     scope 1 {
8         let _1: &u8;                     // in scope 1 at $DIR/tls_access.rs:+2:13: +2:14
+         let _4: *const ();               // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         let _5: usize;                   // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         let _6: usize;                   // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         let _7: usize;                   // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         let _8: usize;                   // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         let _9: bool;                    // in scope 1 at $DIR/tls_access.rs:+2:17: +2:21
9         scope 2 {
10             debug a => _1;               // in scope 2 at $DIR/tls_access.rs:+2:13: +2:14
+             let _10: *const ();          // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+             let _11: usize;              // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+             let _12: usize;              // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+             let _13: usize;              // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+             let _14: usize;              // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+             let _15: bool;               // in scope 2 at $DIR/tls_access.rs:+3:9: +3:17
12     }
13 


16         StorageLive(_2);                 // scope 1 at $DIR/tls_access.rs:+2:18: +2:21
17         _2 = &/*tls*/ mut FOO;           // scope 1 at $DIR/tls_access.rs:+2:18: +2:21
18         _1 = &(*_2);                     // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _4 = _2 as *const () (PtrToPtr); // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _5 = _4 as usize (PointerExposeAddress); // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _6 = AlignOf(u8);                // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _7 = Sub(_6, const 1_usize);     // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _8 = BitAnd(_5, _7);             // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         _9 = Eq(_8, const 0_usize);      // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+         assert(_9, "attempt to dereference a misaligned pointer") -> bb1; // scope 1 at $DIR/tls_access.rs:+2:17: +2:21
+ 
+     bb1: {
+     bb1: {
19         StorageLive(_3);                 // scope 2 at $DIR/tls_access.rs:+3:9: +3:12
20         _3 = &/*tls*/ mut FOO;           // scope 2 at $DIR/tls_access.rs:+3:9: +3:12
21         (*_3) = const 42_u8;             // scope 2 at $DIR/tls_access.rs:+3:9: +3:17

+         _10 = _3 as *const () (PtrToPtr); // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         _11 = _10 as usize (PointerExposeAddress); // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         _12 = AlignOf(u8);               // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         _13 = Sub(_12, const 1_usize);   // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         _14 = BitAnd(_11, _13);          // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         _15 = Eq(_14, const 0_usize);    // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+         assert(_15, "attempt to dereference a misaligned pointer") -> bb2; // scope 2 at $DIR/tls_access.rs:+3:9: +3:17
+ 
+     bb2: {
+     bb2: {
22         StorageDead(_3);                 // scope 2 at $DIR/tls_access.rs:+3:17: +3:18
23         _0 = const ();                   // scope 1 at $DIR/tls_access.rs:+1:5: +4:6
24         StorageDead(_2);                 // scope 1 at $DIR/tls_access.rs:+4:5: +4:6

thread '[mir-opt] src/test/mir-opt/tls_access.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/tls_access.main.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/uninhabited_enum.rs stdout ----
---- [mir-opt] src/test/mir-opt/uninhabited_enum.rs stdout ----
8         debug _input => _2;              // in scope 1 at $DIR/uninhabited_enum.rs:+1:8: +1:14
10     scope 2 {
10     scope 2 {
+         let _3: *const ();               // in scope 2 at $DIR/uninhabited_enum.rs:+1:26: +1:33
+         let _4: usize;                   // in scope 2 at $DIR/uninhabited_enum.rs:+1:26: +1:33
12 
13     bb0: {


+         StorageLive(_2);                 // scope 0 at $DIR/uninhabited_enum.rs:+1:8: +1:14
+         _3 = _1 as *const () (PtrToPtr); // scope 2 at $DIR/uninhabited_enum.rs:+1:26: +1:33
+         _4 = _3 as usize (PointerExposeAddress); // scope 2 at $DIR/uninhabited_enum.rs:+1:26: +1:33
14         unreachable;                     // scope 0 at $DIR/uninhabited_enum.rs:+0:39: +2:2
16 }


thread '[mir-opt] src/test/mir-opt/uninhabited_enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum.process_never.SimplifyLocals.after.mir', src/tools/compiletest/src/runtest.rs:3447:21
---- [mir-opt] src/test/mir-opt/unreachable_diverging.rs stdout ----
29   
30       bb1: {
30       bb1: {
31           _3 = discriminant(_2);           // scope 2 at $DIR/unreachable_diverging.rs:+2:12: +2:22
-           switchInt(move _3) -> [1_isize: bb2, otherwise: bb6]; // scope 2 at $DIR/unreachable_diverging.rs:+2:12: +2:22
+ -         switchInt(move _3) -> [1_isize: bb2, otherwise: bb6]; // scope 2 at $DIR/unreachable_diverging.rs:+2:12: +2:22
+ +         switchInt(move _3) -> [1_isize: bb2, otherwise: bb5]; // scope 2 at $DIR/unreachable_diverging.rs:+2:12: +2:22
34   
35       bb2: {

42       }
42       }
43   
44       bb3: {
-           _5 = loop_forever() -> bb5;      // scope 2 at $DIR/unreachable_diverging.rs:+4:13: +4:27
+ -         _5 = loop_forever() -> bb7;      // scope 2 at $DIR/unreachable_diverging.rs:+4:13: +4:27
+ +         _5 = loop_forever() -> bb6;      // scope 2 at $DIR/unreachable_diverging.rs:+4:13: +4:27
46                                            // mir::Constant
47                                            // + span: $DIR/unreachable_diverging.rs:16:13: 16:25
48                                            // + literal: Const { ty: fn() {loop_forever}, val: Value(<ZST>) }

58 -         StorageDead(_6);                 // scope 2 at $DIR/unreachable_diverging.rs:+5:9: +5:10
59 -         StorageDead(_5);                 // scope 2 at $DIR/unreachable_diverging.rs:+5:9: +5:10
60 -         StorageLive(_7);                 // scope 2 at $DIR/unreachable_diverging.rs:+6:9: +6:22
-           unreachable;                     // scope 2 at $DIR/unreachable_diverging.rs:+6:15: +6:19
-   
-       bb6: {
-       bb6: {
+ -         unreachable;                     // scope 2 at $DIR/unreachable_diverging.rs:+6:15: +6:19
+ -     }
+ -     bb6: {
+ -     bb6: {
65           _0 = const ();                   // scope 1 at $DIR/unreachable_diverging.rs:+7:6: +7:6
66           StorageDead(_1);                 // scope 0 at $DIR/unreachable_diverging.rs:+8:1: +8:2
67           StorageDead(_2);                 // scope 0 at $DIR/unreachable_diverging.rs:+8:1: +8:2

68           return;                          // scope 0 at $DIR/unreachable_diverging.rs:+8:2: +8:2
+   
+ -     bb7: {
+ -     bb7: {
+ -         goto -> bb5;                     // scope 2 at $DIR/unreachable_diverging.rs:+4:13: +4:27
+ +     bb6: {
+ +         unreachable;                     // scope 2 at $DIR/unreachable_diverging.rs:+4:13: +4:27
70   }
71   


thread '[mir-opt] src/test/mir-opt/unreachable_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unreachable_diverging.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3447:21

failures:
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] src/test/mir-opt/const_prop/boxes.rs
