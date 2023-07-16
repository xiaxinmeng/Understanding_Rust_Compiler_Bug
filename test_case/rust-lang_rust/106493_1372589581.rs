plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
i.......i................................
failures:

---- [mir-opt] src/test/mir-opt/building/breakable_scope_drops.rs stdout ----
1 // MIR for `weird_temporary` after built
2 
3 fn weird_temporary(_1: A, _2: B, _3: ((), (), ()), _4: bool) -> ((), (), ()) {
-     debug a => _1;                       // in scope 0 at $DIR/breakable-scope-drops.rs:+0:20: +0:21
-     debug b => _2;                       // in scope 0 at $DIR/breakable-scope-drops.rs:+0:26: +0:27
-     debug nothing => _3;                 // in scope 0 at $DIR/breakable-scope-drops.rs:+0:32: +0:39
-     debug x => _4;                       // in scope 0 at $DIR/breakable-scope-drops.rs:+0:55: +0:56
-     let mut _0: ((), (), ());            // return place in scope 0 at $DIR/breakable-scope-drops.rs:+0:67: +0:79
-     let _5: ((), (), ());                // in scope 0 at $DIR/breakable-scope-drops.rs:+1:9: +1:14
-     let mut _6: ();                      // in scope 0 at $DIR/breakable-scope-drops.rs:+3:13: +8:14
-     let _7: B;                           // in scope 0 at $DIR/breakable-scope-drops.rs:+4:21: +4:23
-     let mut _8: bool;                    // in scope 0 at $DIR/breakable-scope-drops.rs:+5:20: +5:21
-     let mut _9: !;                       // in scope 0 at $DIR/breakable-scope-drops.rs:+5:22: +7:18
-     let mut _10: ();                     // in scope 0 at $DIR/breakable-scope-drops.rs:+9:13: +11:14
-     let mut _11: A;                      // in scope 0 at $DIR/breakable-scope-drops.rs:+9:19: +9:24
-     let mut _12: ();                     // in scope 0 at $DIR/breakable-scope-drops.rs:+12:13: +12:24
+     debug a => _1;                       // in scope 0 at $DIR/breakable_scope_drops.rs:+0:20: +0:21
+     debug b => _2;                       // in scope 0 at $DIR/breakable_scope_drops.rs:+0:26: +0:27
+     debug nothing => _3;                 // in scope 0 at $DIR/breakable_scope_drops.rs:+0:32: +0:39
+     debug x => _4;                       // in scope 0 at $DIR/breakable_scope_drops.rs:+0:55: +0:56
+     let mut _0: ((), (), ());            // return place in scope 0 at $DIR/breakable_scope_drops.rs:+0:67: +0:79
+     let _5: ((), (), ());                // in scope 0 at $DIR/breakable_scope_drops.rs:+1:9: +1:14
+     let mut _6: ();                      // in scope 0 at $DIR/breakable_scope_drops.rs:+3:13: +8:14
+     let _7: B;                           // in scope 0 at $DIR/breakable_scope_drops.rs:+4:21: +4:23
+     let mut _8: bool;                    // in scope 0 at $DIR/breakable_scope_drops.rs:+5:20: +5:21
+     let mut _9: !;                       // in scope 0 at $DIR/breakable_scope_drops.rs:+5:22: +7:18
+     let mut _10: ();                     // in scope 0 at $DIR/breakable_scope_drops.rs:+9:13: +11:14
+     let mut _11: A;                      // in scope 0 at $DIR/breakable_scope_drops.rs:+9:19: +9:24
+     let mut _12: ();                     // in scope 0 at $DIR/breakable_scope_drops.rs:+12:13: +12:24
17     scope 1 {
-         debug _temp => _5;               // in scope 1 at $DIR/breakable-scope-drops.rs:+1:9: +1:14
+         debug _temp => _5;               // in scope 1 at $DIR/breakable_scope_drops.rs:+1:9: +1:14
20     scope 2 {
20     scope 2 {
-         debug _z => _7;                  // in scope 2 at $DIR/breakable-scope-drops.rs:+4:21: +4:23
+         debug _z => _7;                  // in scope 2 at $DIR/breakable_scope_drops.rs:+4:21: +4:23
23 
24     bb0: {


-         StorageLive(_5);                 // scope 0 at $DIR/breakable-scope-drops.rs:+1:9: +1:14
-         StorageLive(_6);                 // scope 0 at $DIR/breakable-scope-drops.rs:+3:13: +8:14
-         StorageLive(_7);                 // scope 0 at $DIR/breakable-scope-drops.rs:+4:21: +4:23
-         _7 = move _2;                    // scope 0 at $DIR/breakable-scope-drops.rs:+4:26: +4:27
-         FakeRead(ForLet(None), _7);      // scope 0 at $DIR/breakable-scope-drops.rs:+4:21: +4:23
-         StorageLive(_8);                 // scope 2 at $DIR/breakable-scope-drops.rs:+5:20: +5:21
-         _8 = _4;                         // scope 2 at $DIR/breakable-scope-drops.rs:+5:20: +5:21
-         switchInt(move _8) -> [0: bb2, otherwise: bb1]; // scope 2 at $DIR/breakable-scope-drops.rs:+5:20: +5:21
+         StorageLive(_5);                 // scope 0 at $DIR/breakable_scope_drops.rs:+1:9: +1:14
+         StorageLive(_6);                 // scope 0 at $DIR/breakable_scope_drops.rs:+3:13: +8:14
+         StorageLive(_7);                 // scope 0 at $DIR/breakable_scope_drops.rs:+4:21: +4:23
+         _7 = move _2;                    // scope 0 at $DIR/breakable_scope_drops.rs:+4:26: +4:27
+         FakeRead(ForLet(None), _7);      // scope 0 at $DIR/breakable_scope_drops.rs:+4:21: +4:23
+         StorageLive(_8);                 // scope 2 at $DIR/breakable_scope_drops.rs:+5:20: +5:21
+         _8 = _4;                         // scope 2 at $DIR/breakable_scope_drops.rs:+5:20: +5:21
+         switchInt(move _8) -> [0: bb2, otherwise: bb1]; // scope 2 at $DIR/breakable_scope_drops.rs:+5:20: +5:21
34 
35     bb1: {


-         _5 = _3;                         // scope 2 at $DIR/breakable-scope-drops.rs:+6:34: +6:41
-         goto -> bb11;                    // scope 2 at $DIR/breakable-scope-drops.rs:+6:21: +6:41
+         _5 = _3;                         // scope 2 at $DIR/breakable_scope_drops.rs:+6:34: +6:41
+         goto -> bb11;                    // scope 2 at $DIR/breakable_scope_drops.rs:+6:21: +6:41
39 
40     bb2: {


-         goto -> bb5;                     // scope 2 at $DIR/breakable-scope-drops.rs:+5:20: +5:21
+         goto -> bb5;                     // scope 2 at $DIR/breakable_scope_drops.rs:+5:20: +5:21
43 
44     bb3: {


-         unreachable;                     // scope 2 at $DIR/breakable-scope-drops.rs:+5:22: +7:18
+         unreachable;                     // scope 2 at $DIR/breakable_scope_drops.rs:+5:22: +7:18
47 
48     bb4: {


-         goto -> bb6;                     // scope 2 at $DIR/breakable-scope-drops.rs:+5:17: +7:18
+         goto -> bb6;                     // scope 2 at $DIR/breakable_scope_drops.rs:+5:17: +7:18
51 
52     bb5: {


-         _6 = const ();                   // scope 2 at $DIR/breakable-scope-drops.rs:+7:18: +7:18
-         goto -> bb6;                     // scope 2 at $DIR/breakable-scope-drops.rs:+5:17: +7:18
+         _6 = const ();                   // scope 2 at $DIR/breakable_scope_drops.rs:+7:18: +7:18
+         goto -> bb6;                     // scope 2 at $DIR/breakable_scope_drops.rs:+5:17: +7:18
56 
57     bb6: {


-         StorageDead(_8);                 // scope 2 at $DIR/breakable-scope-drops.rs:+7:17: +7:18
-         drop(_7) -> [return: bb7, unwind: bb17]; // scope 0 at $DIR/breakable-scope-drops.rs:+8:13: +8:14
+         StorageDead(_8);                 // scope 2 at $DIR/breakable_scope_drops.rs:+7:17: +7:18
+         drop(_7) -> [return: bb7, unwind: bb17]; // scope 0 at $DIR/breakable_scope_drops.rs:+8:13: +8:14
61 
62     bb7: {


-         StorageDead(_7);                 // scope 0 at $DIR/breakable-scope-drops.rs:+8:13: +8:14
-         StorageLive(_10);                // scope 0 at $DIR/breakable-scope-drops.rs:+9:13: +11:14
-         StorageLive(_11);                // scope 0 at $DIR/breakable-scope-drops.rs:+9:19: +9:24
-         _11 = move _1;                   // scope 0 at $DIR/breakable-scope-drops.rs:+9:21: +9:22
-         FakeRead(ForMatchedPlace(None), _11); // scope 0 at $DIR/breakable-scope-drops.rs:+9:19: +9:24
-         _10 = ();                        // scope 0 at $DIR/breakable-scope-drops.rs:+10:22: +10:24
-         goto -> bb8;                     // scope 0 at $DIR/breakable-scope-drops.rs:+10:22: +10:24
+         StorageDead(_7);                 // scope 0 at $DIR/breakable_scope_drops.rs:+8:13: +8:14
+         StorageLive(_10);                // scope 0 at $DIR/breakable_scope_drops.rs:+9:13: +11:14
+         StorageLive(_11);                // scope 0 at $DIR/breakable_scope_drops.rs:+9:19: +9:24
+         _11 = move _1;                   // scope 0 at $DIR/breakable_scope_drops.rs:+9:21: +9:22
+         FakeRead(ForMatchedPlace(None), _11); // scope 0 at $DIR/breakable_scope_drops.rs:+9:19: +9:24
+         _10 = ();                        // scope 0 at $DIR/breakable_scope_drops.rs:+10:22: +10:24
+         goto -> bb8;                     // scope 0 at $DIR/breakable_scope_drops.rs:+10:22: +10:24
71 
72     bb8: {


-         StorageLive(_12);                // scope 0 at $DIR/breakable-scope-drops.rs:+12:13: +12:24
-         _12 = no_unwind() -> [return: bb9, unwind: bb16]; // scope 0 at $DIR/breakable-scope-drops.rs:+12:13: +12:24
+         StorageLive(_12);                // scope 0 at $DIR/breakable_scope_drops.rs:+12:13: +12:24
+         _12 = no_unwind() -> [return: bb9, unwind: bb16]; // scope 0 at $DIR/breakable_scope_drops.rs:+12:13: +12:24
75                                          // mir::Constant
-                                          // + span: $DIR/breakable-scope-drops.rs:29:13: 29:22
+                                          // + span: $DIR/breakable_scope_drops.rs:29:13: 29:22
77                                          // + literal: Const { ty: fn() {no_unwind}, val: Value(<ZST>) }
79 

80     bb9: {
80     bb9: {
-         _5 = (move _6, move _10, move _12); // scope 0 at $DIR/breakable-scope-drops.rs:+2:9: +13:10
-         StorageDead(_12);                // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
-         drop(_11) -> [return: bb10, unwind: bb17]; // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
+         _5 = (move _6, move _10, move _12); // scope 0 at $DIR/breakable_scope_drops.rs:+2:9: +13:10
+         StorageDead(_12);                // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
+         drop(_11) -> [return: bb10, unwind: bb17]; // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
85 
86     bb10: {


-         StorageDead(_11);                // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
-         StorageDead(_10);                // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
-         StorageDead(_6);                 // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
-         goto -> bb13;                    // scope 0 at $DIR/breakable-scope-drops.rs:+1:17: +14:6
+         StorageDead(_11);                // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
+         StorageDead(_10);                // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
+         StorageDead(_6);                 // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
+         goto -> bb13;                    // scope 0 at $DIR/breakable_scope_drops.rs:+1:17: +14:6
92 
93     bb11: {


-         StorageDead(_8);                 // scope 2 at $DIR/breakable-scope-drops.rs:+7:17: +7:18
-         drop(_7) -> [return: bb12, unwind: bb17]; // scope 0 at $DIR/breakable-scope-drops.rs:+8:13: +8:14
+         StorageDead(_8);                 // scope 2 at $DIR/breakable_scope_drops.rs:+7:17: +7:18
+         drop(_7) -> [return: bb12, unwind: bb17]; // scope 0 at $DIR/breakable_scope_drops.rs:+8:13: +8:14
97 
98     bb12: {


-         StorageDead(_7);                 // scope 0 at $DIR/breakable-scope-drops.rs:+8:13: +8:14
-         StorageDead(_6);                 // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
-         goto -> bb13;                    // scope 0 at $DIR/breakable-scope-drops.rs:+1:17: +14:6
+         StorageDead(_7);                 // scope 0 at $DIR/breakable_scope_drops.rs:+8:13: +8:14
+         StorageDead(_6);                 // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
+         goto -> bb13;                    // scope 0 at $DIR/breakable_scope_drops.rs:+1:17: +14:6
103 
104     bb13: {


-         FakeRead(ForLet(None), _5);      // scope 0 at $DIR/breakable-scope-drops.rs:+1:9: +1:14
-         _0 = _3;                         // scope 1 at $DIR/breakable-scope-drops.rs:+16:5: +16:12
-         StorageDead(_5);                 // scope 0 at $DIR/breakable-scope-drops.rs:+17:1: +17:2
-         drop(_2) -> [return: bb14, unwind: bb18]; // scope 0 at $DIR/breakable-scope-drops.rs:+17:1: +17:2
+         FakeRead(ForLet(None), _5);      // scope 0 at $DIR/breakable_scope_drops.rs:+1:9: +1:14
+         _0 = _3;                         // scope 1 at $DIR/breakable_scope_drops.rs:+16:5: +16:12
+         StorageDead(_5);                 // scope 0 at $DIR/breakable_scope_drops.rs:+17:1: +17:2
+         drop(_2) -> [return: bb14, unwind: bb18]; // scope 0 at $DIR/breakable_scope_drops.rs:+17:1: +17:2
110 
111     bb14: {


-         drop(_1) -> [return: bb15, unwind: bb19]; // scope 0 at $DIR/breakable-scope-drops.rs:+17:1: +17:2
+         drop(_1) -> [return: bb15, unwind: bb19]; // scope 0 at $DIR/breakable_scope_drops.rs:+17:1: +17:2
114 
115     bb15: {


-         return;                          // scope 0 at $DIR/breakable-scope-drops.rs:+17:2: +17:2
+         return;                          // scope 0 at $DIR/breakable_scope_drops.rs:+17:2: +17:2
118 
118 
119     bb16 (cleanup): {

-         drop(_11) -> bb17;               // scope 0 at $DIR/breakable-scope-drops.rs:+13:9: +13:10
+         drop(_11) -> bb17;               // scope 0 at $DIR/breakable_scope_drops.rs:+13:9: +13:10
122 
122 
123     bb17 (cleanup): {

-         drop(_2) -> bb18;                // scope 0 at $DIR/breakable-scope-drops.rs:+17:1: +17:2
+         drop(_2) -> bb18;                // scope 0 at $DIR/breakable_scope_drops.rs:+17:1: +17:2
126 
126 
127     bb18 (cleanup): {

-         drop(_1) -> bb19;                // scope 0 at $DIR/breakable-scope-drops.rs:+17:1: +17:2
+         drop(_1) -> bb19;                // scope 0 at $DIR/breakable_scope_drops.rs:+17:1: +17:2
130 
130 
131     bb19 (cleanup): {

-         resume;                          // scope 0 at $DIR/breakable-scope-drops.rs:+0:1: +17:2
+         resume;                          // scope 0 at $DIR/breakable_scope_drops.rs:+0:1: +17:2
134 }
135 


thread '[mir-opt] src/test/mir-opt/building/breakable_scope_drops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/building/breakable_scope_drops.weird_temporary.built.after.mir', src/tools/compiletest/src/runtest.rs:3465:21


failures:
    [mir-opt] src/test/mir-opt/building/breakable_scope_drops.rs
