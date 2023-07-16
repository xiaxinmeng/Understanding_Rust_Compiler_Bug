plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 164 tests
..........F........................i...................F......F.........F....F..i.....FFF....FFFF... 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.......F..........F.....FF....i.......F.........F..FF...........

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
3   
3   
4   static mut BAR: *const &i32 = {
5       let mut _0: *const &i32;             // return place in scope 0 at $DIR/const-promotion-extern-static.rs:9:17: 9:28
-       let mut _1: &[&i32];                 // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
-       let mut _2: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+       let mut _1: &[&i32];                 // in scope 0 at $DIR/const-promotion-extern-static.rs:9:36: 9:42
+       let mut _2: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
8       let _3: [&i32; 1];                   // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
9       let mut _4: &i32;                    // in scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
10       let _5: &i32;                        // in scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34

- +     let mut _6: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ +     let mut _6: &[&i32; 1];              // in scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
13       bb0: {
13       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
-           StorageLive(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+           StorageLive(_1);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:36: 9:42
+           StorageLive(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
16 -         StorageLive(_3);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
17 -         StorageLive(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
18 -         StorageLive(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34

19 -         _5 = const {alloc1: &i32};       // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
- +         _6 = const BAR::promoted[0];     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ +         _6 = const BAR::promoted[0];     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
21                                            // ty::Const
22 -                                          // + ty: &i32
23 -                                          // + val: Value(Scalar(alloc1))

28 -                                          // + literal: Const { ty: &i32, val: Value(Scalar(alloc1)) }
29 -         _4 = &(*_5);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
30 -         _3 = [move _4];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
- -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
- +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+ +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:44
33 +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[55e6]::BAR), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
- +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
-           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:36: 9:42
36 -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
37           StorageDead(_2);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35
38           _0 = core::slice::<impl [&i32]>::as_ptr(move _1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44

thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/deduplicate_blocks.rs stdout ----
---- [mir-opt] mir-opt/deduplicate_blocks.rs stdout ----
5       debug s => _1;                       // in scope 0 at $DIR/deduplicate_blocks.rs:2:36: 2:37
6       let mut _0: bool;                    // return place in scope 0 at $DIR/deduplicate_blocks.rs:2:48: 2:52
7       let mut _2: &[u8];                   // in scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
-       let mut _3: &str;                    // in scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:12
+       let mut _3: &str;                    // in scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
9       let mut _4: usize;                   // in scope 0 at $DIR/deduplicate_blocks.rs:5:9: 5:31
10       let mut _5: bool;                    // in scope 0 at $DIR/deduplicate_blocks.rs:5:9: 5:31
11       let mut _6: usize;                   // in scope 0 at $DIR/deduplicate_blocks.rs:4:9: 4:37
19   
20       bb0: {
20       bb0: {
21           StorageLive(_2);                 // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
-           StorageLive(_3);                 // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:12
-           _3 = _1;                         // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:12
+           StorageLive(_3);                 // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
+           _3 = _1;                         // scope 0 at $DIR/deduplicate_blocks.rs:3:11: 3:23
24           StorageLive(_8);                 // scope 2 at $DIR/deduplicate_blocks.rs:3:11: 3:23
25           _8 = _3;                         // scope 2 at $DIR/deduplicate_blocks.rs:3:11: 3:23
26 -         _2 = transmute::<&str, &[u8]>(move _8) -> bb14; // scope 2 at $DIR/deduplicate_blocks.rs:3:11: 3:23

thread '[mir-opt] mir-opt/deduplicate_blocks.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deduplicate_blocks.is_line_doc_comment_2.DeduplicateBlocks.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
7       debug upper => _3;                   // in scope 0 at $DIR/funky_arms.rs:11:69: 11:74
8       let mut _0: std::result::Result<(), std::fmt::Error>; // return place in scope 0 at $DIR/funky_arms.rs:11:85: 11:91
9       let _4: bool;                        // in scope 0 at $DIR/funky_arms.rs:15:9: 15:19
-       let mut _5: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:15:22: 15:25
+       let mut _5: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:15:22: 15:37
11       let mut _7: std::option::Option<usize>; // in scope 0 at $DIR/funky_arms.rs:24:30: 24:45
-       let mut _8: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:24:30: 24:33
+       let mut _8: &std::fmt::Formatter;    // in scope 0 at $DIR/funky_arms.rs:24:30: 24:45
13       let mut _9: isize;                   // in scope 0 at $DIR/funky_arms.rs:24:12: 24:27
14       let mut _11: &mut std::fmt::Formatter; // in scope 0 at $DIR/funky_arms.rs:26:43: 26:46
15       let mut _12: &T;                     // in scope 0 at $DIR/funky_arms.rs:26:48: 26:51
36   
37       bb0: {
37       bb0: {
38           StorageLive(_4);                 // scope 0 at $DIR/funky_arms.rs:15:9: 15:19
-           StorageLive(_5);                 // scope 0 at $DIR/funky_arms.rs:15:22: 15:25
-           _5 = &(*_1);                     // scope 0 at $DIR/funky_arms.rs:15:22: 15:25
+           StorageLive(_5);                 // scope 0 at $DIR/funky_arms.rs:15:22: 15:37
+           _5 = &(*_1);                     // scope 0 at $DIR/funky_arms.rs:15:22: 15:37
41           _4 = Formatter::sign_plus(move _5) -> bb1; // scope 0 at $DIR/funky_arms.rs:15:22: 15:37
42                                            // mir::Constant
43                                            // + span: $DIR/funky_arms.rs:15:26: 15:35
62   
63       bb4: {
63       bb4: {
64           StorageLive(_7);                 // scope 2 at $DIR/funky_arms.rs:24:30: 24:45
-           StorageLive(_8);                 // scope 2 at $DIR/funky_arms.rs:24:30: 24:33
-           _8 = &(*_1);                     // scope 2 at $DIR/funky_arms.rs:24:30: 24:33
+           StorageLive(_8);                 // scope 2 at $DIR/funky_arms.rs:24:30: 24:45
+           _8 = &(*_1);                     // scope 2 at $DIR/funky_arms.rs:24:30: 24:45
67           _7 = Formatter::precision(move _8) -> bb5; // scope 2 at $DIR/funky_arms.rs:24:30: 24:45
68                                            // mir::Constant
69                                            // + span: $DIR/funky_arms.rs:24:34: 24:43

thread '[mir-opt] mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] mir-opt/inline/cycle.rs stdout ----
5       debug g => _1;                       // in scope 0 at $DIR/cycle.rs:5:6: 5:7
6       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:5:20: 5:20
7       let _2: ();                          // in scope 0 at $DIR/cycle.rs:6:5: 6:8
-       let mut _3: &impl Fn();              // in scope 0 at $DIR/cycle.rs:6:5: 6:6
+       let mut _3: &impl Fn();              // in scope 0 at $DIR/cycle.rs:6:5: 6:8
9       let mut _4: ();                      // in scope 0 at $DIR/cycle.rs:6:5: 6:8
11       bb0: {


12           StorageLive(_2);                 // scope 0 at $DIR/cycle.rs:6:5: 6:8
-           StorageLive(_3);                 // scope 0 at $DIR/cycle.rs:6:5: 6:6
-           _3 = &_1;                        // scope 0 at $DIR/cycle.rs:6:5: 6:6
+           StorageLive(_3);                 // scope 0 at $DIR/cycle.rs:6:5: 6:8
+           _3 = &_1;                        // scope 0 at $DIR/cycle.rs:6:5: 6:8
15           StorageLive(_4);                 // scope 0 at $DIR/cycle.rs:6:5: 6:8
16           _2 = <impl Fn() as Fn<()>>::call(move _3, move _4) -> [return: bb1, unwind: bb3]; // scope 0 at $DIR/cycle.rs:6:5: 6:8
17                                            // mir::Constant

thread '[mir-opt] mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/cycle.f.Inline.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-shims.rs stdout ----
4   fn clone(_1: fn(A, B)) -> fn(A, B) {
5       debug f => _1;                       // in scope 0 at $DIR/inline-shims.rs:5:20: 5:21
6       let mut _0: fn(A, B);                // return place in scope 0 at $DIR/inline-shims.rs:5:36: 5:44
-       let mut _2: &fn(A, B);               // in scope 0 at $DIR/inline-shims.rs:6:5: 6:6
+       let mut _2: &fn(A, B);               // in scope 0 at $DIR/inline-shims.rs:6:5: 6:14
8 +     scope 1 (inlined <fn(A, B) as Clone>::clone - shim(fn(A, B))) { // at $DIR/inline-shims.rs:6:5: 6:14
9 +     }

11       bb0: {
11       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/inline-shims.rs:6:5: 6:6
-           _2 = &_1;                        // scope 0 at $DIR/inline-shims.rs:6:5: 6:6
+           StorageLive(_2);                 // scope 0 at $DIR/inline-shims.rs:6:5: 6:14
+           _2 = &_1;                        // scope 0 at $DIR/inline-shims.rs:6:5: 6:14
14 -         _0 = <fn(A, B) as Clone>::clone(move _2) -> bb1; // scope 0 at $DIR/inline-shims.rs:6:5: 6:14
15 -                                          // mir::Constant
16 -                                          // + span: $DIR/inline-shims.rs:6:7: 6:12

thread '[mir-opt] mir-opt/inline/inline-shims.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_shims.clone.Inline.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/inline-closure.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-closure.rs stdout ----
5     debug q => _2;                       // in scope 0 at $DIR/inline-closure.rs:10:24: 10:25
6     let mut _0: i32;                     // return place in scope 0 at $DIR/inline-closure.rs:10:35: 10:38
7     let _3: [closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure.rs:11:9: 11:10
-     let mut _4: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure.rs:12:5: 12:6
+     let mut _4: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure.rs:12:5: 12:12
9     let mut _5: (i32, i32);              // in scope 0 at $DIR/inline-closure.rs:12:5: 12:12
10     let mut _6: i32;                     // in scope 0 at $DIR/inline-closure.rs:12:7: 12:8
11     let mut _7: i32;                     // in scope 0 at $DIR/inline-closure.rs:12:10: 12:11
21 
22     bb0: {
22     bb0: {
23         StorageLive(_3);                 // scope 0 at $DIR/inline-closure.rs:11:9: 11:10
-         StorageLive(_4);                 // scope 1 at $DIR/inline-closure.rs:12:5: 12:6
-         _4 = &_3;                        // scope 1 at $DIR/inline-closure.rs:12:5: 12:6
+         StorageLive(_4);                 // scope 1 at $DIR/inline-closure.rs:12:5: 12:12
+         _4 = &_3;                        // scope 1 at $DIR/inline-closure.rs:12:5: 12:12
26         StorageLive(_5);                 // scope 1 at $DIR/inline-closure.rs:12:5: 12:12
27         StorageLive(_6);                 // scope 1 at $DIR/inline-closure.rs:12:7: 12:8
28         _6 = _2;                         // scope 1 at $DIR/inline-closure.rs:12:7: 12:8

thread '[mir-opt] mir-opt/inline/inline-closure.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_closure.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
5     debug q => _2;                       // in scope 0 at $DIR/inline-closure-borrows-arg.rs:11:24: 11:25
6     let mut _0: i32;                     // return place in scope 0 at $DIR/inline-closure-borrows-arg.rs:11:36: 11:39
7     let _3: [closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-borrows-arg.rs:12:9: 12:10
-     let mut _4: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:6
+     let mut _4: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
9     let mut _5: (&i32, &i32);            // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
10     let mut _6: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:7: 16:8
11     let mut _7: &i32;                    // in scope 0 at $DIR/inline-closure-borrows-arg.rs:16:10: 16:11
25 
26     bb0: {
26     bb0: {
27         StorageLive(_3);                 // scope 0 at $DIR/inline-closure-borrows-arg.rs:12:9: 12:10
-         StorageLive(_4);                 // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:6
-         _4 = &_3;                        // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:6
+         StorageLive(_4);                 // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
+         _4 = &_3;                        // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
30         StorageLive(_5);                 // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:5: 16:12
31         StorageLive(_6);                 // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:7: 16:8
32         _6 = &(*_2);                     // scope 1 at $DIR/inline-closure-borrows-arg.rs:16:7: 16:8

thread '[mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
7     let _3: [closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-captures.rs:11:9: 11:10
8     let mut _4: &i32;                    // in scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
9     let mut _5: &T;                      // in scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
-     let mut _6: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:6
+     let mut _6: &[closure@foo<T>::{closure#0}]; // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9
11     let mut _7: (i32,);                  // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9
12     let mut _8: i32;                     // in scope 0 at $DIR/inline-closure-captures.rs:12:7: 12:8
13     let mut _9: i32;                     // in scope 0 at $DIR/inline-closure-captures.rs:12:5: 12:9

32         (_3.1: &T) = move _5;            // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
33         StorageDead(_5);                 // scope 0 at $DIR/inline-closure-captures.rs:11:23: 11:24
34         StorageDead(_4);                 // scope 0 at $DIR/inline-closure-captures.rs:11:23: 11:24
-         StorageLive(_6);                 // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:6
-         _6 = &_3;                        // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:6
+         StorageLive(_6);                 // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
+         _6 = &_3;                        // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
37         StorageLive(_7);                 // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9
38         StorageLive(_8);                 // scope 1 at $DIR/inline-closure-captures.rs:12:7: 12:8
39         _8 = _2;                         // scope 1 at $DIR/inline-closure-captures.rs:12:7: 12:8

thread '[mir-opt] mir-opt/inline/inline-closure-captures.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_closure_captures.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/issue-76997-inline-scopes-parenting.rs stdout ----
3 fn main() -> () {
3 fn main() -> () {
4     let mut _0: ();                      // return place in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:4:11: 4:11
5     let _1: [closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:33]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:5:9: 5:10
-     let mut _2: &[closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:33]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:6
+     let mut _2: &[closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:33]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
7     let mut _3: ((),);                   // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
8     let mut _4: ();                      // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:7: 6:9
9     let mut _5: ();                      // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
20 
21     bb0: {
21     bb0: {
22         StorageLive(_1);                 // scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:5:9: 5:10
-         StorageLive(_2);                 // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:6
-         _2 = &_1;                        // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:6
+         StorageLive(_2);                 // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
+         _2 = &_1;                        // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
25         StorageLive(_3);                 // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
26         StorageLive(_4);                 // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:7: 6:9
27         (_3.0: ()) = move _4;            // scope 1 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10

thread '[mir-opt] mir-opt/inline/issue-76997-inline-scopes-parenting.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_76997_inline_scopes_parenting.main.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/inline-trait-method.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-trait-method.rs stdout ----
3 fn test(_1: &dyn X) -> u32 {
4     debug x => _1;                       // in scope 0 at $DIR/inline-trait-method.rs:8:9: 8:10
5     let mut _0: u32;                     // return place in scope 0 at $DIR/inline-trait-method.rs:8:23: 8:26
-     let mut _2: &dyn X;                  // in scope 0 at $DIR/inline-trait-method.rs:9:5: 9:6
+     let mut _2: &dyn X;                  // in scope 0 at $DIR/inline-trait-method.rs:9:5: 9:10
8     bb0: {
8     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/inline-trait-method.rs:9:5: 9:6
-         _2 = &(*_1);                     // scope 0 at $DIR/inline-trait-method.rs:9:5: 9:6
+         StorageLive(_2);                 // scope 0 at $DIR/inline-trait-method.rs:9:5: 9:10
+         _2 = &(*_1);                     // scope 0 at $DIR/inline-trait-method.rs:9:5: 9:10
11         _0 = <dyn X as X>::y(move _2) -> bb1; // scope 0 at $DIR/inline-trait-method.rs:9:5: 9:10
12                                          // mir::Constant
13                                          // + span: $DIR/inline-trait-method.rs:9:7: 9:8

thread '[mir-opt] mir-opt/inline/inline-trait-method.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_trait_method.test.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/issue-49232.rs stdout ----
43     }
44 
45     bb6: {
45     bb6: {
-         unreachable;                     // scope 0 at $DIR/issue-49232.rs:10:25: 10:30
+         unreachable;                     // scope 0 at $DIR/issue-49232.rs:8:13: 11:14
48 
49     bb7: {


thread '[mir-opt] mir-opt/issue-49232.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_49232.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/issue-72181-1.rs stdout ----
19 
20     bb2: {
20     bb2: {
21         StorageDead(_3);                 // scope 0 at $DIR/issue-72181-1.rs:11:14: 11:15
-         unreachable;                     // scope 0 at $DIR/issue-72181-1.rs:10:20: 12:2
+         unreachable;                     // scope 0 at $DIR/issue-72181-1.rs:11:5: 11:15
24 
25     bb3: {


thread '[mir-opt] mir-opt/issue-72181-1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181_1.f.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/lower_slice_len.rs stdout ----
---- [mir-opt] mir-opt/lower_slice_len.rs stdout ----
8       let mut _3: bool;                    // in scope 0 at $DIR/lower_slice_len.rs:5:8: 5:27
9       let mut _4: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:5:8: 5:13
10       let mut _5: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
-       let mut _6: &[u8];                   // in scope 0 at $DIR/lower_slice_len.rs:5:16: 5:21
+       let mut _6: &[u8];                   // in scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
12       let _7: usize;                       // in scope 0 at $DIR/lower_slice_len.rs:6:15: 6:20
13       let mut _8: usize;                   // in scope 0 at $DIR/lower_slice_len.rs:6:9: 6:21
14       let mut _9: bool;                    // in scope 0 at $DIR/lower_slice_len.rs:6:9: 6:21

18           StorageLive(_4);                 // scope 0 at $DIR/lower_slice_len.rs:5:8: 5:13
19           _4 = _1;                         // scope 0 at $DIR/lower_slice_len.rs:5:8: 5:13
20           StorageLive(_5);                 // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
-           StorageLive(_6);                 // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:21
-           _6 = &(*_2);                     // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:21
+           StorageLive(_6);                 // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
+           _6 = &(*_2);                     // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
23 -         _5 = core::slice::<impl [u8]>::len(move _6) -> bb1; // scope 0 at $DIR/lower_slice_len.rs:5:16: 5:27
24 -                                          // mir::Constant
25 -                                          // + span: $DIR/lower_slice_len.rs:5:22: 5:25

thread '[mir-opt] mir-opt/lower_slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_slice_len.bound.LowerSliceLenCalls.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs stdout ----
---- [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs stdout ----
5     let mut _0: &mut [T];                // return place in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:2:29: 2:37
6     let mut _2: &mut [T];                // in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
7     let mut _3: &mut [T];                // in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
-     let mut _4: &mut [T];                // in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
+     let mut _4: &mut [T];                // in scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
9     scope 1 (inlined <[T] as AsMut<[T]>>::as_mut) { // at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
10         debug self => _4;                // in scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
11         let mut _5: &mut [T];            // in scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
14     bb0: {
14     bb0: {
15         StorageLive(_2);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
16         StorageLive(_3);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
-         StorageLive(_4);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
-         _4 = &mut (*_1);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:6
+         StorageLive(_4);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
+         _4 = &mut (*_1);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
19         StorageLive(_5);                 // scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
20         _5 = &mut (*_4);                 // scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15
21         _3 = &mut (*_5);                 // scope 1 at $DIR/issue-58867-inline-as-ref-as-mut.rs:3:5: 3:15

thread '[mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_58867_inline_as_ref_as_mut.a.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
4     let mut _0: ();                      // return place in scope 0 at $DIR/no-spurious-drop-after-call.rs:8:11: 8:11
5     let _1: ();                          // in scope 0 at $DIR/no-spurious-drop-after-call.rs:9:5: 9:35
6     let mut _2: std::string::String;     // in scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
-     let mut _3: &str;                    // in scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
+     let mut _3: &str;                    // in scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
8     let _4: &str;                        // in scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
10     bb0: {


11         StorageLive(_1);                 // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:5: 9:35
12         StorageLive(_2);                 // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
-         StorageLive(_3);                 // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
+         StorageLive(_3);                 // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
14         StorageLive(_4);                 // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
15         _4 = const "";                   // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
16                                          // ty::Const
19                                          // mir::Constant
19                                          // mir::Constant
20                                          // + span: $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
21                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
-         _3 = &(*_4);                     // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
+         _3 = &(*_4);                     // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
23         _2 = <str as ToString>::to_string(move _3) -> bb1; // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
24                                          // mir::Constant
25                                          // + span: $DIR/no-spurious-drop-after-call.rs:9:23: 9:32

thread '[mir-opt] mir-opt/no-spurious-drop-after-call.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_spurious_drop_after_call.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/receiver-ptr-mutability.rs stdout ----
---- [mir-opt] mir-opt/receiver-ptr-mutability.rs stdout ----
10     let mut _0: ();                      // return place in scope 0 at $DIR/receiver-ptr-mutability.rs:13:11: 13:11
11     let _1: *mut Test as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 0 at $DIR/receiver-ptr-mutability.rs:14:9: 14:12
12     let _2: ();                          // in scope 0 at $DIR/receiver-ptr-mutability.rs:15:5: 15:12
-     let mut _3: *const Test;             // in scope 0 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
+     let mut _3: *const Test;             // in scope 0 at $DIR/receiver-ptr-mutability.rs:15:9: 15:10
14     let mut _4: *mut Test;               // in scope 0 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
15     let _6: &&&&*mut Test;               // in scope 0 at $DIR/receiver-ptr-mutability.rs:18:34: 18:41
16     let _7: &&&*mut Test;                // in scope 0 at $DIR/receiver-ptr-mutability.rs:18:35: 18:41

17     let _8: &&*mut Test;                 // in scope 0 at $DIR/receiver-ptr-mutability.rs:18:36: 18:41
18     let _9: &*mut Test;                  // in scope 0 at $DIR/receiver-ptr-mutability.rs:18:37: 18:41
19     let _10: ();                         // in scope 0 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
-     let mut _11: *const Test;            // in scope 0 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
-     let mut _12: *mut Test;              // in scope 0 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
+     let mut _11: *const Test;            // in scope 0 at $DIR/receiver-ptr-mutability.rs:19:13: 19:14
+     let mut _12: *mut Test;              // in scope 0 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
22     scope 1 {
23         debug ptr => _1;                 // in scope 1 at $DIR/receiver-ptr-mutability.rs:14:9: 14:12
24         let _5: &&&&*mut Test as UserTypeProjection { base: UserType(2), projs: [] }; // in scope 1 at $DIR/receiver-ptr-mutability.rs:18:9: 18:16

39         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/receiver-ptr-mutability.rs:14:9: 14:12
40         AscribeUserType(_1, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/receiver-ptr-mutability.rs:14:14: 14:23
41         StorageLive(_2);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:12
-         StorageLive(_3);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
+         StorageLive(_3);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:9: 15:10
43         StorageLive(_4);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
44         _4 = _1;                         // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
-         _3 = move _4 as *const Test (Pointer(MutToConstPointer)); // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:8
+         _3 = move _4 as *const Test (Pointer(MutToConstPointer)); // scope 1 at $DIR/receiver-ptr-mutability.rs:15:9: 15:10
46         StorageDead(_4);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:15:7: 15:8
47         _2 = Test::x(move _3) -> [return: bb2, unwind: bb4]; // scope 1 at $DIR/receiver-ptr-mutability.rs:15:5: 15:12
48                                          // mir::Constant

67         AscribeUserType(_5, o, UserTypeProjection { base: UserType(3), projs: [] }); // scope 1 at $DIR/receiver-ptr-mutability.rs:18:18: 18:31
68         StorageDead(_6);                 // scope 1 at $DIR/receiver-ptr-mutability.rs:18:41: 18:42
69         StorageLive(_10);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
-         StorageLive(_11);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
-         StorageLive(_12);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
-         _12 = (*(*(*(*_5))));            // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
-         _11 = move _12 as *const Test (Pointer(MutToConstPointer)); // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:12
+         StorageLive(_11);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:13: 19:14
+         StorageLive(_12);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
+         _12 = (*(*(*(*_5))));            // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
+         _11 = move _12 as *const Test (Pointer(MutToConstPointer)); // scope 2 at $DIR/receiver-ptr-mutability.rs:19:13: 19:14
74         StorageDead(_12);                // scope 2 at $DIR/receiver-ptr-mutability.rs:19:11: 19:12
75         _10 = Test::x(move _11) -> [return: bb3, unwind: bb4]; // scope 2 at $DIR/receiver-ptr-mutability.rs:19:5: 19:16
76                                          // mir::Constant

thread '[mir-opt] mir-opt/receiver-ptr-mutability.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/receiver_ptr_mutability.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
41 -     }
42 - 
43 -     bb6: {
43 -     bb6: {
- -         unreachable;                     // scope 0 at $DIR/simplify_cfg.rs:9:18: 11:10
+ -         unreachable;                     // scope 0 at $DIR/simplify_cfg.rs:9:9: 11:10
45 -     }
47 -     bb7: {


thread '[mir-opt] mir-opt/simplify_cfg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_cfg.main.SimplifyCfg-initial.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/retag.rs stdout ----
---- [mir-opt] mir-opt/retag.rs stdout ----
4     let mut _0: ();                      // return place in scope 0 at $DIR/retag.rs:29:11: 29:11
5     let mut _1: i32;                     // in scope 0 at $DIR/retag.rs:30:9: 30:14
6     let _2: ();                          // in scope 0 at $DIR/retag.rs:31:5: 37:6
-     let mut _4: &Test;                   // in scope 0 at $DIR/retag.rs:32:17: 32:24
+     let mut _4: &Test;                   // in scope 0 at $DIR/retag.rs:32:17: 32:36
8     let _5: Test;                        // in scope 0 at $DIR/retag.rs:32:17: 32:24
9     let mut _6: &mut i32;                // in scope 0 at $DIR/retag.rs:32:29: 32:35
10     let mut _7: &mut i32;                // in scope 0 at $DIR/retag.rs:32:29: 32:35

15     let mut _17: &i32;                   // in scope 0 at $DIR/retag.rs:44:16: 44:18
16     let _18: &i32;                       // in scope 0 at $DIR/retag.rs:44:16: 44:18
17     let _19: &i32;                       // in scope 0 at $DIR/retag.rs:47:5: 47:24
-     let mut _20: &Test;                  // in scope 0 at $DIR/retag.rs:47:5: 47:12
+     let mut _20: &Test;                  // in scope 0 at $DIR/retag.rs:47:5: 47:24
19     let _21: Test;                       // in scope 0 at $DIR/retag.rs:47:5: 47:12
20     let mut _22: &i32;                   // in scope 0 at $DIR/retag.rs:47:21: 47:23
21     let _23: &i32;                       // in scope 0 at $DIR/retag.rs:47:21: 47:23

60         _1 = const 0_i32;                // scope 0 at $DIR/retag.rs:30:17: 30:18
61         StorageLive(_2);                 // scope 1 at $DIR/retag.rs:31:5: 37:6
62         StorageLive(_3);                 // scope 1 at $DIR/retag.rs:32:13: 32:14
-         StorageLive(_4);                 // scope 1 at $DIR/retag.rs:32:17: 32:24
+         StorageLive(_4);                 // scope 1 at $DIR/retag.rs:32:17: 32:36
64         StorageLive(_5);                 // scope 1 at $DIR/retag.rs:32:17: 32:24
65         _5 = Test(const 0_i32);          // scope 1 at $DIR/retag.rs:32:17: 32:24
-         _4 = &_5;                        // scope 1 at $DIR/retag.rs:32:17: 32:24
-         Retag(_4);                       // scope 1 at $DIR/retag.rs:32:17: 32:24
+         _4 = &_5;                        // scope 1 at $DIR/retag.rs:32:17: 32:36
+         Retag(_4);                       // scope 1 at $DIR/retag.rs:32:17: 32:36
68         StorageLive(_6);                 // scope 1 at $DIR/retag.rs:32:29: 32:35
69         StorageLive(_7);                 // scope 1 at $DIR/retag.rs:32:29: 32:35
70         _7 = &mut _1;                    // scope 1 at $DIR/retag.rs:32:29: 32:35

91         StorageLive(_9);                 // scope 2 at $DIR/retag.rs:33:19: 33:20
92         _9 = move _3;                    // scope 2 at $DIR/retag.rs:33:19: 33:20
93         Retag(_9);                       // scope 2 at $DIR/retag.rs:33:19: 33:20
-         _8 = &mut (*_9);                 // scope 2 at $DIR/retag.rs:33:19: 33:20
-         Retag(_8);                       // scope 2 at $DIR/retag.rs:33:19: 33:20
+         _8 = &mut (*_9);                 // scope 2 at $DIR/retag.rs:33:17: 33:22
+         Retag(_8);                       // scope 2 at $DIR/retag.rs:33:17: 33:22
96         StorageDead(_9);                 // scope 2 at $DIR/retag.rs:33:22: 33:23
97         StorageLive(_10);                // scope 3 at $DIR/retag.rs:34:13: 34:14
98         _10 = move _8;                   // scope 3 at $DIR/retag.rs:34:17: 34:18

140         StorageDead(_16);                // scope 6 at $DIR/retag.rs:44:18: 44:19
141         StorageDead(_18);                // scope 6 at $DIR/retag.rs:44:19: 44:20
142         StorageLive(_19);                // scope 7 at $DIR/retag.rs:47:5: 47:24
-         StorageLive(_20);                // scope 7 at $DIR/retag.rs:47:5: 47:12
+         StorageLive(_20);                // scope 7 at $DIR/retag.rs:47:5: 47:24
144         StorageLive(_21);                // scope 7 at $DIR/retag.rs:47:5: 47:12
145         _21 = Test(const 0_i32);         // scope 7 at $DIR/retag.rs:47:5: 47:12
-         _20 = &_21;                      // scope 7 at $DIR/retag.rs:47:5: 47:12
-         Retag(_20);                      // scope 7 at $DIR/retag.rs:47:5: 47:12
+         _20 = &_21;                      // scope 7 at $DIR/retag.rs:47:5: 47:24
+         Retag(_20);                      // scope 7 at $DIR/retag.rs:47:5: 47:24
148         StorageLive(_22);                // scope 7 at $DIR/retag.rs:47:21: 47:23
149         StorageLive(_23);                // scope 7 at $DIR/retag.rs:47:21: 47:23
150         _28 = const main::promoted[0];   // scope 7 at $DIR/retag.rs:47:21: 47:23

thread '[mir-opt] mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
31                                            // mir::Constant
31                                            // mir::Constant
32                                            // + span: $DIR/uninhabited_enum_branching.rs:23:21: 23:24
33                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
-           _1 = &(*_5);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
---
test result: FAILED. 141 passed; 20 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:51
