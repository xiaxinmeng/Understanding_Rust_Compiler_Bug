plain
...............ii.i.ii.iii.........ii........i.............................F.

failures:

---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
35           StorageLive(_1);                 // scope 0 at $DIR/invalid_constant.rs:+6:9: +6:22
36           StorageLive(_2);                 // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:63
37           _2 = InvalidChar { int: const 1114113_u32 }; // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:63
-           _1 = (_2.1: char);               // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:67
+ -         _1 = (_2.1: char);               // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:67
+ +         _1 = const {transmute(0x00110001): char}; // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:67
39           StorageDead(_2);                 // scope 0 at $DIR/invalid_constant.rs:+6:69: +6:70
40           StorageLive(_3);                 // scope 1 at $DIR/invalid_constant.rs:+13:9: +13:21
41           StorageLive(_4);                 // scope 1 at $DIR/invalid_constant.rs:+13:25: +13:59
Build completed unsuccessfully in 0:13:16
Build completed unsuccessfully in 0:13:16
42           StorageLive(_5);                 // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
43           _5 = InvalidTag { int: const 4_u32 }; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
-           _4 = (_5.1: E);                  // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
+ -         _4 = (_5.1: E);                  // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
+ +         _4 = const Scalar(0x00000004): E; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
+ +                                          // mir::Constant
+ +                                          // + span: no-location
+ +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
45           _3 = [move _4];                  // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
46           StorageDead(_4);                 // scope 1 at $DIR/invalid_constant.rs:+13:59: +13:60
47           StorageDead(_5);                 // scope 1 at $DIR/invalid_constant.rs:+13:60: +13:61

thread '[mir-opt] tests/mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3581:21

