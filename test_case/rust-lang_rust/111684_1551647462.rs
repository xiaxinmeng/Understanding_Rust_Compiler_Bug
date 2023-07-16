plain
..................iii.iiiii.........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/offset_of.rs stdout ----
4   fn concrete() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/offset_of.rs:+0:15: +0:15
6       let _1: usize;                       // in scope 0 at $DIR/offset_of.rs:+1:9: +1:10
+       let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+       let mut _4: usize;                   // in scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+       let mut _6: usize;                   // in scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+       let mut _8: usize;                   // in scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
7       scope 1 {
8           debug x => _1;                   // in scope 1 at $DIR/offset_of.rs:+1:9: +1:10
-           let _2: usize;                   // in scope 1 at $DIR/offset_of.rs:+2:9: +2:10
+           let _3: usize;                   // in scope 1 at $DIR/offset_of.rs:+2:9: +2:10
10           scope 2 {
-               debug y => _2;               // in scope 2 at $DIR/offset_of.rs:+2:9: +2:10
-               let _3: usize;               // in scope 2 at $DIR/offset_of.rs:+3:9: +3:11
+               debug y => _3;               // in scope 2 at $DIR/offset_of.rs:+2:9: +2:10
+               let _5: usize;               // in scope 2 at $DIR/offset_of.rs:+3:9: +3:11
13               scope 3 {
-                   debug z0 => _3;          // in scope 3 at $DIR/offset_of.rs:+3:9: +3:11
-                   let _4: usize;           // in scope 3 at $DIR/offset_of.rs:+4:9: +4:11
+                   debug z0 => _5;          // in scope 3 at $DIR/offset_of.rs:+3:9: +3:11
Build completed unsuccessfully in 0:10:58
+                   let _7: usize;           // in scope 3 at $DIR/offset_of.rs:+4:9: +4:11
16                   scope 4 {
-                       debug z1 => _4;      // in scope 4 at $DIR/offset_of.rs:+4:9: +4:11
+                       debug z1 => _7;      // in scope 4 at $DIR/offset_of.rs:+4:9: +4:11
19               }
20           }

22   
22   
23       bb0: {
24           StorageLive(_1);                 // scope 0 at $DIR/offset_of.rs:+1:9: +1:10
- -         _1 = OffsetOf(Alpha, [0]);       // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
- +         _1 = const 4_usize;              // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-           StorageLive(_2);                 // scope 1 at $DIR/offset_of.rs:+2:9: +2:10
- -         _2 = OffsetOf(Alpha, [1]);       // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
- +         _2 = const 0_usize;              // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-           StorageLive(_3);                 // scope 2 at $DIR/offset_of.rs:+3:9: +3:11
- -         _3 = OffsetOf(Alpha, [2, 0]);    // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
- +         _3 = const 2_usize;              // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-           StorageLive(_4);                 // scope 3 at $DIR/offset_of.rs:+4:9: +4:11
- -         _4 = OffsetOf(Alpha, [2, 1]);    // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
- +         _4 = const 3_usize;              // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           StorageLive(_2);                 // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ -         _2 = OffsetOf(Alpha, [0]);       // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ +         _2 = const 4_usize;              // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           _1 = must_use::<usize>(move _2) -> bb1; // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // mir::Constant
+                                            // + span: $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // + literal: Const { ty: fn(usize) -> usize {must_use::<usize>}, val: Value(<ZST>) }
+   
+       bb1: {
+       bb1: {
+           StorageDead(_2);                 // scope 0 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           StorageLive(_3);                 // scope 1 at $DIR/offset_of.rs:+2:9: +2:10
+           StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ -         _4 = OffsetOf(Alpha, [1]);       // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ +         _4 = const 0_usize;              // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           _3 = must_use::<usize>(move _4) -> bb2; // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // mir::Constant
+                                            // + span: $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // + literal: Const { ty: fn(usize) -> usize {must_use::<usize>}, val: Value(<ZST>) }
+   
+       bb2: {
+       bb2: {
+           StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           StorageLive(_5);                 // scope 2 at $DIR/offset_of.rs:+3:9: +3:11
+           StorageLive(_6);                 // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ -         _6 = OffsetOf(Alpha, [2, 0]);    // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ +         _6 = const 2_usize;              // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           _5 = must_use::<usize>(move _6) -> bb3; // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // mir::Constant
+                                            // + span: $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // + literal: Const { ty: fn(usize) -> usize {must_use::<usize>}, val: Value(<ZST>) }
+   
+       bb3: {
+       bb3: {
+           StorageDead(_6);                 // scope 2 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           StorageLive(_7);                 // scope 3 at $DIR/offset_of.rs:+4:9: +4:11
+           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ -         _8 = OffsetOf(Alpha, [2, 1]);    // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ +         _8 = const 3_usize;              // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+           _7 = must_use::<usize>(move _8) -> bb4; // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // mir::Constant
+                                            // + span: $SRC_DIR/core/src/mem/mod.rs:LL:COL
+                                            // + literal: Const { ty: fn(usize) -> usize {must_use::<usize>}, val: Value(<ZST>) }
+   
+       bb4: {
+       bb4: {
+           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
36           _0 = const ();                   // scope 0 at $DIR/offset_of.rs:+0:15: +5:2
-           StorageDead(_4);                 // scope 3 at $DIR/offset_of.rs:+5:1: +5:2
-           StorageDead(_3);                 // scope 2 at $DIR/offset_of.rs:+5:1: +5:2
-           StorageDead(_2);                 // scope 1 at $DIR/offset_of.rs:+5:1: +5:2
+           StorageDead(_7);                 // scope 3 at $DIR/offset_of.rs:+5:1: +5:2
+           StorageDead(_5);                 // scope 2 at $DIR/offset_of.rs:+5:1: +5:2
+           StorageDead(_3);                 // scope 1 at $DIR/offset_of.rs:+5:1: +5:2
40           StorageDead(_1);                 // scope 0 at $DIR/offset_of.rs:+5:1: +5:2
41           return;                          // scope 0 at $DIR/offset_of.rs:+5:2: +5:2


thread '[mir-opt] tests/mir-opt/const_prop/offset_of.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/offset_of.concrete.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/const_prop/offset_of.rs
