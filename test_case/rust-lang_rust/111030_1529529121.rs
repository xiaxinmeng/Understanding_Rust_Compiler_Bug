plain

running 249 tests
.............................................................................i..........  88/249
.....................................................i...........ii.i................... 176/249
.............ii.iiiii..F.....ii........i...............F.................
failures:

---- [mir-opt] tests/mir-opt/issue_99325.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_99325.rs stdout ----
26     let _21: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
27     let mut _22: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
28     let _23: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _24: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _24: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
30     let mut _25: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _26: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _26: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
32     let _27: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33     let mut _28: (&&[u8], &&[u8; 4]);    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
34     let mut _29: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

48     let _46: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
49     let mut _47: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
50     let _48: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _49: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _49: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
52     let mut _50: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _51: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _51: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
54     scope 1 {
55         debug left_val => _8;            // in scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
56         debug right_val => _9;           // in scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

160         _23 = &(*_9);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
161         _22 = &(*_23);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
162         StorageLive(_24);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
Build completed unsuccessfully in 0:11:23
-         _24 = &(*_16);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _24 = Option::<Arguments<'_>>::None; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
164         StorageLive(_25);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _25 = &(*_17);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _25 = &(*_16);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
166         StorageLive(_26);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _26 = Option::<Arguments<'_>>::None; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _26 = &(*_17);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
168         _18 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _19, move _20, move _22, move _24, move _25, move _26) -> bb19; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
169                                          // mir::Constant
170                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL

-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], &'static str, &'static str, Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>, &'static str, &'static str) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
173 
174     bb4: {


289         _48 = &(*_34);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
290         _47 = &(*_48);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
291         StorageLive(_49);                // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _49 = &(*_41);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _49 = Option::<Arguments<'_>>::None; // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
293         StorageLive(_50);                // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _50 = &(*_42);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _50 = &(*_41);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
295         StorageLive(_51);                // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _51 = Option::<Arguments<'_>>::None; // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _51 = &(*_42);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
297         _43 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _44, move _45, move _47, move _49, move _50, move _51) -> bb19; // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
298                                          // mir::Constant
299                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL

-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], &'static str, &'static str, Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>, &'static str, &'static str) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
302 
303     bb13: {


thread '[mir-opt] tests/mir-opt/issue_99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_99325.main.built.after.mir', src/tools/compiletest/src/runtest.rs:3578:21

---- [mir-opt] tests/mir-opt/retag.rs stdout ----
---- [mir-opt] tests/mir-opt/retag.rs stdout ----
29     let _33: &usize;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
30     let mut _34: &usize;                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
31     let _35: &usize;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _36: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _36: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33     let mut _37: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _38: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _38: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
35     scope 1 {
36         debug x => _1;                   // in scope 1 at $DIR/retag.rs:+1:9: +1:14
37         let _2: *mut usize;              // in scope 1 at $DIR/retag.rs:+2:9: +2:10

183         _35 = &(*_21);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
184         _34 = &(*_35);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
185         StorageLive(_36);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _36 = &(*_28);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _36 = Option::<Arguments<'_>>::None; // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         Retag(_36);                      // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
187         StorageLive(_37);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _37 = &(*_29);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _37 = &(*_28);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
189         StorageLive(_38);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _38 = Option::<Arguments<'_>>::None; // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         Retag(_38);                      // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _38 = &(*_29);                   // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
192         _30 = core::panicking::assert_failed::<usize, usize>(move _31, move _32, move _34, move _36, move _37, move _38); // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
193                                          // mir::Constant
194                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL

-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a usize, &'b usize, &'static str, &'static str, Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a usize, &'b usize, Option<Arguments<'c>>, &'static str, &'static str) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
197 
198     bb4: {


thread '[mir-opt] tests/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/retag.array_casts.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3578:21

failures:
    [mir-opt] tests/mir-opt/issue_99325.rs
    [mir-opt] tests/mir-opt/retag.rs