---- [mir-opt] tests/mir-opt/pre-codegen/loops.rs stdout ----
---- [mir-opt] tests/mir-opt/pre-codegen/loops.rs stdout ----
6     let mut _2: std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
7     let mut _3: std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
8     let mut _4: &mut [impl Sized];       // in scope 0 at $DIR/loops.rs:+1:14: +1:26
-     let mut _5: std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
-     let _6: ();                          // in scope 0 at $DIR/loops.rs:+1:14: +1:26
-     let mut _7: std::option::Option<&mut impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
-     let mut _8: &mut std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
-     let mut _9: isize;                   // in scope 0 at $DIR/loops.rs:+1:5: +3:6
-     let mut _11: std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _5: &mut std::vec::Vec<impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
+     let mut _6: std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
+     let _7: ();                          // in scope 0 at $DIR/loops.rs:+1:14: +1:26
+     let mut _8: std::option::Option<&mut impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
+     let mut _9: &mut std::slice::IterMut<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+1:14: +1:26
+     let mut _10: isize;                  // in scope 0 at $DIR/loops.rs:+1:5: +3:6
15     let mut _12: std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let _13: &[impl Sized];              // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let mut _14: std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let _15: ();                         // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let mut _16: std::option::Option<&impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let mut _17: &mut std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
-     let mut _18: isize;                  // in scope 0 at $DIR/loops.rs:+4:5: +6:6
+     let mut _13: std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _14: &[impl Sized];          // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let _15: &[impl Sized];              // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _16: &std::vec::Vec<impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _17: std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let _18: ();                         // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _19: std::option::Option<&impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _20: &mut std::slice::Iter<'_, impl Sized>; // in scope 0 at $DIR/loops.rs:+4:14: +4:22
+     let mut _21: isize;                  // in scope 0 at $DIR/loops.rs:+4:5: +6:6
22     scope 1 {
-         debug iter => _5;                // in scope 1 at $DIR/loops.rs:+1:14: +1:26
-         let _10: &mut impl Sized;        // in scope 1 at $DIR/loops.rs:+1:9: +1:10
+         debug iter => _6;                // in scope 1 at $DIR/loops.rs:+1:14: +1:26
+         let _11: &mut impl Sized;        // in scope 1 at $DIR/loops.rs:+1:9: +1:10
25         scope 2 {
-             debug x => _10;              // in scope 2 at $DIR/loops.rs:+1:9: +1:10
+             debug x => _11;              // in scope 2 at $DIR/loops.rs:+1:9: +1:10
28     }
29     scope 3 {


-         debug iter => _14;               // in scope 3 at $DIR/loops.rs:+4:14: +4:22
-         let _19: &impl Sized;            // in scope 3 at $DIR/loops.rs:+4:9: +4:10
+         debug iter => _17;               // in scope 3 at $DIR/loops.rs:+4:14: +4:22
+         let _22: &impl Sized;            // in scope 3 at $DIR/loops.rs:+4:9: +4:10
32         scope 4 {
-             debug x => _19;              // in scope 4 at $DIR/loops.rs:+4:9: +4:10
+             debug x => _22;              // in scope 4 at $DIR/loops.rs:+4:9: +4:10
35     }
35     }
36     scope 5 (inlined <Vec<impl Sized> as DerefMut>::deref_mut) { // at $DIR/loops.rs:24:14: 24:26

37         debug self => &_1;               // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         let mut _20: *mut impl Sized;    // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         let mut _21: usize;              // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         let mut _23: &mut [impl Sized];  // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         let mut _24: *mut impl Sized;    // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         let mut _25: usize;              // in scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
40         scope 6 {
41             scope 7 (inlined Vec::<impl Sized>::as_mut_ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
42                 debug self => &_1;       // in scope 7 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL

43                 scope 8 (inlined alloc::raw_vec::RawVec::<impl Sized>::ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                     debug self => &(_1.0: alloc::raw_vec::RawVec<impl Sized>); // in scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                     let mut _22: std::ptr::Unique<impl Sized>; // in scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+                     debug self => &((*_5).0: alloc::raw_vec::RawVec<impl Sized>); // in scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+                     let mut _26: std::ptr::Unique<impl Sized>; // in scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
46                     scope 9 (inlined Unique::<impl Sized>::as_ptr) { // at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                         debug self => _22; // in scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                         let mut _23: std::ptr::NonNull<impl Sized>; // in scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+                         debug self => _26; // in scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+                         let mut _27: std::ptr::NonNull<impl Sized>; // in scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
49                         scope 10 (inlined NonNull::<impl Sized>::as_ptr) { // at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                             debug self => _23; // in scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-                             let mut _24: *const impl Sized; // in scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                             debug self => _27; // in scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                             let mut _28: *const impl Sized; // in scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
53                     }
54                 }

55             }
55             }
-             scope 11 (inlined std::slice::from_raw_parts_mut::<'_, impl Sized>) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 debug data => _20;       // in scope 11 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 debug len => _21;        // in scope 11 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 let mut _25: *mut [impl Sized]; // in scope 11 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 let mut _26: *mut impl Sized; // in scope 11 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                 let mut _27: usize;      // in scope 11 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                 scope 12 {
-                     scope 13 (inlined std::slice::from_raw_parts_mut::runtime::<impl Sized>) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         debug data => _26; // in scope 13 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         debug len => _27; // in scope 13 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         scope 14 (inlined intrinsics::is_valid_allocation_size::<impl Sized>) { // at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                             debug len => _27; // in scope 14 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                             scope 15 {
-                                 debug max_len => const _; // in scope 15 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         }
-                     }
-                     }
-                     scope 16 (inlined slice_from_raw_parts_mut::<impl Sized>) { // at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         debug data => _20; // in scope 16 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         debug len => _21; // in scope 16 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         let mut _28: *mut (); // in scope 16 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         scope 17 (inlined ptr::mut_ptr::<impl *mut impl Sized>::cast::<()>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                             debug self => _20; // in scope 17 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-                         }
-                         scope 18 (inlined std::ptr::from_raw_parts_mut::<[impl Sized]>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                             debug data_address => _28; // in scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             debug metadata => _21; // in scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             let mut _29: std::ptr::metadata::PtrRepr<[impl Sized]>; // in scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             let mut _30: std::ptr::metadata::PtrComponents<[impl Sized]>; // in scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             let mut _31: *const (); // in scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             scope 19 {
-                         }
-                     }
-                 }
-             }
-             }
92         }
93     }
-     scope 20 (inlined core::slice::<impl [impl Sized]>::iter_mut) { // at $DIR/loops.rs:24:16: 24:26
-         debug self => _4;                // in scope 20 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+     scope 11 (inlined core::slice::<impl [impl Sized]>::iter_mut) { // at $DIR/loops.rs:24:16: 24:26
+         debug self => _4;                // in scope 11 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
96     }
-     scope 21 (inlined <std::slice::IterMut<'_, impl Sized> as IntoIterator>::into_iter) { // at $DIR/loops.rs:24:14: 24:26
-         debug self => _3;                // in scope 21 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+     scope 12 (inlined <std::slice::IterMut<'_, impl Sized> as IntoIterator>::into_iter) { // at $DIR/loops.rs:24:14: 24:26
+         debug self => _3;                // in scope 12 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
99     }
-     scope 22 (inlined <Vec<impl Sized> as Deref>::deref) { // at $DIR/loops.rs:27:14: 27:22
-         debug self => &_1;               // in scope 22 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         let mut _32: *const impl Sized;  // in scope 22 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         let mut _33: usize;              // in scope 22 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         scope 23 {
-             scope 24 (inlined Vec::<impl Sized>::as_ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 debug self => &_1;       // in scope 24 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 let mut _34: *mut impl Sized; // in scope 24 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 scope 25 (inlined alloc::raw_vec::RawVec::<impl Sized>::ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                     debug self => &(_1.0: alloc::raw_vec::RawVec<impl Sized>); // in scope 25 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                     let mut _35: std::ptr::Unique<impl Sized>; // in scope 25 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                     scope 26 (inlined Unique::<impl Sized>::as_ptr) { // at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-                         debug self => _35; // in scope 26 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                         let mut _36: std::ptr::NonNull<impl Sized>; // in scope 26 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                         scope 27 (inlined NonNull::<impl Sized>::as_ptr) { // at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-                             debug self => _36; // in scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-                             let mut _37: *const impl Sized; // in scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+     scope 13 (inlined <Vec<impl Sized> as Deref>::deref) { // at $DIR/loops.rs:27:14: 27:22
+         debug self => &_1;               // in scope 13 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         let mut _29: *const impl Sized;  // in scope 13 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         let mut _30: usize;              // in scope 13 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         scope 14 {
+             scope 15 (inlined Vec::<impl Sized>::as_ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                 debug self => &_1;       // in scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                 let mut _31: *mut impl Sized; // in scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                 scope 16 (inlined alloc::raw_vec::RawVec::<impl Sized>::ptr) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                     debug self => &((*_16).0: alloc::raw_vec::RawVec<impl Sized>); // in scope 16 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+                     let mut _32: std::ptr::Unique<impl Sized>; // in scope 16 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+                     scope 17 (inlined Unique::<impl Sized>::as_ptr) { // at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+                         debug self => _32; // in scope 17 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+                         let mut _33: std::ptr::NonNull<impl Sized>; // in scope 17 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+                         scope 18 (inlined NonNull::<impl Sized>::as_ptr) { // at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+                             debug self => _33; // in scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                             let mut _34: *const impl Sized; // in scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
118                     }
119                 }

120             }
120             }
-             scope 28 (inlined std::slice::from_raw_parts::<'_, impl Sized>) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-                 debug data => _32;       // in scope 28 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 debug len => _33;        // in scope 28 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 let _38: *const [impl Sized]; // in scope 28 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                 let mut _39: *const impl Sized; // in scope 28 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                 let mut _40: usize;      // in scope 28 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                 scope 29 {
-                     scope 30 (inlined std::slice::from_raw_parts::runtime::<impl Sized>) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         debug data => _39; // in scope 30 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         debug len => _40; // in scope 30 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         scope 31 (inlined intrinsics::is_valid_allocation_size::<impl Sized>) { // at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                             debug len => _40; // in scope 31 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                             scope 32 {
-                                 debug max_len => const _; // in scope 32 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         }
-                     }
-                     }
-                     scope 33 (inlined slice_from_raw_parts::<impl Sized>) { // at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-                         debug data => _32; // in scope 33 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         debug len => _33; // in scope 33 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         let mut _41: *const (); // in scope 33 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                         scope 34 (inlined ptr::const_ptr::<impl *const impl Sized>::cast::<()>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                             debug self => _32; // in scope 34 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-                         }
-                         scope 35 (inlined std::ptr::from_raw_parts::<[impl Sized]>) { // at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                             debug data_address => _41; // in scope 35 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             debug metadata => _33; // in scope 35 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             let mut _42: std::ptr::metadata::PtrRepr<[impl Sized]>; // in scope 35 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             let mut _43: std::ptr::metadata::PtrComponents<[impl Sized]>; // in scope 35 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-                             scope 36 {
-                         }
-                     }
-                 }
-             }
-             }
156         }
157     }
-     scope 37 (inlined core::slice::<impl [impl Sized]>::iter) { // at $DIR/loops.rs:27:16: 27:22
-         debug self => _38;               // in scope 37 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+     scope 19 (inlined core::slice::<impl [impl Sized]>::iter) { // at $DIR/loops.rs:27:16: 27:22
+         debug self => _14;               // in scope 19 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
160     }
-     scope 38 (inlined <std::slice::Iter<'_, impl Sized> as IntoIterator>::into_iter) { // at $DIR/loops.rs:27:14: 27:22
-         debug self => _12;               // in scope 38 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+     scope 20 (inlined <std::slice::Iter<'_, impl Sized> as IntoIterator>::into_iter) { // at $DIR/loops.rs:27:14: 27:22
+         debug self => _13;               // in scope 20 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
164 
165     bb0: {


166         StorageLive(_2);                 // scope 0 at $DIR/loops.rs:+1:14: +1:26
167         StorageLive(_3);                 // scope 0 at $DIR/loops.rs:+1:14: +1:26
-         StorageLive(_20);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_22);                // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         _22 = ((_1.0: alloc::raw_vec::RawVec<impl Sized>).0: std::ptr::Unique<impl Sized>); // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         StorageLive(_23);                // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         _23 = (_22.0: std::ptr::NonNull<impl Sized>); // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageLive(_24);                // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _24 = (_23.0: *const impl Sized); // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _20 = move _24 as *mut impl Sized (PtrToPtr); // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_24);                // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_23);                // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageDead(_22);                // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         StorageLive(_21);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _21 = (_1.1: usize);             // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_26);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_27);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_25);                // scope 12 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageLive(_28);                // scope 16 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _28 = _20 as *mut () (PtrToPtr); // scope 17 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-         StorageLive(_29);                // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_30);                // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_31);                // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _31 = _28 as *const () (Pointer(MutToConstPointer)); // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _30 = ptr::metadata::PtrComponents::<[impl Sized]> { data_address: move _31, metadata: _21 }; // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_31);                // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _29 = ptr::metadata::PtrRepr::<[impl Sized]> { const_ptr: move _30 }; // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_30);                // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _25 = (_29.1: *mut [impl Sized]); // scope 19 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_29);                // scope 18 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_28);                // scope 16 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _4 = &mut (*_25);                // scope 12 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_25);                // scope 11 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_27);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_26);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_21);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_20);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _3 = std::slice::IterMut::<'_, impl Sized>::new(_4) -> [return: bb15, unwind: bb13]; // scope 20 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageLive(_5);                 // scope 0 at $DIR/loops.rs:+1:14: +1:26
+         StorageLive(_23);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageLive(_24);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageLive(_26);                // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         _26 = ((_1.0: alloc::raw_vec::RawVec<impl Sized>).0: std::ptr::Unique<impl Sized>); // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         StorageLive(_27);                // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         _27 = (_26.0: std::ptr::NonNull<impl Sized>); // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         StorageLive(_28);                // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         _28 = (_27.0: *const impl Sized); // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         _24 = move _28 as *mut impl Sized (PtrToPtr); // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         StorageDead(_28);                // scope 10 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         StorageDead(_27);                // scope 9 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         StorageDead(_26);                // scope 8 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         StorageLive(_25);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         _25 = (_1.1: usize);             // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         _23 = std::slice::from_raw_parts_mut::<'_, impl Sized>(move _24, move _25) -> [return: bb15, unwind: bb13]; // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
204                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/slice/mod.rs:LL:COL
-                                          // + user_ty: UserType(0)
-                                          // + literal: Const { ty: fn(&mut [impl Sized]) -> std::slice::IterMut<'_, impl Sized> {std::slice::IterMut::<'_, impl Sized>::new}, val: Value(<ZST>) }
+                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                                          // + literal: Const { ty: unsafe fn(*mut impl Sized, usize) -> &mut [impl Sized] {std::slice::from_raw_parts_mut::<'_, impl Sized>}, val: Value(<ZST>) }
209 
210     bb1: {


-         StorageLive(_7);                 // scope 1 at $DIR/loops.rs:+1:14: +1:26
-         _8 = &mut _5;                    // scope 1 at $DIR/loops.rs:+1:14: +1:26
-         _7 = <std::slice::IterMut<'_, impl Sized> as Iterator>::next(_8) -> [return: bb2, unwind: bb13]; // scope 1 at $DIR/loops.rs:+1:14: +1:26
+         StorageLive(_8);                 // scope 1 at $DIR/loops.rs:+1:14: +1:26
+         _9 = &mut _6;                    // scope 1 at $DIR/loops.rs:+1:14: +1:26
+         _8 = <std::slice::IterMut<'_, impl Sized> as Iterator>::next(_9) -> [return: bb2, unwind: bb13]; // scope 1 at $DIR/loops.rs:+1:14: +1:26
214                                          // mir::Constant
215                                          // + span: $DIR/loops.rs:24:14: 24:26
216                                          // + literal: Const { ty: for<'a> fn(&'a mut std::slice::IterMut<'_, impl Sized>) -> Option<<std::slice::IterMut<'_, impl Sized> as Iterator>::Item> {<std::slice::IterMut<'_, impl Sized> as Iterator>::next}, val: Value(<ZST>) }
217     }
218 
219     bb2: {
219     bb2: {
-         _9 = discriminant(_7);           // scope 1 at $DIR/loops.rs:+1:14: +1:26
-         switchInt(move _9) -> [0: bb5, 1: bb3, otherwise: bb4]; // scope 1 at $DIR/loops.rs:+1:14: +1:26
+         _10 = discriminant(_8);          // scope 1 at $DIR/loops.rs:+1:14: +1:26
+         switchInt(move _10) -> [0: bb5, 1: bb3, otherwise: bb4]; // scope 1 at $DIR/loops.rs:+1:14: +1:26
223 
224     bb3: {


-         _10 = move ((_7 as Some).0: &mut impl Sized); // scope 1 at $DIR/loops.rs:+1:9: +1:10
-         _6 = opaque::<&mut impl Sized>(move _10) -> [return: bb6, unwind: bb13]; // scope 2 at $DIR/loops.rs:+2:9: +2:18
+         _11 = move ((_8 as Some).0: &mut impl Sized); // scope 1 at $DIR/loops.rs:+1:9: +1:10
+         _7 = opaque::<&mut impl Sized>(move _11) -> [return: bb6, unwind: bb13]; // scope 2 at $DIR/loops.rs:+2:9: +2:18
227                                          // mir::Constant
228                                          // + span: $DIR/loops.rs:25:9: 25:15
229                                          // + literal: Const { ty: fn(&mut impl Sized) {opaque::<&mut impl Sized>}, val: Value(<ZST>) }
234     }
235 
236     bb5: {
236     bb5: {
-         StorageDead(_7);                 // scope 1 at $DIR/loops.rs:+3:5: +3:6
-         StorageDead(_5);                 // scope 0 at $DIR/loops.rs:+3:5: +3:6
+         StorageDead(_8);                 // scope 1 at $DIR/loops.rs:+3:5: +3:6
+         StorageDead(_6);                 // scope 0 at $DIR/loops.rs:+3:5: +3:6
239         StorageDead(_2);                 // scope 0 at $DIR/loops.rs:+3:5: +3:6
-         StorageLive(_11);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
241         StorageLive(_12);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
-         StorageLive(_32);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_34);                // scope 24 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_35);                // scope 25 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         _35 = ((_1.0: alloc::raw_vec::RawVec<impl Sized>).0: std::ptr::Unique<impl Sized>); // scope 25 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         StorageLive(_36);                // scope 26 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         _36 = (_35.0: std::ptr::NonNull<impl Sized>); // scope 26 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageLive(_37);                // scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _37 = (_36.0: *const impl Sized); // scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         _34 = move _37 as *mut impl Sized (PtrToPtr); // scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_37);                // scope 27 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
-         StorageDead(_36);                // scope 26 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
-         StorageDead(_35);                // scope 25 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
-         _32 = move _34 as *const impl Sized (Pointer(MutToConstPointer)); // scope 24 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_34);                // scope 24 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_33);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _33 = (_1.1: usize);             // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_39);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_40);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageLive(_38);                // scope 29 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageLive(_41);                // scope 33 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _41 = _32 as *const () (PtrToPtr); // scope 34 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-         StorageLive(_42);                // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageLive(_43);                // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _43 = ptr::metadata::PtrComponents::<[impl Sized]> { data_address: _41, metadata: _33 }; // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _42 = ptr::metadata::PtrRepr::<[impl Sized]> { const_ptr: move _43 }; // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_43);                // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         _38 = (_42.0: *const [impl Sized]); // scope 36 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_42);                // scope 35 at $SRC_DIR/core/src/ptr/metadata.rs:LL:COL
-         StorageDead(_41);                // scope 33 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-         _13 = &(*_38);                   // scope 29 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_38);                // scope 28 at $SRC_DIR/core/src/slice/raw.rs:LL:COL
-         StorageDead(_40);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_39);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_33);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         StorageDead(_32);                // scope 23 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
-         _12 = std::slice::Iter::<'_, impl Sized>::new(_13) -> [return: bb16, unwind: bb13]; // scope 37 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageLive(_13);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
+         StorageLive(_14);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
+         StorageLive(_15);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
+         StorageLive(_16);                // scope 0 at $DIR/loops.rs:+4:14: +4:22
+         StorageLive(_29);                // scope 14 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageLive(_31);                // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageLive(_32);                // scope 16 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         _32 = ((_1.0: alloc::raw_vec::RawVec<impl Sized>).0: std::ptr::Unique<impl Sized>); // scope 16 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         StorageLive(_33);                // scope 17 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         _33 = (_32.0: std::ptr::NonNull<impl Sized>); // scope 17 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         StorageLive(_34);                // scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         _34 = (_33.0: *const impl Sized); // scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         _31 = move _34 as *mut impl Sized (PtrToPtr); // scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         StorageDead(_34);                // scope 18 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         StorageDead(_33);                // scope 17 at $SRC_DIR/core/src/ptr/unique.rs:LL:COL
+         StorageDead(_32);                // scope 16 at $SRC_DIR/alloc/src/raw_vec.rs:LL:COL
+         _29 = move _31 as *const impl Sized (Pointer(MutToConstPointer)); // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageDead(_31);                // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageLive(_30);                // scope 14 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         _30 = (_1.1: usize);             // scope 14 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         _15 = std::slice::from_raw_parts::<'_, impl Sized>(move _29, move _30) -> [return: bb17, unwind: bb13]; // scope 14 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
278                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/slice/mod.rs:LL:COL
-                                          // + user_ty: UserType(0)
-                                          // + literal: Const { ty: fn(&[impl Sized]) -> std::slice::Iter<'_, impl Sized> {std::slice::Iter::<'_, impl Sized>::new}, val: Value(<ZST>) }
+                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                                          // + literal: Const { ty: unsafe fn(*const impl Sized, usize) -> &[impl Sized] {std::slice::from_raw_parts::<'_, impl Sized>}, val: Value(<ZST>) }
283 
284     bb6: {


-         StorageDead(_7);                 // scope 1 at $DIR/loops.rs:+3:5: +3:6
+         StorageDead(_8);                 // scope 1 at $DIR/loops.rs:+3:5: +3:6
286         goto -> bb1;                     // scope 1 at $DIR/loops.rs:+1:5: +3:6
288 

289     bb7: {
289     bb7: {
-         StorageLive(_16);                // scope 3 at $DIR/loops.rs:+4:14: +4:22
-         _17 = &mut _14;                  // scope 3 at $DIR/loops.rs:+4:14: +4:22
-         _16 = <std::slice::Iter<'_, impl Sized> as Iterator>::next(_17) -> [return: bb8, unwind: bb13]; // scope 3 at $DIR/loops.rs:+4:14: +4:22
+         StorageLive(_19);                // scope 3 at $DIR/loops.rs:+4:14: +4:22
+         _20 = &mut _17;                  // scope 3 at $DIR/loops.rs:+4:14: +4:22
+         _19 = <std::slice::Iter<'_, impl Sized> as Iterator>::next(_20) -> [return: bb8, unwind: bb13]; // scope 3 at $DIR/loops.rs:+4:14: +4:22
293                                          // mir::Constant
294                                          // + span: $DIR/loops.rs:27:14: 27:22
295                                          // + literal: Const { ty: for<'a> fn(&'a mut std::slice::Iter<'_, impl Sized>) -> Option<<std::slice::Iter<'_, impl Sized> as Iterator>::Item> {<std::slice::Iter<'_, impl Sized> as Iterator>::next}, val: Value(<ZST>) }
296     }
297 
298     bb8: {
298     bb8: {
-         _18 = discriminant(_16);         // scope 3 at $DIR/loops.rs:+4:14: +4:22
-         switchInt(move _18) -> [0: bb10, 1: bb9, otherwise: bb4]; // scope 3 at $DIR/loops.rs:+4:14: +4:22
+         _21 = discriminant(_19);         // scope 3 at $DIR/loops.rs:+4:14: +4:22
+         switchInt(move _21) -> [0: bb10, 1: bb9, otherwise: bb4]; // scope 3 at $DIR/loops.rs:+4:14: +4:22
302 
303     bb9: {


-         _19 = ((_16 as Some).0: &impl Sized); // scope 3 at $DIR/loops.rs:+4:9: +4:10
-         _15 = opaque::<&impl Sized>(_19) -> [return: bb11, unwind: bb13]; // scope 4 at $DIR/loops.rs:+5:9: +5:18
+         _22 = ((_19 as Some).0: &impl Sized); // scope 3 at $DIR/loops.rs:+4:9: +4:10
+         _18 = opaque::<&impl Sized>(_22) -> [return: bb11, unwind: bb13]; // scope 4 at $DIR/loops.rs:+5:9: +5:18
306                                          // mir::Constant
307                                          // + span: $DIR/loops.rs:28:9: 28:15
308                                          // + literal: Const { ty: fn(&impl Sized) {opaque::<&impl Sized>}, val: Value(<ZST>) }
309     }
310 
311     bb10: {
311     bb10: {
-         StorageDead(_16);                // scope 3 at $DIR/loops.rs:+6:5: +6:6
-         StorageDead(_14);                // scope 0 at $DIR/loops.rs:+6:5: +6:6
-         StorageDead(_11);                // scope 0 at $DIR/loops.rs:+6:5: +6:6
+         StorageDead(_19);                // scope 3 at $DIR/loops.rs:+6:5: +6:6
+         StorageDead(_17);                // scope 0 at $DIR/loops.rs:+6:5: +6:6
+         StorageDead(_15);                // scope 0 at $DIR/loops.rs:+6:5: +6:6
+         StorageDead(_12);                // scope 0 at $DIR/loops.rs:+6:5: +6:6
315         drop(_1) -> bb12;                // scope 0 at $DIR/loops.rs:+7:1: +7:2
317 

318     bb11: {
318     bb11: {
-         StorageDead(_16);                // scope 3 at $DIR/loops.rs:+6:5: +6:6
+         StorageDead(_19);                // scope 3 at $DIR/loops.rs:+6:5: +6:6
320         goto -> bb7;                     // scope 3 at $DIR/loops.rs:+4:5: +6:6
322 

333     }
334 
334 
335     bb15: {
-         _2 = move _3;                    // scope 21 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+         _4 = _23;                        // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageDead(_25);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageDead(_24);                // scope 6 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageDead(_23);                // scope 5 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+         StorageDead(_5);                 // scope 0 at $DIR/loops.rs:+1:14: +1:15
+         _3 = std::slice::IterMut::<'_, impl Sized>::new(_4) -> [return: bb16, unwind: bb13]; // scope 11 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/slice/mod.rs:LL:COL
+                                          // + user_ty: UserType(0)
+                                          // + literal: Const { ty: fn(&mut [impl Sized]) -> std::slice::IterMut<'_, impl Sized> {std::slice::IterMut::<'_, impl Sized>::new}, val: Value(<ZST>) }
+ 
+     bb16: {
+     bb16: {
+         _2 = move _3;                    // scope 12 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
337         StorageDead(_3);                 // scope 0 at $DIR/loops.rs:+1:25: +1:26
-         StorageLive(_5);                 // scope 0 at $DIR/loops.rs:+1:14: +1:26
