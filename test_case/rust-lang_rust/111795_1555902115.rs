plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
................................................................................i.......  88/255
.....................FF.F..........F....F................i...........iii................ 176/255
.F................iii.iiiii.........ii........i.....................F..........
failures:

---- [mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs' panicked at 'the mir dump file for cycle.cycle.DeadStoreElimination.before.mir does not exist (requested in /checkout/tests/mir-opt/dead-store-elimination/cycle.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/dead-store-elimination/place_mention.rs stdout ----
---- [mir-opt] tests/mir-opt/dead-store-elimination/place_mention.rs stdout ----
thread '[mir-opt] tests/mir-opt/dead-store-elimination/place_mention.rs' panicked at 'the mir dump file for place_mention.main.DeadStoreElimination.before.mir does not exist (requested in /checkout/tests/mir-opt/dead-store-elimination/place_mention.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
---- [mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
thread '[mir-opt] tests/mir-opt/dead-store-elimination/provenance_soundness.rs' panicked at 'the mir dump file for provenance_soundness.pointer_to_int.DeadStoreElimination.before.mir does not exist (requested in /checkout/tests/mir-opt/dead-store-elimination/provenance_soundness.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs stdout ----
7     let mut _3: usize;                   // in scope 0 at $DIR/dead_stores_better.rs:+3:9: +3:10
8     let mut _4: usize;                   // in scope 0 at $DIR/dead_stores_better.rs:+4:8: +4:9
9     scope 1 {
-         debug b => _1;                   // in scope 1 at $DIR/dead_stores_better.rs:+1:9: +1:10
Build completed unsuccessfully in 0:13:00
+         debug b => _3;                   // in scope 1 at $DIR/dead_stores_better.rs:+1:9: +1:10
12 
13     bb0: {


14         nop;                             // scope 0 at $DIR/dead_stores_better.rs:+1:9: +1:10
-         nop;                             // scope 0 at $DIR/dead_stores_better.rs:+1:13: +1:14
+         _3 = _1;                         // scope 0 at $DIR/dead_stores_better.rs:+1:13: +1:14
+         _1 = const 5_usize;              // scope 1 at $DIR/dead_stores_better.rs:+2:5: +2:10
16         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+3:9: +3:10
17         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+3:9: +3:10
-         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+3:5: +3:10
+         _1 = move _3;                    // scope 1 at $DIR/dead_stores_better.rs:+3:5: +3:10
19         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+3:9: +3:10
20         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+4:8: +4:9
21         nop;                             // scope 1 at $DIR/dead_stores_better.rs:+4:8: +4:9

thread '[mir-opt] tests/mir-opt/dest-prop/dead_stores_better.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dest-prop/dead_stores_better.f.DestinationPropagation.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/dest-prop/union.rs stdout ----
25       }
26   
27       bb1: {
27       bb1: {
+           _1 = Un { us: move _2 };         // scope 0 at $DIR/union.rs:+5:14: +5:30
28           StorageDead(_2);                 // scope 0 at $DIR/union.rs:+5:29: +5:30
29           StorageLive(_3);                 // scope 1 at $DIR/union.rs:+7:10: +7:26
+           _3 = (_1.0: u32);                // scope 2 at $DIR/union.rs:+7:19: +7:24
30           StorageDead(_3);                 // scope 1 at $DIR/union.rs:+7:26: +7:27
31           StorageDead(_1);                 // scope 0 at $DIR/union.rs:+8:1: +8:2
32           return;                          // scope 0 at $DIR/union.rs:+8:2: +8:2

thread '[mir-opt] tests/mir-opt/dest-prop/union.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dest-prop/union.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
33           StorageLive(_2);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:65
34           StorageLive(_3);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:58
35           StorageLive(_4);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:17
+           _4 = const 0_u32;                // scope 1 at $DIR/issue_101973.rs:7:19: 7:23
36           StorageLive(_14);                // scope 2 at $DIR/issue_101973.rs:8:12: 8:27
37           StorageLive(_15);                // scope 2 at $DIR/issue_101973.rs:8:12: 8:20
38           _15 = Shr(_1, const 0_i32);      // scope 2 at $DIR/issue_101973.rs:8:12: 8:20

thread '[mir-opt] tests/mir-opt/issue_101973.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_101973.inner.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
3   
3   
4   fn variant_a::{closure#0}(_1: &mut [closure@$DIR/slice_filter.rs:8:25: 8:39], _2: &&(usize, usize, usize, usize)) -> bool {
5       let mut _0: bool;                    // return place in scope 0 at $DIR/slice_filter.rs:+0:40: +0:40
-       let mut _3: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:40: +0:56
-       let mut _4: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:40: +0:46
-       let mut _5: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:50: +0:56
-       let mut _6: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:60: +0:76
-       let mut _7: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:60: +0:66
-       let mut _8: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:70: +0:76
-       let mut _9: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
-       let mut _10: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
-       let mut _11: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
-       let mut _12: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let _3: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:27: +0:28
+       let _4: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:30: +0:31
+       let _5: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:33: +0:34
+       let _6: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:36: +0:37
+       let mut _7: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:40: +0:56
+       let mut _8: bool;                    // in scope 0 at $DIR/slice_filter.rs:+0:40: +0:46
+       let mut _9: &&usize;                 // in scope 0 at $DIR/slice_filter.rs:+0:40: +0:41
+       let mut _10: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:45: +0:46
+       let _11: &usize;                     // in scope 0 at $DIR/slice_filter.rs:+0:45: +0:46
+       let mut _12: bool;                   // in scope 0 at $DIR/slice_filter.rs:+0:50: +0:56
+       let mut _13: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:50: +0:51
+       let mut _14: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:55: +0:56
+       let _15: &usize;                     // in scope 0 at $DIR/slice_filter.rs:+0:55: +0:56
+       let mut _16: bool;                   // in scope 0 at $DIR/slice_filter.rs:+0:60: +0:76
+       let mut _17: bool;                   // in scope 0 at $DIR/slice_filter.rs:+0:60: +0:66
+       let mut _18: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:60: +0:61
+       let mut _19: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:65: +0:66
+       let _20: &usize;                     // in scope 0 at $DIR/slice_filter.rs:+0:65: +0:66
+       let mut _21: bool;                   // in scope 0 at $DIR/slice_filter.rs:+0:70: +0:76
+       let mut _22: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:70: +0:71
+       let mut _23: &&usize;                // in scope 0 at $DIR/slice_filter.rs:+0:75: +0:76
+       let _24: &usize;                     // in scope 0 at $DIR/slice_filter.rs:+0:75: +0:76
+       let mut _25: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let mut _26: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let mut _27: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let mut _28: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let mut _31: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _32: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _37: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _38: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _43: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _44: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _49: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _50: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
16       scope 1 {
-           debug a => &((*_9).0: usize);    // in scope 1 at $DIR/slice_filter.rs:+0:27: +0:28
-           debug b => &((*_10).1: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:30: +0:31
-           debug c => &((*_11).2: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:33: +0:34
-           debug d => &((*_12).3: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:36: +0:37
+           debug a => &((*_25).0: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:27: +0:28
+           debug b => &((*_26).1: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:30: +0:31
+           debug c => &((*_27).2: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:33: +0:34
+           debug d => &((*_28).3: usize);   // in scope 1 at $DIR/slice_filter.rs:+0:36: +0:37
21           scope 2 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:40: 8:46
-               debug self => &&((*_9).0: usize); // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               debug other => &&((*_11).2: usize); // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug self => &&((*_25).0: usize); // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug other => &&((*_27).2: usize); // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _29: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _30: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
24               scope 3 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug self => &((*_9).0: usize); // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug other => &((*_11).2: usize); // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _13: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _14: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug self => &((*_25).0: usize); // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug other => &((*_27).2: usize); // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _33: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _34: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
30           }
30           }
31           scope 4 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:60: 8:66

-               debug self => &&((*_11).2: usize); // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               debug other => &&((*_9).0: usize); // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug self => &&((*_27).2: usize); // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug other => &&((*_25).0: usize); // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _35: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _36: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
34               scope 5 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug self => &((*_11).2: usize); // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug other => &((*_9).0: usize); // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _15: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _16: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug self => &((*_27).2: usize); // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug other => &((*_25).0: usize); // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _39: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _40: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
40           }
40           }
41           scope 6 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:50: 8:56

-               debug self => &&((*_12).3: usize); // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               debug other => &&((*_10).1: usize); // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug self => &&((*_28).3: usize); // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug other => &&((*_26).1: usize); // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _41: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _42: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
44               scope 7 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug self => &((*_12).3: usize); // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug other => &((*_10).1: usize); // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _17: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _18: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug self => &((*_28).3: usize); // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug other => &((*_26).1: usize); // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _45: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _46: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
50           }
50           }
51           scope 8 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:70: 8:76

-               debug self => &&((*_10).1: usize); // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               debug other => &&((*_12).3: usize); // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug self => &&((*_26).1: usize); // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               debug other => &&((*_28).3: usize); // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _47: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _48: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
54               scope 9 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug self => &((*_10).1: usize); // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   debug other => &((*_12).3: usize); // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _19: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-                   let mut _20: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug self => &((*_26).1: usize); // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   debug other => &((*_28).3: usize); // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _51: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+                   let mut _52: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
60           }
61       }

62   
62   
63       bb0: {
-           _9 = deref_copy (*_2);           // scope 0 at $DIR/slice_filter.rs:+0:27: +0:28
-           _10 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:30: +0:31
-           _11 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:33: +0:34
-           _12 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:36: +0:37
- -         StorageLive(_3);                 // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
+           _25 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:27: +0:28
+           _26 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:30: +0:31
+           _27 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:33: +0:34
+           _28 = deref_copy (*_2);          // scope 0 at $DIR/slice_filter.rs:+0:36: +0:37
+ -         StorageLive(_7);                 // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
69 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
-           StorageLive(_4);                 // scope 1 at $DIR/slice_filter.rs:+0:40: +0:46
-           StorageLive(_13);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _13 = ((*_9).0: usize);          // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageLive(_14);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _14 = ((*_11).2: usize);         // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _4 = Le(move _13, move _14);     // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_13);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           switchInt(move _4) -> [0: bb4, otherwise: bb5]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
+           StorageLive(_8);                 // scope 1 at $DIR/slice_filter.rs:+0:40: +0:46
+           StorageLive(_33);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _33 = ((*_25).0: usize);         // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageLive(_34);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _34 = ((*_27).2: usize);         // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _8 = Le(move _33, move _34);     // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_34);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_33);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           switchInt(move _8) -> [0: bb4, otherwise: bb5]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
80   
81       bb1: {

84       }
84       }
85   
86       bb2: {
- -         StorageLive(_6);                 // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
+ -         StorageLive(_16);                // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
88 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
-           StorageLive(_7);                 // scope 1 at $DIR/slice_filter.rs:+0:60: +0:66
-           StorageLive(_15);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _15 = ((*_11).2: usize);         // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageLive(_16);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _16 = ((*_9).0: usize);          // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _7 = Le(move _15, move _16);     // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_16);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_15);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           switchInt(move _7) -> [0: bb6, otherwise: bb7]; // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
+           StorageLive(_17);                // scope 1 at $DIR/slice_filter.rs:+0:60: +0:66
+           StorageLive(_39);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _39 = ((*_27).2: usize);         // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageLive(_40);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _40 = ((*_25).0: usize);         // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _17 = Le(move _39, move _40);    // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_40);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_39);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           switchInt(move _17) -> [0: bb6, otherwise: bb7]; // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
99   
100       bb3: {


- -         StorageDead(_6);                 // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
- -         StorageDead(_3);                 // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
+ -         StorageDead(_16);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
+ -         StorageDead(_7);                 // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
103 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
104 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
105           return;                          // scope 0 at $DIR/slice_filter.rs:+0:76: +0:76
106       }
107   
108       bb4: {
108       bb4: {
- -         StorageDead(_5);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+ -         _7 = const false;                // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
+ -         StorageDead(_12);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+ +         _12 = const false;               // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
110 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
-           StorageDead(_4);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+           StorageDead(_8);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
112           goto -> bb2;                     // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
114   

115       bb5: {
115       bb5: {
- -         StorageLive(_5);                 // scope 1 at $DIR/slice_filter.rs:+0:50: +0:56
+ -         StorageLive(_12);                // scope 1 at $DIR/slice_filter.rs:+0:50: +0:56
117 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:50: +0:56
-           StorageLive(_17);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _17 = ((*_12).3: usize);         // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageLive(_18);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _18 = ((*_10).1: usize);         // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _5 = Le(move _17, move _18);     // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_18);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_17);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _3 = move _5;                    // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
- -         StorageDead(_5);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+           StorageLive(_45);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _45 = ((*_28).3: usize);         // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageLive(_46);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _46 = ((*_26).1: usize);         // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _12 = Le(move _45, move _46);    // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_46);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_45);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _7 = move _12;                   // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
+ -         StorageDead(_12);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
127 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:40: +0:56
128 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
-           StorageDead(_4);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
- -         switchInt(move _3) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
- +         switchInt(move _5) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
+           StorageDead(_8);                 // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+ -         switchInt(move _7) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
+ +         switchInt(move _12) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
133   
134       bb6: {


- -         _6 = const false;                // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
+ -         _16 = const false;               // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
136 +         _0 = const false;                // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
137           goto -> bb8;                     // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76

139   
140       bb7: {
140       bb7: {
- -         StorageLive(_8);                 // scope 1 at $DIR/slice_filter.rs:+0:70: +0:76
+ -         StorageLive(_21);                // scope 1 at $DIR/slice_filter.rs:+0:70: +0:76
142 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:70: +0:76
-           StorageLive(_19);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _19 = ((*_10).1: usize);         // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageLive(_20);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _20 = ((*_12).3: usize);         // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _8 = Le(move _19, move _20);     // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +         _0 = Le(move _19, move _20);     // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_20);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           StorageDead(_19);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _6 = move _8;                    // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
+           StorageLive(_51);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _51 = ((*_26).1: usize);         // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageLive(_52);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _52 = ((*_28).3: usize);         // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _21 = Le(move _51, move _52);    // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +         _0 = Le(move _51, move _52);     // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_52);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           StorageDead(_51);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _16 = move _21;                  // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
152 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76
153           goto -> bb8;                     // scope 1 at $DIR/slice_filter.rs:+0:60: +0:76

155   
156       bb8: {
156       bb8: {
- -         StorageDead(_8);                 // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
+ -         StorageDead(_21);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
158 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
-           StorageDead(_7);                 // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
- -         _0 = move _6;                    // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
+           StorageDead(_17);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
+ -         _0 = move _16;                   // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
161 +         nop;                             // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76
162           goto -> bb3;                     // scope 1 at $DIR/slice_filter.rs:+0:40: +0:76


thread '[mir-opt] tests/mir-opt/slice_filter.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/slice_filter.variant_a-{closure#0}.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/dead-store-elimination/cycle.rs
    [mir-opt] tests/mir-opt/dead-store-elimination/place_mention.rs
