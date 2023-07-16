plain
test [mir-opt] tests/mir-opt/unusual_item_types.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs stdout ----
29                     scope 6 (inlined core::num::<impl u32>::unchecked_shl) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
30                         debug self => _1; // in scope 6 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
31                         debug rhs => _9; // in scope 6 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                         let mut _11: (u32,); // in scope 6 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                         let mut _12: u32; // in scope 6 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                         let mut _11: u32; // in scope 6 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                         let mut _12: (u32,); // in scope 6 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                         let mut _13: u32; // in scope 6 at $SRC_DIR/core/src/num/mod.rs:LL:COL
34                         scope 7 {
35                             scope 8 (inlined core::num::<impl u32>::unchecked_shl::conv) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                                 debug x => _12; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                                 let mut _13: std::option::Option<u32>; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                                 let mut _14: std::result::Result<u32, std::convert::Infallible>; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                                 debug x => _13; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                                 let mut _14: std::option::Option<u32>; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                                 let mut _15: std::result::Result<u32, std::convert::Infallible>; // in scope 8 at $SRC_DIR/core/src/num/mod.rs:LL:COL
39                                 scope 9 {
40                                     scope 10 (inlined <u32 as TryInto<u32>>::try_into) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                                         debug self => _12; // in scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+                                         debug self => _13; // in scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
42                                         scope 11 (inlined <u32 as TryFrom<u32>>::try_from) { // at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-                                             debug value => _12; // in scope 11 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+                                             debug value => _13; // in scope 11 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
44                                             scope 21 (inlined <u32 as Into<u32>>::into) { // at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-                                                 debug self => _12; // in scope 21 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+                                                 debug self => _13; // in scope 21 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
46                                                 scope 22 (inlined <u32 as From<u32>>::from) { // at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-                                                     debug t => _12; // in scope 22 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+                                                     debug t => _13; // in scope 22 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
49                                             }
50                                         }

51                                     }
51                                     }
52                                     scope 12 (inlined Result::<u32, Infallible>::ok) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
-                                         debug self => _14; // in scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
-                                         let _15: u32; // in scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
+                                         debug self => _15; // in scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
+                                         let _16: u32; // in scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
55                                         scope 13 {
-                                             debug x => _15; // in scope 13 at $SRC_DIR/core/src/result.rs:LL:COL
+                                             debug x => _16; // in scope 13 at $SRC_DIR/core/src/result.rs:LL:COL
58                                     }
58                                     }
59                                     scope 14 (inlined #[track_caller] Option::<u32>::unwrap_unchecked) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL

-                                         debug self => _13; // in scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
-                                         let mut _16: &std::option::Option<u32>; // in scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
-                                         let _17: u32; // in scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
+                                         debug self => _14; // in scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
+                                         let mut _17: &std::option::Option<u32>; // in scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
63                                         scope 15 {
-                                             debug val => _17; // in scope 15 at $SRC_DIR/core/src/option.rs:LL:COL
+                                             debug val => _11; // in scope 15 at $SRC_DIR/core/src/option.rs:LL:COL
66                                         scope 16 {
66                                         scope 16 {
67                                             scope 18 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL
72                                             }
73                                         }
73                                         }
74                                         scope 17 (inlined Option::<u32>::is_some) { // at $SRC_DIR/core/src/option.rs:LL:COL
-                                             debug self => _16; // in scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
+                                             debug self => _17; // in scope 17 at $SRC_DIR/core/src/option.rs:LL:COL
77                                     }
78                                 }


89         StorageLive(_4);                 // scope 0 at $DIR/checked_ops.rs:+1:7: +1:23
90         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
91         StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageLive(_12);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         StorageLive(_13);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
93         StorageLive(_9);                 // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
94         StorageLive(_10);                // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
95         _10 = const 31_u32;              // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL

96         _9 = BitAnd(_2, move _10);       // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
97         StorageDead(_10);                // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageLive(_17);                // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
99         StorageLive(_11);                // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _11 = (_9,);                     // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _12 = move (_11.0: u32);         // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         StorageLive(_13);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         StorageLive(_12);                // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         _12 = (_9,);                     // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         _13 = move (_12.0: u32);         // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
103         StorageLive(_14);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _14 = Result::<u32, Infallible>::Ok(_12); // scope 11 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
105         StorageLive(_15);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _15 = move ((_14 as Ok).0: u32); // scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
-         _13 = Option::<u32>::Some(move _15); // scope 13 at $SRC_DIR/core/src/result.rs:LL:COL
-         StorageDead(_15);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         StorageDead(_14);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         _15 = Result::<u32, Infallible>::Ok(_13); // scope 11 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
110         StorageLive(_16);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _17 = move ((_13 as Some).0: u32); // scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
+         _16 = move ((_15 as Ok).0: u32); // scope 12 at $SRC_DIR/core/src/result.rs:LL:COL
+         _14 = Option::<u32>::Some(move _16); // scope 13 at $SRC_DIR/core/src/result.rs:LL:COL
112         StorageDead(_16);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         StorageDead(_13);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         StorageDead(_11);                // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
-         _7 = unchecked_shl::<u32>(_1, move _17) -> [return: bb5, unwind unreachable]; // scope 7 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         StorageDead(_15);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         StorageLive(_17);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         _11 = move ((_14 as Some).0: u32); // scope 14 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageDead(_17);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         StorageDead(_14);                // scope 9 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         StorageDead(_12);                // scope 7 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+         _7 = unchecked_shl::<u32>(_1, move _11) -> [return: bb5, unwind unreachable]; // scope 7 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
116                                          // mir::Constant
117                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
118                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u32, u32) -> u32 {unchecked_shl::<u32>}, val: Value(<ZST>) }
140     }
141 
142     bb5: {
142     bb5: {
-         StorageDead(_17);                // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         StorageDead(_11);                // scope 7 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
144         StorageDead(_9);                 // scope 5 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageDead(_12);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         StorageDead(_13);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
146         StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
147         _8 = Ge(_2, const _);            // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
148         _5 = (move _7, move _8);         // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL

thread '[mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/checked_ops.checked_shl.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21


failures:
    [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs
