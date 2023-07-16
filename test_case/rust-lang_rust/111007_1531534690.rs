plain
test [mir-opt] tests/mir-opt/sroa/structs.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs stdout ----
16 +         let mut _4: usize;               // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
17 +         let mut _5: usize;               // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
18 +         let mut _6: *mut u8;             // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         let mut _7: *const std::vec::Vec<u32>; // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _7: std::boxed::Box<std::vec::Vec<u32>>; // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _8: *const std::vec::Vec<u32>; // in scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
20 +         scope 4 {
22 +     }

65       bb3: {
65       bb3: {
66 -         StorageDead(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
67 -         return;                          // scope 0 at $DIR/inline_into_box_place.rs:+2:2: +2:2
- +         _1 = ShallowInitBox(move _6, std::vec::Vec<u32>); // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         _7 = (((_1.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         (*_7) = move _2;                 // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         StorageLive(_7);                 // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _7 = ShallowInitBox(move _6, std::vec::Vec<u32>); // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _8 = (((_7.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         (*_8) = move _2;                 // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _1 = move _7;                    // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         StorageDead(_7);                 // scope 3 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
71 +         StorageDead(_2);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:48: +1:49
72 +         _0 = const ();                   // scope 0 at $DIR/inline_into_box_place.rs:+0:11: +2:2
73 +         drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2

thread '[mir-opt] tests/mir-opt/inline/inline_into_box_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_into_box_place.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3581:21

---- [mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs stdout ----
9 +         debug self => _2;                // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
10 +         let mut _3: &std::option::Option<T>; // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
11 +         let mut _4: isize;               // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         let _5: T;                       // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
12 +         scope 2 {
- +             debug val => _0;             // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+ +             debug val => _5;             // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
15 +         scope 3 {
15 +         scope 3 {
16 +             scope 5 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL

46 -     bb2 (cleanup): {
47 -         resume;                          // scope 0 at $DIR/unwrap_unchecked.rs:+0:1: +2:2
48 +     bb2: {
- +         _0 = move ((_2 as Some).0: T);   // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         _5 = move ((_2 as Some).0: T);   // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         _0 = move _5;                    // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+ +         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
50 +         StorageDead(_3);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:9: +1:27
51 +         StorageDead(_2);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:26: +1:27
52 +         return;                          // scope 0 at $DIR/unwrap_unchecked.rs:+2:2: +2:2

thread '[mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/unwrap_unchecked.unwrap_unchecked.Inline.diff', src/tools/compiletest/src/runtest.rs:3581:21
---- [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs stdout ----
12 +         debug rhs => _4;                 // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
13 +         let mut _5: u16;                 // in scope 1 at $SRC_DIR/core/src/num/mod.rs:LL:COL
14 +         let mut _6: (u32,);              // in scope 1 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         let mut _7: u32;                 // in scope 1 at $SRC_DIR/core/src/num/mod.rs:LL:COL
16 +         scope 2 {
- +             scope 3 (inlined core::num::<impl u16>::unchecked_shl::conv) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                 debug x => _7;           // in scope 3 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                 let mut _8: std::option::Option<u16>; // in scope 3 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                 let mut _9: std::result::Result<u16, std::num::TryFromIntError>; // in scope 3 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                 scope 4 {
- +                     scope 5 (inlined <u32 as TryInto<u16>>::try_into) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                         debug self => _7; // in scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- +                         scope 6 (inlined convert::num::<impl TryFrom<u32> for u16>::try_from) { // at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- +                             debug u => _7; // in scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                             let mut _10: bool; // in scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                             let mut _11: u32; // in scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                             let mut _12: u16; // in scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                         }
- +                     }
- +                     scope 7 (inlined Result::<u16, TryFromIntError>::ok) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                         debug self => _9; // in scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +                         let mut _13: isize; // in scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +                         let _14: u16;    // in scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +                         scope 8 {
- +                             debug x => _14; // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
- +                         }
- +                     }
- +                     scope 9 (inlined #[track_caller] Option::<u16>::unwrap_unchecked) { // at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +                         debug self => _8; // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         let mut _15: &std::option::Option<u16>; // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         let mut _16: isize; // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         scope 10 {
- +                             debug val => _5; // in scope 10 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         }
- +                         scope 11 {
- +                             scope 13 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL
- +                                 scope 14 {
- +                                     scope 15 (inlined unreachable_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
- +                                     }
- +                                 }
- +                             }
- +                         }
- +                         scope 12 (inlined Option::<u16>::is_some) { // at $SRC_DIR/core/src/option.rs:LL:COL
- +                             debug self => _15; // in scope 12 at $SRC_DIR/core/src/option.rs:LL:COL
- +                         }
- +                     }
- +                 }
- +             }
61 +     }
62   


66           StorageLive(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:21: +1:22
67           _4 = _2;                         // scope 0 at $DIR/unchecked_shifts.rs:+1:21: +1:22
68 -         _0 = core::num::<impl u16>::unchecked_shl(move _3, move _4) -> bb1; // scope 0 at $DIR/unchecked_shifts.rs:+1:5: +1:23
- -                                          // mir::Constant
- -                                          // + span: $DIR/unchecked_shifts.rs:11:7: 11:20
- -                                          // + literal: Const { ty: unsafe fn(u16, u32) -> u16 {core::num::<impl u16>::unchecked_shl}, val: Value(<ZST>) }
72 +         StorageLive(_5);                 // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
73 +         StorageLive(_6);                 // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
74 +         _6 = (_4,);                      // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL

- +         StorageLive(_7);                 // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         _7 = move (_6.0: u32);           // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageLive(_8);                 // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageLive(_10);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageLive(_11);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _11 = const 65535_u32;           // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _10 = Gt(_7, move _11);          // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageDead(_11);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         switchInt(move _10) -> [0: bb3, otherwise: bb2]; // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
+ +         _5 = core::num::<impl u16>::unchecked_shl::conv(move (_6.0: u32)) -> bb1; // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
+                                            // mir::Constant
+ -                                          // + span: $DIR/unchecked_shifts.rs:11:7: 11:20
+ -                                          // + literal: Const { ty: unsafe fn(u16, u32) -> u16 {core::num::<impl u16>::unchecked_shl}, val: Value(<ZST>) }
+ +                                          // + span: $SRC_DIR/core/src/num/mod.rs:LL:COL
+ +                                          // + literal: Const { ty: fn(u32) -> u16 {core::num::<impl u16>::unchecked_shl::conv}, val: Value(<ZST>) }
86   
87       bb1: {


- +         StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           StorageDead(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
-           StorageDead(_3);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
-           return;                          // scope 0 at $DIR/unchecked_shifts.rs:+2:2: +2:2
- +     }
- +     bb2: {
- +     bb2: {
- +         _9 = Result::<u16, TryFromIntError>::Err(const TryFromIntError(())); // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                                          // mir::Constant
- +                                          // + span: no-location
- +                                          // + literal: Const { ty: TryFromIntError, val: Value(<ZST>) }
- +         goto -> bb4;                     // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +     }
- +     bb3: {
- +     bb3: {
- +         StorageLive(_12);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _12 = _7 as u16 (IntToInt);      // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _9 = Result::<u16, TryFromIntError>::Ok(move _12); // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageDead(_12);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         goto -> bb4;                     // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +     }
- +     bb4: {
- +     bb4: {
- +         StorageDead(_10);                // scope 6 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageLive(_14);                // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         _13 = discriminant(_9);          // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +         switchInt(move _13) -> [0: bb7, 1: bb5, otherwise: bb6]; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb5: {
- +     bb5: {
- +         _8 = Option::<u16>::None;        // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +         goto -> bb8;                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb6: {
- +     bb6: {
- +         unreachable;                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb7: {
- +     bb7: {
- +         _14 = move ((_9 as Ok).0: u16);  // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +         _8 = Option::<u16>::Some(move _14); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
- +         goto -> bb8;                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb8: {
- +     bb8: {
- +         StorageDead(_14);                // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         _16 = discriminant(_8);          // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +         switchInt(move _16) -> [1: bb9, otherwise: bb6]; // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +     }
- +     bb9: {
- +     bb9: {
- +         _5 = move ((_8 as Some).0: u16); // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +         StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageDead(_8);                 // scope 4 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         StorageDead(_7);                 // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
145 +         StorageDead(_6);                 // scope 2 at $SRC_DIR/core/src/num/mod.rs:LL:COL
- +         _0 = unchecked_shl::<u16>(_3, move _5) -> [return: bb1, unwind unreachable]; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _0 = unchecked_shl::<u16>(_3, move _5) -> [return: bb2, unwind unreachable]; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
147 +                                          // mir::Constant
148 +                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
149 +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u16, u16) -> u16 {unchecked_shl::<u16>}, val: Value(<ZST>) }
+ +     }
+ + 
+ +     bb2: {
+ +     bb2: {
+ +         StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+           StorageDead(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
+           StorageDead(_3);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
+           return;                          // scope 0 at $DIR/unchecked_shifts.rs:+2:2: +2:2
151   }
152   


thread '[mir-opt] tests/mir-opt/inline/unchecked_shifts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/unchecked_shifts.unchecked_shl_unsigned_smaller.Inline.diff', src/tools/compiletest/src/runtest.rs:3581:21

failures:
    [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs
    [mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs
