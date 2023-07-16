plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
.......................................i................................................ 88/188
..........i.................................F............ii.......i............F........ 176/188
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
27     let mut _20: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
28     let _21: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29     let mut _22: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _23: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _24: (&&[u8], &&[u8; 4]);    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _25: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _26: &[u8];                      // in scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
-     let mut _27: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _28: &[u8; 4];                   // in scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
-     let _29: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _30: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _31: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _32: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _33: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _34: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _35: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _37: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _38: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _39: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _40: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _41: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let _42: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     let mut _43: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _23: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _24: &str;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _25: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _26: (&&[u8], &&[u8; 4]);    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _27: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _28: &[u8];                      // in scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
+     let mut _29: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _30: &[u8; 4];                   // in scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
+     let _31: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _32: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _33: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _34: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _35: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _36: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _37: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _39: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _40: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _41: &&[u8];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _42: &&[u8];                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _43: &&[u8; 4];              // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _44: &&[u8; 4];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _45: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _46: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _47: &str;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
50     scope 1 {
51         debug left_val => _8;            // in scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
52         debug right_val => _9;           // in scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
56         }
57     }
58     scope 3 {
58     scope 3 {
-         debug left_val => _29;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         debug right_val => _30;          // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         let _36: core::panicking::AssertKind; // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         debug left_val => _31;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         debug right_val => _32;          // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         let _38: core::panicking::AssertKind; // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
62         scope 4 {
-             debug kind => _36;           // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+             debug kind => _38;           // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
65     }
66 


129         _20 = &(*_21);                   // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
130         StorageLive(_22);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
131         _22 = Option::<Arguments<'_>>::None; // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _16 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _17, move _18, move _20, move _22) -> bb19; // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_23);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_24);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _24 = const "assertion failed: `function_with_bytes::<b\"AAAA\">() == &[0x41, 0x41, 0x41, 0x41]`"; // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
133                                          // mir::Constant
134                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+         _23 = &(*_24);                   // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _16 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _17, move _18, move _20, move _22, move _23) -> bb19; // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // + literal: Const { ty: for<'a, 'b, 'c, 'd> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>, &'d str) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
137 
138     bb4: {

140     }
140     }
141 
142     bb5: {
+         StorageDead(_23);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
143         StorageDead(_22);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
144         StorageDead(_20);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
145         StorageDead(_18);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

146         StorageDead(_17);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_24);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
147         StorageDead(_21);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
148         StorageDead(_19);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
149         StorageDead(_16);                // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

173         StorageDead(_4);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
174         StorageDead(_2);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
175         StorageDead(_1);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_23);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_24);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
178         StorageLive(_25);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_26);                // scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
-         _26 = function_with_bytes::<&*b"AAAA">() -> [return: bb10, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
+         StorageLive(_26);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_27);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_28);                // scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
+         _28 = function_with_bytes::<&*b"AAAA">() -> [return: bb10, unwind: bb19]; // scope 0 at $DIR/issue-99325.rs:+2:16: +2:70
181                                          // mir::Constant
182                                          // + span: $DIR/issue-99325.rs:11:16: 11:68
183                                          // + user_ty: UserType(1)
185     }
186 
187     bb10: {
187     bb10: {
-         _25 = &_26;                      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_27);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_28);                // scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
-         _28 = const b"AAAA";             // scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
+         _27 = &_28;                      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_29);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_30);                // scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
+         _30 = const b"AAAA";             // scope 0 at $DIR/issue-99325.rs:+2:72: +2:79
192                                          // mir::Constant
193                                          // + span: $DIR/issue-99325.rs:11:72: 11:79
194                                          // + literal: Const { ty: &[u8; 4], val: Value(Scalar(alloc4)) }

