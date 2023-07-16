plain
test [mir-opt] tests/mir-opt/unusual_item_types.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/pre-codegen/slice_iter.rs stdout ----
6     let mut _0: ();                      // return place in scope 0 at $DIR/slice_iter.rs:+0:60: +0:60
7     let mut _3: std::slice::Iter<'_, T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
8     let mut _4: std::slice::Iter<'_, T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-     let mut _5: std::slice::Iter<'_, T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-     let _6: ();                          // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-     let mut _7: std::option::Option<&T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-     let mut _8: &mut std::slice::Iter<'_, T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-     let mut _9: isize;                   // in scope 0 at $DIR/slice_iter.rs:+1:5: +3:6
-     let mut _11: &impl Fn(&T);           // in scope 0 at $DIR/slice_iter.rs:+2:9: +2:10
-     let mut _12: (&T,);                  // in scope 0 at $DIR/slice_iter.rs:+2:9: +2:13
+     let _5: ();                          // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
+     let mut _6: std::option::Option<&T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
+     let mut _7: &mut std::slice::Iter<'_, T>; // in scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
+     let mut _8: isize;                   // in scope 0 at $DIR/slice_iter.rs:+1:5: +3:6
+     let mut _10: &impl Fn(&T);           // in scope 0 at $DIR/slice_iter.rs:+2:9: +2:10
+     let mut _11: (&T,);                  // in scope 0 at $DIR/slice_iter.rs:+2:9: +2:13
16     scope 1 {
-         debug iter => _5;                // in scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
-         let _10: &T;                     // in scope 1 at $DIR/slice_iter.rs:+1:9: +1:10
+         debug iter => _4;                // in scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         let _9: &T;                      // in scope 1 at $DIR/slice_iter.rs:+1:9: +1:10
19         scope 2 {
-             debug x => _10;              // in scope 2 at $DIR/slice_iter.rs:+1:9: +1:10
+             debug x => _9;               // in scope 2 at $DIR/slice_iter.rs:+1:9: +1:10
22     }
22     }
23     scope 3 (inlined core::slice::<impl [T]>::iter) { // at $DIR/slice_iter.rs:28:20: 28:26

24         debug self => _1;                // in scope 3 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         scope 4 (inlined std::slice::Iter::<'_, T>::new) { // at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+             debug slice => _1;           // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let _12: *const T;           // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _14: bool;           // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _15: usize;          // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _16: usize;          // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _17: std::ptr::NonNull<T>; // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _18: *mut T;         // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             let mut _19: *const T;       // in scope 4 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+             scope 5 {
+                 debug ptr => _12;        // in scope 5 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                 scope 6 {
+                     let _13: *const T;   // in scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                     scope 7 {
+                         debug end => _13; // in scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                         scope 13 (inlined NonNull::<T>::new_unchecked) { // at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                             debug ptr => _18; // in scope 13 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                             let mut _21: *const T; // in scope 13 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                             let mut _22: *mut T; // in scope 13 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                             scope 14 {
+                                 scope 15 (inlined NonNull::<T>::new_unchecked::runtime::<T>) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                                     debug ptr => _22; // in scope 15 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                                     scope 16 (inlined ptr::mut_ptr::<impl *mut T>::is_null) { // at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+                                         debug self => _22; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                         let mut _23: *mut u8; // in scope 16 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                         scope 17 {
+                                             scope 18 (inlined ptr::mut_ptr::<impl *mut T>::is_null::runtime_impl) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                 debug ptr => _23; // in scope 18 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                 scope 19 (inlined ptr::mut_ptr::<impl *mut u8>::addr) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                     debug self => _23; // in scope 19 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                     scope 20 {
+                                                         scope 21 (inlined ptr::mut_ptr::<impl *mut u8>::cast::<()>) { // at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                             debug self => _23; // in scope 21 at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+                                                     }
+                                                 }
+                                             }
+                                         }
+                                         }
+                                     }
+                                 }
+                             }
+                         }
+                     }
+                     scope 9 (inlined invalid::<T>) { // at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                         debug addr => _15; // in scope 9 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+                         scope 10 {
+                     }
+                     }
+                     scope 11 (inlined ptr::const_ptr::<impl *const T>::add) { // at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                         debug self => _12; // in scope 11 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+                         debug count => _16; // in scope 11 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+                         scope 12 {
+                     }
+                 }
+             }
+             }
+             scope 8 (inlined core::slice::<impl [T]>::as_ptr) { // at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                 debug self => _1;        // in scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+                 let mut _20: *const [T]; // in scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         }
25     }
25     }
-     scope 4 (inlined <std::slice::Iter<'_, T> as IntoIterator>::into_iter) { // at $DIR/slice_iter.rs:28:14: 28:26
-         debug self => _4;                // in scope 4 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
+     scope 22 (inlined <std::slice::Iter<'_, T> as IntoIterator>::into_iter) { // at $DIR/slice_iter.rs:28:14: 28:26
+         debug self => _3;                // in scope 22 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
29 
30     bb0: {


-         StorageLive(_3);                 // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-         StorageLive(_4);                 // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-         _4 = std::slice::Iter::<'_, T>::new(_1) -> [return: bb10, unwind: bb8]; // scope 3 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/slice/mod.rs:LL:COL
-                                          // + user_ty: UserType(0)
-                                          // + literal: Const { ty: fn(&[T]) -> std::slice::Iter<'_, T> {std::slice::Iter::<'_, T>::new}, val: Value(<ZST>) }
+         StorageLive(_12);                // scope 3 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageLive(_20);                // scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         _20 = &raw const (*_1);          // scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         _12 = move _20 as *const T (PtrToPtr); // scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageDead(_20);                // scope 8 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageLive(_13);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
Build completed unsuccessfully in 0:05:39
+         StorageLive(_14);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _14 = const _;                   // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         switchInt(move _14) -> [0: bb11, otherwise: bb10]; // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
39 
40     bb1: {


-         StorageLive(_7);                 // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
-         _8 = &mut _5;                    // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
-         _7 = <std::slice::Iter<'_, T> as Iterator>::next(_8) -> [return: bb2, unwind: bb8]; // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         StorageLive(_6);                 // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         _7 = &mut _4;                    // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         _6 = <std::slice::Iter<'_, T> as Iterator>::next(_7) -> [return: bb2, unwind: bb8]; // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
44                                          // mir::Constant
45                                          // + span: $DIR/slice_iter.rs:28:14: 28:26
46                                          // + literal: Const { ty: for<'a> fn(&'a mut std::slice::Iter<'_, T>) -> Option<<std::slice::Iter<'_, T> as Iterator>::Item> {<std::slice::Iter<'_, T> as Iterator>::next}, val: Value(<ZST>) }
47     }
48 
49     bb2: {
49     bb2: {
-         _9 = discriminant(_7);           // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
-         switchInt(move _9) -> [0: bb5, 1: bb3, otherwise: bb4]; // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         _8 = discriminant(_6);           // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
+         switchInt(move _8) -> [0: bb5, 1: bb3, otherwise: bb4]; // scope 1 at $DIR/slice_iter.rs:+1:14: +1:26
53 
54     bb3: {


-         _10 = ((_7 as Some).0: &T);      // scope 1 at $DIR/slice_iter.rs:+1:9: +1:10
-         StorageLive(_11);                // scope 2 at $DIR/slice_iter.rs:+2:9: +2:10
-         _11 = &_2;                       // scope 2 at $DIR/slice_iter.rs:+2:9: +2:10
-         StorageLive(_12);                // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
-         _12 = (_10,);                    // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
-         _6 = <impl Fn(&T) as Fn<(&T,)>>::call(move _11, move _12) -> [return: bb6, unwind: bb8]; // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
+         _9 = ((_6 as Some).0: &T);       // scope 1 at $DIR/slice_iter.rs:+1:9: +1:10
+         StorageLive(_10);                // scope 2 at $DIR/slice_iter.rs:+2:9: +2:10
+         _10 = &_2;                       // scope 2 at $DIR/slice_iter.rs:+2:9: +2:10
+         StorageLive(_11);                // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
+         _11 = (_9,);                     // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
+         _5 = <impl Fn(&T) as Fn<(&T,)>>::call(move _10, move _11) -> [return: bb6, unwind: bb8]; // scope 2 at $DIR/slice_iter.rs:+2:9: +2:13
61                                          // mir::Constant
62                                          // + span: $DIR/slice_iter.rs:29:9: 29:10
63                                          // + literal: Const { ty: for<'a> extern "rust-call" fn(&'a impl Fn(&T), (&T,)) -> <impl Fn(&T) as FnOnce<(&T,)>>::Output {<impl Fn(&T) as Fn<(&T,)>>::call}, val: Value(<ZST>) }
68     }
69 
70     bb5: {
70     bb5: {
-         StorageDead(_7);                 // scope 1 at $DIR/slice_iter.rs:+3:5: +3:6
-         StorageDead(_5);                 // scope 0 at $DIR/slice_iter.rs:+3:5: +3:6
-         StorageDead(_3);                 // scope 0 at $DIR/slice_iter.rs:+3:5: +3:6
+         StorageDead(_6);                 // scope 1 at $DIR/slice_iter.rs:+3:5: +3:6
+         StorageDead(_4);                 // scope 0 at $DIR/slice_iter.rs:+3:5: +3:6
74         drop(_2) -> bb7;                 // scope 0 at $DIR/slice_iter.rs:+4:1: +4:2
76 

77     bb6: {
77     bb6: {
-         StorageDead(_12);                // scope 2 at $DIR/slice_iter.rs:+2:12: +2:13
79         StorageDead(_11);                // scope 2 at $DIR/slice_iter.rs:+2:12: +2:13
-         StorageDead(_7);                 // scope 1 at $DIR/slice_iter.rs:+3:5: +3:6
+         StorageDead(_10);                // scope 2 at $DIR/slice_iter.rs:+2:12: +2:13
+         StorageDead(_6);                 // scope 1 at $DIR/slice_iter.rs:+3:5: +3:6
81         goto -> bb1;                     // scope 1 at $DIR/slice_iter.rs:+1:5: +3:6
83 

94     }
95 
95 
96     bb10: {
-         _3 = move _4;                    // scope 4 at $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL
-         StorageDead(_4);                 // scope 0 at $DIR/slice_iter.rs:+1:25: +1:26
-         StorageLive(_5);                 // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
-         _5 = move _3;                    // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
+         StorageLive(_15);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _15 = Len((*_1));                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _13 = _15 as *const T (Transmute); // scope 10 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         StorageDead(_15);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         goto -> bb12;                    // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+ 
+     bb11: {
+     bb11: {
+         StorageLive(_16);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _16 = Len((*_1));                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _13 = Offset(_12, _16);          // scope 12 at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+         StorageDead(_16);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         goto -> bb12;                    // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+ 
+     bb12: {
+     bb12: {
+         StorageDead(_14);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_17);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_18);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _18 = _12 as *mut T (PtrToPtr);  // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_21);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_22);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_23);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _21 = _18 as *const T (Pointer(MutToConstPointer)); // scope 14 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         _17 = NonNull::<T> { pointer: _21 }; // scope 14 at $SRC_DIR/core/src/ptr/non_null.rs:LL:COL
+         StorageDead(_23);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_22);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_21);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_18);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageLive(_19);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _19 = _13;                       // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         _3 = std::slice::Iter::<'_, T> { ptr: move _17, end: move _19, _marker: const ZeroSized: PhantomData<&T> }; // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: no-location
+                                          // + literal: Const { ty: PhantomData<&T>, val: Value(<ZST>) }
+                                          // adt
+                                          // + user_ty: UserType(1)
+         StorageDead(_19);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_17);                // scope 7 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_13);                // scope 6 at $SRC_DIR/core/src/slice/iter.rs:LL:COL
+         StorageDead(_12);                // scope 3 at $SRC_DIR/core/src/slice/mod.rs:LL:COL
+         StorageLive(_4);                 // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
+         _4 = move _3;                    // scope 0 at $DIR/slice_iter.rs:+1:14: +1:26
101         goto -> bb1;                     // scope 1 at $DIR/slice_iter.rs:+1:5: +3:6
103 }


thread '[mir-opt] tests/mir-opt/pre-codegen/slice_iter.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/slice_iter.forward_loop.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/pre-codegen/slice_iter.rs
