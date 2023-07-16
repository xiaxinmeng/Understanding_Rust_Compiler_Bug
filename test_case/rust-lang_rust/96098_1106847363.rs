plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 171 tests
......................................i................................................i 88/171
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......F........F..................................i................................

---- [mir-opt] src/test/mir-opt/inline/inline-diverging.rs stdout ----
4   fn h() -> () {
4   fn h() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-diverging.rs:21:12: 21:12
6       let _1: (!, !);                      // in scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- +     let mut _2: (!, !);                  // in scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- +     let mut _3: fn() -> ! {sleep};       // in scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- +     let mut _10: ();                     // in scope 0 at $DIR/inline-diverging.rs:27:13: 27:16
- +     let mut _11: ();                     // in scope 0 at $DIR/inline-diverging.rs:28:13: 28:16
+ +     let mut _2: fn() -> ! {sleep};       // in scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
+ +     let mut _9: ();                      // in scope 0 at $DIR/inline-diverging.rs:27:13: 27:16
+ +     let mut _10: ();                     // in scope 0 at $DIR/inline-diverging.rs:28:13: 28:16
11 +     scope 1 (inlined call_twice::<!, fn() -> ! {sleep}>) { // at $DIR/inline-diverging.rs:22:5: 22:22
- +         debug f => _3;                   // in scope 1 at $DIR/inline-diverging.rs:26:36: 26:37
- +         let _4: !;                       // in scope 1 at $DIR/inline-diverging.rs:27:9: 27:10
- +         let mut _5: &fn() -> ! {sleep};  // in scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
- +         let mut _7: &fn() -> ! {sleep};  // in scope 1 at $DIR/inline-diverging.rs:28:13: 28:14
- +         let mut _8: !;                   // in scope 1 at $DIR/inline-diverging.rs:29:6: 29:7
- +         let mut _9: !;                   // in scope 1 at $DIR/inline-diverging.rs:29:9: 29:10
+ +         debug f => _2;                   // in scope 1 at $DIR/inline-diverging.rs:26:36: 26:37
+ +         let _3: !;                       // in scope 1 at $DIR/inline-diverging.rs:27:9: 27:10
+ +         let mut _4: &fn() -> ! {sleep};  // in scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
+ +         let mut _6: &fn() -> ! {sleep};  // in scope 1 at $DIR/inline-diverging.rs:28:13: 28:14
+ +         let mut _7: !;                   // in scope 1 at $DIR/inline-diverging.rs:29:6: 29:7
+ +         let mut _8: !;                   // in scope 1 at $DIR/inline-diverging.rs:29:9: 29:10
18 +         scope 2 {
- +             debug a => _4;               // in scope 2 at $DIR/inline-diverging.rs:27:9: 27:10
- +             let _6: !;                   // in scope 2 at $DIR/inline-diverging.rs:28:9: 28:10
+ +             debug a => _3;               // in scope 2 at $DIR/inline-diverging.rs:27:9: 27:10
+ +             let _5: !;                   // in scope 2 at $DIR/inline-diverging.rs:28:9: 28:10
21 +             scope 3 {
- +                 debug b => _6;           // in scope 3 at $DIR/inline-diverging.rs:28:9: 28:10
+ +                 debug b => _5;           // in scope 3 at $DIR/inline-diverging.rs:28:9: 28:10
23 +             }
24 +             scope 6 (inlined <fn() -> ! {sleep} as Fn<()>>::call - shim(fn() -> ! {sleep})) { // at $DIR/inline-diverging.rs:28:13: 28:16
25 +                 scope 7 (inlined sleep) { // at $SRC_DIR/core/src/ops/function.rs:LL:COL
34   
35       bb0: {
35       bb0: {
36           StorageLive(_1);                 // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- -         call_twice::<!, fn() -> ! {sleep}>(sleep); // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
+ -         _1 = call_twice::<!, fn() -> ! {sleep}>(sleep); // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
38 +         StorageLive(_2);                 // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- +         StorageLive(_3);                 // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
- +         _3 = sleep;                      // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
+ +         _2 = sleep;                      // scope 0 at $DIR/inline-diverging.rs:22:5: 22:22
41                                            // mir::Constant
42 -                                          // + span: $DIR/inline-diverging.rs:22:5: 22:15
43 -                                          // + literal: Const { ty: fn(fn() -> ! {sleep}) -> (!, !) {call_twice::<!, fn() -> ! {sleep}>}, val: Value(Scalar(<ZST>)) }
44 -                                          // mir::Constant
44 -                                          // mir::Constant
45                                            // + span: $DIR/inline-diverging.rs:22:16: 22:21
46                                            // + literal: Const { ty: fn() -> ! {sleep}, val: Value(Scalar(<ZST>)) }
- +         StorageLive(_4);                 // scope 1 at $DIR/inline-diverging.rs:27:9: 27:10
- +         StorageLive(_5);                 // scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
- +         _5 = &_3;                        // scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
- +         StorageLive(_10);                // scope 1 at $DIR/inline-diverging.rs:27:13: 27:16
- +         _10 = const ();                  // scope 1 at $DIR/inline-diverging.rs:27:13: 27:16
+ +         StorageLive(_3);                 // scope 1 at $DIR/inline-diverging.rs:27:9: 27:10
+ +         StorageLive(_4);                 // scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
+ +         _4 = &_2;                        // scope 1 at $DIR/inline-diverging.rs:27:13: 27:14
+ +         StorageLive(_9);                 // scope 1 at $DIR/inline-diverging.rs:27:13: 27:16
+ +         _9 = const ();                   // scope 1 at $DIR/inline-diverging.rs:27:13: 27:16
52 +         goto -> bb1;                     // scope 5 at $DIR/inline-diverging.rs:39:5: 39:12
54 + 


thread '[mir-opt] src/test/mir-opt/inline/inline-diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_diverging.h.Inline.diff', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] src/test/mir-opt/issue-72181-1.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-72181-1.rs stdout ----
21         StorageLive(_2);                 // scope 0 at $DIR/issue-72181-1.rs:16:9: 16:10
22         StorageLive(_3);                 // scope 2 at $DIR/issue-72181-1.rs:17:41: 17:43
23         _3 = ();                         // scope 2 at $DIR/issue-72181-1.rs:17:41: 17:43
-         transmute::<(), Void>(move _3) -> bb4; // scope 2 at $DIR/issue-72181-1.rs:17:9: 17:44
+         _2 = transmute::<(), Void>(move _3) -> bb4; // scope 2 at $DIR/issue-72181-1.rs:17:9: 17:44
25                                          // mir::Constant
26                                          // + span: $DIR/issue-72181-1.rs:17:9: 17:40
27                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(()) -> Void {transmute::<(), Void>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] src/test/mir-opt/issue-72181-1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181_1.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3406:25

failures:
    [mir-opt] src/test/mir-opt/inline/inline-diverging.rs
    [mir-opt] src/test/mir-opt/issue-72181-1.rs