-         _27 = &_28;                      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _24 = (move _25, move _27);      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _29 = &_30;                      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _26 = (move _27, move _29);      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_29);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
197         StorageDead(_27);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_25);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         FakeRead(ForMatchedPlace(None), _24); // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_29);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _29 = (_24.0: &&[u8]);           // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_30);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _30 = (_24.1: &&[u8; 4]);        // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_31);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_32);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         FakeRead(ForMatchedPlace(None), _26); // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_31);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _31 = (_26.0: &&[u8]);           // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_32);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _32 = (_26.1: &&[u8; 4]);        // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
206         StorageLive(_33);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _33 = &(*_29);                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
208         StorageLive(_34);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _34 = &(*_30);                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _32 = <&[u8] as PartialEq<&[u8; 4]>>::eq(move _33, move _34) -> [return: bb11, unwind: bb19]; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_35);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _35 = &(*_31);                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_36);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _36 = &(*_32);                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _34 = <&[u8] as PartialEq<&[u8; 4]>>::eq(move _35, move _36) -> [return: bb11, unwind: bb19]; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
211                                          // mir::Constant
212                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
213                                          // + literal: Const { ty: for<'a, 'b> fn(&'a &[u8], &'b &[u8; 4]) -> bool {<&[u8] as PartialEq<&[u8; 4]>>::eq}, val: Value(<ZST>) }
214     }
215 
216     bb11: {
216     bb11: {
+         StorageDead(_36);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_35);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _33 = Not(move _34);             // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
217         StorageDead(_34);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_33);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _31 = Not(move _32);             // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_32);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         switchInt(move _31) -> [false: bb13, otherwise: bb12]; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         switchInt(move _33) -> [false: bb13, otherwise: bb12]; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
223 
224     bb12: {


-         StorageLive(_36);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _36 = core::panicking::AssertKind::Eq; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         FakeRead(ForLet(None), _36);     // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_37);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageLive(_38);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _38 = move _36;                  // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_38);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _38 = core::panicking::AssertKind::Eq; // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         FakeRead(ForLet(None), _38);     // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
231         StorageLive(_39);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
232         StorageLive(_40);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _40 = &(*_29);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _39 = &(*_40);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _40 = move _38;                  // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
235         StorageLive(_41);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
236         StorageLive(_42);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _42 = &(*_30);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _42 = &(*_31);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
238         _41 = &(*_42);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
239         StorageLive(_43);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _43 = Option::<Arguments<'_>>::None; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _37 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _38, move _39, move _41, move _43) -> bb19; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_44);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _44 = &(*_32);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _43 = &(*_44);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_45);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _45 = Option::<Arguments<'_>>::None; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_46);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_47);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _47 = const "assertion failed: `function_with_bytes::<{ &[0x41, 0x41, 0x41, 0x41] }>() == b\"AAAA\"`"; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
242                                          // mir::Constant
243                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+         _46 = &(*_47);                   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _39 = core::panicking::assert_failed::<&[u8], &[u8; 4]>(move _40, move _41, move _43, move _45, move _46) -> bb19; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // + literal: Const { ty: for<'a, 'b, 'c, 'd> fn(core::panicking::AssertKind, &'a &[u8], &'b &[u8; 4], Option<Arguments<'c>>, &'d str) -> ! {core::panicking::assert_failed::<&[u8], &[u8; 4]>}, val: Value(<ZST>) }
246 
247     bb13: {

249     }
249     }
250 
251     bb14: {
+         StorageDead(_46);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_45);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
252         StorageDead(_43);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
253         StorageDead(_41);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_39);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_38);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_42);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
257         StorageDead(_40);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_37);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_36);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_47);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_44);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_42);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_39);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_38);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
260         unreachable;                     // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
262 

265     }
266 
266 
267     bb16: {
-         _23 = const ();                  // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _25 = const ();                  // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
269         goto -> bb17;                    // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
271 

272     bb17: {
272     bb17: {
-         StorageDead(_31);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_30);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_29);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_33);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_32);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_31);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
276         goto -> bb18;                    // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
278 

279     bb18: {
279     bb18: {
+         StorageDead(_30);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
280         StorageDead(_28);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
281         StorageDead(_26);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_24);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_23);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_25);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
284         _0 = const ();                   // scope 0 at $DIR/issue-99325.rs:+0:15: +3:2
285         return;                          // scope 0 at $DIR/issue-99325.rs:+3:2: +3:2


thread '[mir-opt] src/test/mir-opt/issue-99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_99325.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25

---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
30     let mut _32: &usize;                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
31     let _33: &usize;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
32     let mut _34: std::option::Option<std::fmt::Arguments<'_>>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _35: &str;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let _36: &str;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33     scope 1 {
34         debug x => _1;                   // in scope 1 at $DIR/retag.rs:+1:9: +1:14
35         let _2: *mut usize;              // in scope 1 at $DIR/retag.rs:+2:9: +2:10

45                     debug p => _9;       // in scope 5 at $DIR/retag.rs:+6:9: +6:10
46                     let _20: &usize;     // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
47                     let _21: &usize;     // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                     let mut _35: &usize; // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                     let mut _37: &usize; // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
49                     scope 6 {
51                     scope 7 {


121         _14 = &_15;                      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
122         Retag(_14);                      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
123         StorageLive(_18);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _35 = const _;                   // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _37 = const _;                   // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                          // mir::Constant
126                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
127                                          // + literal: Const { ty: &usize, val: Unevaluated(array_casts, [], Some(promoted[0])) }

-         Retag(_35);                      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _18 = &(*_35);                   // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         Retag(_37);                      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _18 = &(*_37);                   // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
130         Retag(_18);                      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
131         Deinit(_13);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
132         (_13.0: &usize) = move _14;      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

177         Deinit(_34);                     // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
178         discriminant(_34) = 0;           // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
179         Retag(_34);                      // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         _28 = core::panicking::assert_failed::<usize, usize>(move _29, move _30, move _32, move _34); // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_35);                // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageLive(_36);                // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _36 = const "assertion failed: `unsafe { *p.add(1) } == 1`"; // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
181                                          // mir::Constant
182                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'a, 'b, 'c> fn(core::panicking::AssertKind, &'a usize, &'b usize, Option<Arguments<'c>>) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+         Retag(_36);                      // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _35 = &(*_36);                   // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         Retag(_35);                      // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         _28 = core::panicking::assert_failed::<usize, usize>(move _29, move _30, move _32, move _34, move _35); // scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                          // + literal: Const { ty: for<'a, 'b, 'c, 'd> fn(core::panicking::AssertKind, &'a usize, &'b usize, Option<Arguments<'c>>, &'d str) -> ! {core::panicking::assert_failed::<usize, usize>}, val: Value(<ZST>) }
185 
186     bb4: {


thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.array_casts.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3515:25

failures:
    [mir-opt] src/test/mir-opt/issue-99325.rs
    [mir-opt] src/test/mir-opt/retag.rs
