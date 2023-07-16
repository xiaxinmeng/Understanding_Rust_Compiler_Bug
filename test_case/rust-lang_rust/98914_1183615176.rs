plain
.....
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/deref-patterns/string.rs stdout ----
9     let mut _5: isize;                   // in scope 0 at $DIR/string.rs:8:9: 8:18
10     let _6: std::option::Option<std::string::String>; // in scope 0 at $DIR/string.rs:9:9: 9:10
11     let mut _7: bool;                    // in scope 0 at $DIR/string.rs:11:1: 11:2
-     let mut _30: usize;                  // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
13     scope 1 {
14         debug s => _6;                   // in scope 1 at $DIR/string.rs:9:9: 9:10


-     scope 2 (inlined <String as Deref>::deref) { // at $DIR/string.rs:8:14: 8:17
-         debug self => _2;                // in scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         let mut _8: &[u8];               // in scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         let _9: &[u8];                   // in scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         let mut _10: &std::vec::Vec<u8>; // in scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         let _11: &std::vec::Vec<u8>;     // in scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         scope 3 {
-             scope 4 (inlined <Vec<u8> as Deref>::deref) { // at $SRC_DIR/alloc/src/string.rs:LL:COL
-                 debug self => _10;       // in scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 let mut _12: *const u8;  // in scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 let mut _13: &std::vec::Vec<u8>; // in scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 let mut _14: usize;      // in scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 scope 5 {
-                     scope 6 (inlined Vec::<u8>::as_ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         debug self => _13; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let _15: *mut u8; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let mut _16: &alloc::raw_vec::RawVec<u8>; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let _17: ();     // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let mut _18: bool; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let mut _19: bool; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let mut _20: *mut u8; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         let mut _21: *mut u8; // in scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         scope 7 {
-                             debug ptr => _15; // in scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                             scope 8 {
-                                 scope 12 (inlined ptr::mut_ptr::<impl *mut u8>::is_null) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                                     debug self => _20; // in scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                     let mut _25: *mut u8; // in scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                     let mut _26: *mut u8; // in scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                     let mut _27: *mut u8; // in scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                     scope 13 (inlined null_mut::<u8>) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _28: *mut (); // in scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                         let mut _29: (); // in scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                         scope 14 (inlined invalid_mut::<()>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                             debug addr => _30; // in scope 14 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                             scope 15 {
-                                         }
-                                         }
-                                         scope 16 (inlined std::ptr::from_raw_parts_mut::<u8>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                             debug data_address => _28; // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             debug metadata => _29; // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             let mut _31: std::ptr::metadata::PtrRepr<u8>; // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             let mut _32: std::ptr::metadata::PtrComponents<u8>; // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             let mut _33: *const (); // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             let mut _34: *mut (); // in scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                             scope 17 {
-                                         }
-                                     }
-                                     }
-                                     scope 18 (inlined ptr::mut_ptr::<impl *mut u8>::guaranteed_eq) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         debug self => _25; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         debug other => _27; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _35: *const u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _36: *const u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _37: *mut u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _38: *const u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _39: *const u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                         let mut _40: *mut u8; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                 }
-                             }
-                         }
-                         }
-                         scope 9 (inlined alloc::raw_vec::RawVec::<u8>::ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                             debug self => _16; // in scope 9 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                             let mut _22: std::ptr::Unique<u8>; // in scope 9 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                             scope 10 (inlined Unique::<u8>::as_ptr) { // at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                                 debug self => _22; // in scope 10 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                                 let mut _23: std::ptr::NonNull<u8>; // in scope 10 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                                 scope 11 (inlined NonNull::<u8>::as_ptr) { // at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                                     debug self => _23; // in scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-                                     let mut _24: *const u8; // in scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-                             }
-                         }
-                     }
-                     }
-                     scope 19 (inlined std::slice::from_raw_parts::<u8>) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                         debug data => _12; // in scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         debug len => _14; // in scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         let mut _42: &*const u8; // in scope 19 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         let mut _43: &usize; // in scope 19 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         let _44: ();     // in scope 19 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         let mut _45: (); // in scope 19 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         let mut _46: [closure@std::slice::from_raw_parts<u8>::{closure#0}]; // in scope 19 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         let _47: *const [u8]; // in scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         let mut _48: *const u8; // in scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         let mut _49: usize; // in scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         scope 20 {
-                             let _41: [closure@std::slice::from_raw_parts<u8>::{closure#0}]; // in scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                             scope 21 {
-                                 debug runtime => _41; // in scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                             }
-                             scope 22 (inlined slice_from_raw_parts::<u8>) { // at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                                 debug data => _48; // in scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                 debug len => _49; // in scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                 let mut _50: *const (); // in scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                 let mut _51: *const u8; // in scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                 let mut _52: usize; // in scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                 scope 23 (inlined ptr::const_ptr::<impl *const u8>::cast::<()>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                     debug self => _51; // in scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-                                     let mut _53: *const u8; // in scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-                                 }
-                                 scope 24 (inlined std::ptr::from_raw_parts::<[u8]>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                     debug data_address => _50; // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     debug metadata => _52; // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     let mut _54: std::ptr::metadata::PtrRepr<[u8]>; // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     let mut _55: std::ptr::metadata::PtrComponents<[u8]>; // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     let mut _56: *const (); // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     let mut _57: usize; // in scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                                     scope 25 {
-                                 }
-                             }
-                         }
-                     }
-                     }
-                 }
-             }
-             scope 26 (inlined from_utf8_unchecked) { // at $SRC_DIR/alloc/src/string.rs:LL:COL
-                 debug v => _8;           // in scope 26 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-                 let mut _58: &[u8];      // in scope 26 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-                 scope 27 {
-             }
-         }
-     }
140 
140 
141     bb0: {
142         _7 = const true;                 // scope 0 at $DIR/string.rs:7:11: 7:12

149         _7 = const false;                // scope 0 at $DIR/string.rs:9:9: 9:10
150         _6 = move _1;                    // scope 0 at $DIR/string.rs:9:9: 9:10
151         _0 = const 4321_i32;             // scope 1 at $DIR/string.rs:9:14: 9:18
-         drop(_6) -> [return: bb5, unwind: bb11]; // scope 0 at $DIR/string.rs:9:17: 9:18
+         drop(_6) -> [return: bb6, unwind: bb12]; // scope 0 at $DIR/string.rs:9:17: 9:18
154 
155     bb2: {


156         _2 = &((_1 as Some).0: std::string::String); // scope 0 at $DIR/string.rs:8:14: 8:17
-         StorageLive(_8);                 // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageLive(_9);                 // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageLive(_10);                // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageLive(_11);                // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         _11 = &((*_2).0: std::vec::Vec<u8>); // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         _10 = _11;                       // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageLive(_12);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_13);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _13 = _10;                       // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_15);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_16);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _16 = &((*_13).0: alloc::raw_vec::RawVec<u8>); // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_22);                // scope 9 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         _22 = ((*_16).0: std::ptr::Unique<u8>); // scope 9 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         StorageLive(_23);                // scope 10 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         _23 = (_22.0: std::ptr::NonNull<u8>); // scope 10 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageLive(_24);                // scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _24 = (_23.0: *const u8);        // scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _15 = move _24 as *mut u8 (Misc); // scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_24);                // scope 11 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_23);                // scope 10 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageDead(_22);                // scope 9 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         StorageDead(_16);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_17);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_18);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_19);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_20);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _20 = _15;                       // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_25);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_26);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _26 = _20;                       // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _25 = move _26 as *mut u8 (Misc); // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_26);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_27);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_28);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_30);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _28 = transmute::<usize, *mut ()>(const 0_usize) -> bb13; // scope 15 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         _3 = <String as Deref>::deref(move _2) -> bb3; // scope 0 at $DIR/string.rs:8:14: 8:17
194                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(usize) -> *mut () {transmute::<usize, *mut ()>}, val: Value(<ZST>) }
+                                          // + span: $DIR/string.rs:8:14: 8:17
+                                          // + literal: Const { ty: for<'r> fn(&'r String) -> &'r <String as Deref>::Target {<String as Deref>::deref}, val: Value(<ZST>) }
198 
199     bb3: {


-         switchInt(move _4) -> [false: bb1, otherwise: bb4]; // scope 0 at $DIR/string.rs:8:14: 8:17
+         _4 = <str as PartialEq>::eq(_3, const "a") -> [return: bb4, unwind: bb12]; // scope 0 at $DIR/string.rs:8:14: 8:17
+                                          // mir::Constant
+                                          // + span: $DIR/string.rs:8:14: 8:17
+                                          // + literal: Const { ty: for<'r, 's> fn(&'r str, &'s str) -> bool {<str as PartialEq>::eq}, val: Value(<ZST>) }
+                                          // mir::Constant
+                                          // + span: $DIR/string.rs:8:14: 8:17
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
202 
203     bb4: {


-         _0 = const 1234_i32;             // scope 0 at $DIR/string.rs:8:22: 8:26
-         goto -> bb9;                     // scope 0 at $DIR/string.rs:8:22: 8:26
+         switchInt(move _4) -> [false: bb1, otherwise: bb5]; // scope 0 at $DIR/string.rs:8:14: 8:17
207 
208     bb5: {


-         StorageDead(_6);                 // scope 0 at $DIR/string.rs:9:17: 9:18
-         goto -> bb9;                     // scope 0 at $DIR/string.rs:9:17: 9:18
+         _0 = const 1234_i32;             // scope 0 at $DIR/string.rs:8:22: 8:26
+         goto -> bb10;                    // scope 0 at $DIR/string.rs:8:22: 8:26
212 
213     bb6: {


+         StorageDead(_6);                 // scope 0 at $DIR/string.rs:9:17: 9:18
+         goto -> bb10;                    // scope 0 at $DIR/string.rs:9:17: 9:18
+ 
+     bb7: {
+     bb7: {
214         return;                          // scope 0 at $DIR/string.rs:11:2: 11:2
216 


-     bb7 (cleanup): {
+     bb8 (cleanup): {
218         resume;                          // scope 0 at $DIR/string.rs:6:1: 11:2
220 

-     bb8: {
-     bb8: {
-         drop(_1) -> bb6;                 // scope 0 at $DIR/string.rs:11:1: 11:2
- 
225     bb9: {
225     bb9: {
-         switchInt(_7) -> [false: bb6, otherwise: bb8]; // scope 0 at $DIR/string.rs:11:1: 11:2
+         drop(_1) -> bb7;                 // scope 0 at $DIR/string.rs:11:1: 11:2
228 
228 
-     bb10 (cleanup): {
-         drop(_1) -> bb7;                 // scope 0 at $DIR/string.rs:11:1: 11:2
+     bb10: {
+         switchInt(_7) -> [false: bb7, otherwise: bb9]; // scope 0 at $DIR/string.rs:11:1: 11:2
232 
232 
233     bb11 (cleanup): {

-         switchInt(_7) -> [false: bb7, otherwise: bb10]; // scope 0 at $DIR/string.rs:11:1: 11:2
+         drop(_1) -> bb8;                 // scope 0 at $DIR/string.rs:11:1: 11:2
236 
-     bb12: {
-     bb12: {
-         StorageDead(_18);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_17);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_21);                // scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _21 = _15;                       // scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _12 = move _21 as *const u8 (Pointer(MutToConstPointer)); // scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_21);                // scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_15);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_13);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_14);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _14 = ((*_10).1: usize);         // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_41);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_42);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         _42 = &_12;                      // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_43);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         _43 = &_14;                      // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         Deinit(_41);                     // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         (_41.0: &*const u8) = move _42;  // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         (_41.1: &usize) = move _43;      // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageDead(_43);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageDead(_42);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_44);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_45);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_46);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         _46 = _41;                       // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         _44 = const_eval_select::<(), fn() {std::slice::from_raw_parts::comptime}, [closure@std::slice::from_raw_parts<u8>::{closure#0}], ()>(move _45, std::slice::from_raw_parts::comptime, move _46) -> bb15; // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                                          // + literal: Const { ty: unsafe fn((), fn() {std::slice::from_raw_parts::comptime}, [closure@std::slice::from_raw_parts<u8>::{closure#0}]) {const_eval_select::<(), fn() {std::slice::from_raw_parts::comptime}, [closure@std::slice::from_raw_parts<u8>::{closure#0}], ()>}, val: Value(<ZST>) }
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                                          // + literal: Const { ty: fn() {std::slice::from_raw_parts::comptime}, val: Value(<ZST>) }
- 
-     bb13: {
-     bb13: {
-         StorageDead(_30);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_29);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_31);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_32);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_33);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_34);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _34 = _28;                       // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _33 = move _34 as *const () (Pointer(MutToConstPointer)); // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_34);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         Deinit(_32);                     // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         (_32.0: *const ()) = move _33;   // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_33);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         Deinit(_31);                     // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         (_31.2: std::ptr::metadata::PtrComponents<u8>) = move _32; // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_32);                // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _27 = (_31.1: *mut u8);          // scope 17 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_31);                // scope 16 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_29);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageDead(_28);                // scope 13 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_35);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_36);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_37);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _37 = _25;                       // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _36 = move _37 as *const u8 (Pointer(MutToConstPointer)); // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_37);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _35 = _36;                       // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_38);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_39);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_40);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _40 = _27;                       // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _39 = move _40 as *const u8 (Pointer(MutToConstPointer)); // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_40);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _38 = _39;                       // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         _19 = ptr_guaranteed_eq::<u8>(move _35, move _38) -> bb14; // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                                          // + literal: Const { ty: extern "rust-intrinsic" fn(*const u8, *const u8) -> bool {ptr_guaranteed_eq::<u8>}, val: Value(<ZST>) }
- 
-     bb14: {
-     bb14: {
-         StorageDead(_38);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_35);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_39);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_36);                // scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_27);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_25);                // scope 12 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageDead(_20);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _18 = Not(move _19);             // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_19);                // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _17 = assume(move _18) -> bb12;  // scope 8 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(bool) {assume}, val: Value(<ZST>) }
- 
-     bb15: {
-     bb15: {
-         StorageDead(_46);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageDead(_45);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageDead(_44);                // scope 21 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageDead(_41);                // scope 20 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-         StorageLive(_47);                // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageLive(_48);                // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         _48 = _12;                       // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageLive(_49);                // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         _49 = _14;                       // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageLive(_50);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_51);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _51 = _48;                       // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_53);                // scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-         _53 = _51;                       // scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-         _50 = move _53 as *const () (Misc); // scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-         StorageDead(_53);                // scope 23 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-         StorageDead(_51);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_52);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _52 = _49;                       // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageLive(_54);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_55);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_56);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _56 = _50;                       // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_57);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _57 = _52;                       // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         Deinit(_55);                     // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         (_55.0: *const ()) = move _56;   // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         (_55.1: usize) = move _57;       // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_57);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_56);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         Deinit(_54);                     // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         (_54.2: std::ptr::metadata::PtrComponents<[u8]>) = move _55; // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_55);                // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _47 = (_54.0: *const [u8]);      // scope 25 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_54);                // scope 24 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_52);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageDead(_50);                // scope 22 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         StorageDead(_49);                // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_48);                // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         _9 = &(*_47);                    // scope 20 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_47);                // scope 19 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_14);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_12);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _8 = _9;                         // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageDead(_10);                // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageLive(_58);                // scope 27 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-         _58 = _8;                        // scope 27 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-         _3 = transmute::<&[u8], &str>(move _58) -> bb16; // scope 27 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/str/converts.rs:LL:COL
-                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&[u8]) -> &str {transmute::<&[u8], &str>}, val: Value(<ZST>) }
- 
-     bb16: {
-     bb16: {
-         StorageDead(_58);                // scope 27 at $SRC_DIR/core/src/str/converts.rs:LL:COL
-         StorageDead(_8);                 // scope 3 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageDead(_11);                // scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         StorageDead(_9);                 // scope 2 at $SRC_DIR/alloc/src/string.rs:LL:COL
-         _4 = <str as PartialEq>::eq(_3, const "a") -> [return: bb3, unwind: bb11]; // scope 0 at $DIR/string.rs:8:14: 8:17
-                                          // mir::Constant
-                                          // + span: $DIR/string.rs:8:14: 8:17
-                                          // + literal: Const { ty: for<'r, 's> fn(&'r str, &'s str) -> bool {<str as PartialEq>::eq}, val: Value(<ZST>) }
-                                          // mir::Constant
-                                          // + span: $DIR/string.rs:8:14: 8:17
-                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+     bb12 (cleanup): {
+         switchInt(_7) -> [false: bb8, otherwise: bb11]; // scope 0 at $DIR/string.rs:11:1: 11:2
394 }
395 


thread '[mir-opt] src/test/mir-opt/deref-patterns/string.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deref-patterns/string.foo.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3503:25


failures:
    [mir-opt] src/test/mir-opt/deref-patterns/string.rs
