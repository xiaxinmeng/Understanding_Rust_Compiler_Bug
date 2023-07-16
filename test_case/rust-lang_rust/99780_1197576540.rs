plain
.....
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
5 | 1: user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_99325[8f58]::function_with_bytes), UserSubsts { substs: [Const { ty: &'static [u8; 4], kind: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:8 ~ issue_99325[8f58]::main::{constant#1}), const_param_did: Some(DefId(0:4 ~ issue_99325[8f58]::function_with_bytes::BYTES)) }, substs: [], promoted: None }) }], user_self_ty: None }) }, span: $DIR/issue-99325.rs:11:16: 11:68, inferred_ty: fn() -> &'static [u8] {function_with_bytes::<&*b"AAAA">}
7 fn main() -> () {
7 fn main() -> () {
-     let mut _0: ();                      // return place in scope 0 at $DIR/issue-99325.rs:9:15: 9:15
+     let mut _0: ();                      // return place in scope 0 at $DIR/issue-99325.rs:0:15: 0:15
9     let _1: ();                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
10     let mut _2: (&&[u8], &&[u8; 4]);     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
11     let mut _3: &&[u8];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

-     let _4: &[u8];                       // in scope 0 at $DIR/issue-99325.rs:10:16: 10:48
+     let _4: &[u8];                       // in scope 0 at $DIR/issue-99325.rs:1:16: 1:48
13     let mut _5: &&[u8; 4];               // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _6: &[u8; 4];                    // in scope 0 at $DIR/issue-99325.rs:10:50: 10:75
-     let _7: [u8; 4];                     // in scope 0 at $DIR/issue-99325.rs:10:51: 10:75
+     let _6: &[u8; 4];                    // in scope 0 at $DIR/issue-99325.rs:1:50: 1:75
+     let _7: [u8; 4];                     // in scope 0 at $DIR/issue-99325.rs:1:51: 1:75
16     let _8: &&[u8];                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
17     let _9: &&[u8; 4];                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
18     let mut _10: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

30     let _23: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
31     let mut _24: (&&[u8], &&[u8; 4]);    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
32     let mut _25: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _26: &[u8];                      // in scope 0 at $DIR/issue-99325.rs:11:16: 11:70
+     let _26: &[u8];                      // in scope 0 at $DIR/issue-99325.rs:2:16: 2:70
34     let mut _27: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _28: &[u8; 4];                   // in scope 0 at $DIR/issue-99325.rs:11:72: 11:79
+     let _28: &[u8; 4];                   // in scope 0 at $DIR/issue-99325.rs:2:72: 2:79
36     let _29: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
37     let _30: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
38     let mut _31: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

68         StorageLive(_1);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
69         StorageLive(_2);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
70         StorageLive(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_4);                 // scope 0 at $DIR/issue-99325.rs:10:16: 10:48
-         _4 = function_with_bytes::<&*b"AAAA">() -> [return: bb1, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:10:16: 10:48
+         StorageLive(_4);                 // scope 0 at $DIR/issue-99325.rs:1:16: 1:48
+         _4 = function_with_bytes::<&*b"AAAA">() -> [return: bb1, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:1:16: 1:48
73                                          // mir::Constant
74                                          // + span: $DIR/issue-99325.rs:10:16: 10:46
75                                          // + user_ty: UserType(0)
79     bb1: {
79     bb1: {
80         _3 = &_4;                        // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
81         StorageLive(_5);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_6);                 // scope 0 at $DIR/issue-99325.rs:10:50: 10:75
-         StorageLive(_7);                 // scope 0 at $DIR/issue-99325.rs:10:51: 10:75
-         _7 = [const 65_u8, const 65_u8, const 65_u8, const 65_u8]; // scope 0 at $DIR/issue-99325.rs:10:51: 10:75
-         _6 = &_7;                        // scope 0 at $DIR/issue-99325.rs:10:50: 10:75
+         StorageLive(_6);                 // scope 0 at $DIR/issue-99325.rs:1:50: 1:75
+         StorageLive(_7);                 // scope 0 at $DIR/issue-99325.rs:1:51: 1:75
+         _7 = [const 65_u8, const 65_u8, const 65_u8, const 65_u8]; // scope 0 at $DIR/issue-99325.rs:1:51: 1:75
+         _6 = &_7;                        // scope 0 at $DIR/issue-99325.rs:1:50: 1:75
86         _5 = &_6;                        // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
87         _2 = (move _3, move _5);         // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
88         StorageDead(_5);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

176         StorageLive(_23);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
177         StorageLive(_24);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
178         StorageLive(_25);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_26);                // scope 0 at $DIR/issue-99325.rs:11:16: 11:70
-         _26 = function_with_bytes::<&*b"AAAA">() -> [return: bb10, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:11:16: 11:70
+         StorageLive(_26);                // scope 0 at $DIR/issue-99325.rs:2:16: 2:70
+         _26 = function_with_bytes::<&*b"AAAA">() -> [return: bb10, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:2:16: 2:70
181                                          // mir::Constant
182                                          // + span: $DIR/issue-99325.rs:11:16: 11:68
183                                          // + user_ty: UserType(1)
187     bb10: {
187     bb10: {
188         _25 = &_26;                      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
189         StorageLive(_27);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_28);                // scope 0 at $DIR/issue-99325.rs:11:72: 11:79
-         _28 = const b"AAAA";             // scope 0 at $DIR/issue-99325.rs:11:72: 11:79
+         StorageLive(_28);                // scope 0 at $DIR/issue-99325.rs:2:72: 2:79
+         _28 = const b"AAAA";             // scope 0 at $DIR/issue-99325.rs:2:72: 2:79
192                                          // mir::Constant
193                                          // + span: $DIR/issue-99325.rs:11:72: 11:79
194                                          // + literal: Const { ty: &[u8; 4], val: Value(Scalar(alloc4)) }

281         StorageDead(_26);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
282         StorageDead(_24);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
283         StorageDead(_23);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _0 = const ();                   // scope 0 at $DIR/issue-99325.rs:9:15: 12:2
-         return;                          // scope 0 at $DIR/issue-99325.rs:12:2: 12:2
+         _0 = const ();                   // scope 0 at $DIR/issue-99325.rs:0:15: 3:2
+         return;                          // scope 0 at $DIR/issue-99325.rs:3:2: 3:2
287 
287 
288     bb19 (cleanup): {

-         resume;                          // scope 0 at $DIR/issue-99325.rs:9:1: 12:2
+         resume;                          // scope 0 at $DIR/issue-99325.rs:0:1: 3:2
291 }
292 


thread '[mir-opt] src/test/mir-opt/issue-99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_99325.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/issue-99325.rs
