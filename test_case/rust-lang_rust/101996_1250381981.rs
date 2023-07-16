plain
 finished in 0.601 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 185 tests
.......................................i..........................................F..... 88/185
.........i.......................F........F...........ii........i.........F............. 176/185
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/funky_arms.rs stdout ----
---- [mir-opt] src/test/mir-opt/funky_arms.rs stdout ----
100           _0 = float_to_exponential_common_exact::<T>(move _11, move _12, move _13, move _14, move _17) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
101                                            // mir::Constant
102                                            // + span: $DIR/funky_arms.rs:26:9: 26:42
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut Formatter<'s>, &'t0 T, Sign, u32, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(<ZST>) }
+                                            // + literal: Const { ty: for<'r, 's, 't> fn(&'r mut Formatter<'s>, &'t T, Sign, u32, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(<ZST>) }
105   
106       bb7: {


125           _0 = float_to_exponential_common_shortest::<T>(move _18, move _19, move _20, move _21) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
126                                            // mir::Constant
127                                            // + span: $DIR/funky_arms.rs:28:9: 28:45
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r mut Formatter<'s>, &'t0 T, Sign, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(<ZST>) }
+                                            // + literal: Const { ty: for<'r, 's, 't> fn(&'r mut Formatter<'s>, &'t T, Sign, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(<ZST>) }
130   
131       bb9: {


thread '[mir-opt] src/test/mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3515:25

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
139           _21 = core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
140                                            // mir::Constant
141                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(<ZST>) }
+                                            // + literal: Const { ty: for<'r, 's, 't> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(<ZST>) }
143                                            // mir::Constant
144                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
145                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
132         _16 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _17, move _18, move _20, move _22) -> bb19; // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
133                                          // mir::Constant
134                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r &[u8], &'s &[u8; 4], Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'r, 's, 't> fn(core::panicking::AssertKind, &'r &[u8], &'s &[u8; 4], Option<Arguments<'t>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
137 
138     bb4: {


241         _37 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _38, move _39, move _41, move _43) -> bb19; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
242                                          // mir::Constant
243                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r &[u8], &'s &[u8; 4], Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'r, 's, 't> fn(core::panicking::AssertKind, &'r &[u8], &'s &[u8; 4], Option<Arguments<'t>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
246 
247     bb13: {


thread '[mir-opt] src/test/mir-opt/issue-99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_99325.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
180         _28 = core::panicking::assert_failed::<usize, usize>(move _29, move _30, move _32, move _34); // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
181                                          // mir::Constant
182                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r usize, &'s usize, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'r, 's, 't> fn(core::panicking::AssertKind, &'r usize, &'s usize, Option<Arguments<'t>>) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
185 
186     bb4: {


thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.array_casts.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3515:25

failures:
    [mir-opt] src/test/mir-opt/funky_arms.rs
    [mir-opt] src/test/mir-opt/issue-73223.rs
