plain
test [mir-opt] tests/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs stdout ----
11 +         debug self => _3;                // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
12 +         debug rhs => _4;                 // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
13 +         let mut _5: u16;                 // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         let mut _6: std::option::Option<u16>; // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         let mut _7: std::result::Result<u16, std::num::TryFromIntError>; // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
16 +         scope 2 {
- +             scope 3 (inlined <u32 as TryInto<u16>>::try_into) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +                 debug self => _4;        // in scope 3 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- +                 scope 4 (inlined convert::num::<impl TryFrom<u32> for u16>::try_from) { // at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- +                     debug u => _4;       // in scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                     let mut _8: bool;    // in scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                     let mut _9: u32;     // in scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                     let mut _10: u16;    // in scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                 }
- +             }
- +             scope 5 (inlined Result::<u16, TryFromIntError>::ok) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +                 debug self => _7;        // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +                 let mut _11: isize;      // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +                 let _12: u16;            // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +                 scope 6 {
- +                     debug x => _12;      // in scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
- +                 }
- +                 scope 7 {
- +                     scope 8 {
- +                         debug x => const TryFromIntError(()); // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
- +                     }
- +                 }
- +             }
- +             scope 9 (inlined #[track_caller] Option::<u16>::unwrap_unchecked) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +                 debug self => _6;        // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                 let mut _13: &std::option::Option<u16>; // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                 let mut _14: isize;      // in scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +                 scope 10 {
- +                     debug val => _5;     // in scope 10 at $SRC_DIR/core/src/option.rs:LL:COL
- +                 }
- +                 scope 11 {
- +                     scope 13 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL
- +                         scope 14 {
- +                             scope 15 (inlined unreachable_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
- +                             }
- +                         }
- +                     }
- +                 }
- +                 scope 12 (inlined Option::<u16>::is_some) { // at $SRC_DIR/core/src/option.rs:LL:COL
- +                     debug self => _13;   // in scope 12 at $SRC_DIR/core/src/option.rs:LL:COL
- +                 }
- +             }
59 +     }
60   


64           StorageLive(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:21: +1:22
65           _4 = _2;                         // scope 0 at $DIR/unchecked_shifts.rs:+1:21: +1:22
66 -         _0 = core::num::<impl u16>::unchecked_shl(move _3, move _4) -> bb1; // scope 0 at $DIR/unchecked_shifts.rs:+1:5: +1:23
- -                                          // mir::Constant
+ +         StorageLive(_5);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:7: +1:23
+ +         _5 = _4 as u16 (IntToInt);       // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _0 = unchecked_shl::<u16>(_3, _5) -> [return: bb1, unwind unreachable]; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+                                            // mir::Constant
68 -                                          // + span: $DIR/unchecked_shifts.rs:11:7: 11:20
69 -                                          // + literal: Const { ty: unsafe fn(u16, u32) -> u16 {core::num::<impl u16>::unchecked_shl}, val: Value(<ZST>) }
- +         StorageLive(_5);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageLive(_6);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageLive(_7);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageLive(_8);                 // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _9 = const 65535_u32;            // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _8 = Gt(_4, move _9);            // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         switchInt(move _8) -> [0: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
+ +                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u16, u16) -> u16 {unchecked_shl::<u16>}, val: Value(<ZST>) }
80   
81       bb1: {


- +         StorageDead(_12);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageDead(_7);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageLive(_13);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         _14 = discriminant(_6);          // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +         switchInt(move _14) -> [1: bb9, otherwise: bb7]; // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +     }
- +     bb2: {
- +     bb2: {
- +         StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_5);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:7: +1:23
91           StorageDead(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
92           StorageDead(_3);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23
93           return;                          // scope 0 at $DIR/unchecked_shifts.rs:+2:2: +2:2
- +     }
- + 
- +     bb3: {
- +     bb3: {
- +         _7 = Result::<u16, TryFromIntError>::Err(const TryFromIntError(())); // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +                                          // mir::Constant
- +                                          // + span: no-location
- +                                          // + literal: Const { ty: TryFromIntError, val: Value(<ZST>) }
- +         goto -> bb5;                     // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +     }
- +     bb4: {
- +     bb4: {
- +         StorageLive(_10);                // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _10 = _4 as u16 (IntToInt);      // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         _7 = Result::<u16, TryFromIntError>::Ok(move _10); // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageDead(_10);                // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         goto -> bb5;                     // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +     }
- +     bb5: {
- +     bb5: {
- +         StorageDead(_8);                 // scope 4 at $SRC_DIR/core/src/convert/num.rs:LL:COL
- +         StorageLive(_12);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         _11 = discriminant(_7);          // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +         switchInt(move _11) -> [0: bb8, 1: bb6, otherwise: bb7]; // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb6: {
- +     bb6: {
- +         _6 = Option::<u16>::None;        // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
- +         goto -> bb1;                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb7: {
- +     bb7: {
- +         unreachable;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb8: {
- +     bb8: {
- +         _12 = move ((_7 as Ok).0: u16);  // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +         _6 = Option::<u16>::Some(move _12); // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
- +         goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
- +     }
- +     bb9: {
- +     bb9: {
- +         _5 = move ((_6 as Some).0: u16); // scope 9 at $SRC_DIR/core/src/option.rs:LL:COL
- +         StorageDead(_13);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         StorageDead(_6);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +         _0 = unchecked_shl::<u16>(_3, move _5) -> [return: bb2, unwind unreachable]; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +                                          // mir::Constant
- +                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u16, u16) -> u16 {unchecked_shl::<u16>}, val: Value(<ZST>) }
143   }
144   


thread '[mir-opt] tests/mir-opt/inline/unchecked_shifts.rs' panicked at 'Actual MIR output differs from expected MIR output /Users/runner/work/rust/rust/tests/mir-opt/inline/unchecked_shifts.unchecked_shl_unsigned_smaller.Inline.diff', src/tools/compiletest/src/runtest.rs:3524:21


failures:
    [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs
