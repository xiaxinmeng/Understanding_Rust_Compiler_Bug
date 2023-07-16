plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
...............ii.......i..................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs stdout ----
9           debug _x => _1;                  // in scope 1 at $DIR/inline_into_box_place.rs:+1:9: +1:11
10       }
11 +     scope 2 (inlined Vec::<u32>::new) {  // at $DIR/inline_into_box_place.rs:7:38: 7:48
- +         scope 3 {
- +             scope 4 (inlined Vec::<u32>::new_co) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +                 let mut _3: alloc::raw_vec::RawVec<u32>; // in scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +             }
+ +         scope 3 (inlined Vec::<u32>::new_co) { // at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +             let mut _3: alloc::raw_vec::RawVec<u32>; // in scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
17 +     }
17 +     }
- +     scope 5 (inlined Box::<Vec<u32>>::new) { // at $DIR/inline_into_box_place.rs:7:29: 7:49
- +         debug x => _2;                   // in scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         let mut _4: usize;               // in scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         let mut _5: usize;               // in scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         let mut _6: *mut u8;             // in scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         let mut _7: *const std::vec::Vec<u32>; // in scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         scope 6 {
+ +     scope 4 (inlined Box::<Vec<u32>>::new) { // at $DIR/inline_into_box_place.rs:7:29: 7:49
+ +         debug x => _2;                   // in scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _4: usize;               // in scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _5: usize;               // in scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _6: *mut u8;             // in scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         let mut _7: *const std::vec::Vec<u32>; // in scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         scope 5 {
26 +     }
27   


29           StorageLive(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:9: +1:11
30           StorageLive(_2);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:38: +1:48
31 -         _2 = Vec::<u32>::new() -> bb1;   // scope 0 at $DIR/inline_into_box_place.rs:+1:38: +1:48
- +         StorageLive(_3);                 // scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         _3 = const _;                    // scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         StorageLive(_3);                 // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         _3 = const _;                    // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
34                                            // mir::Constant
35 -                                          // + span: $DIR/inline_into_box_place.rs:7:38: 7:46
36 -                                          // + user_ty: UserType(2)

38 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
39 +                                          // + user_ty: UserType(0)
40 +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T, std::alloc::Global, CO_ALLOC_PREF>::NEW, [u32, Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }], None) }
- +         _2 = Vec::<u32> { buf: move _3, len: const 0_usize }; // scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         StorageDead(_3);                 // scope 4 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         _4 = SizeOf(std::vec::Vec<u32>); // scope 6 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         _5 = AlignOf(std::vec::Vec<u32>); // scope 6 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         _6 = alloc::alloc::exchange_malloc(move _4, move _5) -> [return: bb3, unwind: bb4]; // scope 6 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _2 = Vec::<u32> { buf: move _3, len: const 0_usize }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         StorageDead(_3);                 // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         _4 = SizeOf(std::vec::Vec<u32>); // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _5 = AlignOf(std::vec::Vec<u32>); // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _6 = alloc::alloc::exchange_malloc(move _4, move _5) -> [return: bb3, unwind: bb4]; // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
46 +                                          // mir::Constant
47 +                                          // + span: $SRC_DIR/alloc/src/boxed.rs:LL:COL
48 +                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(<ZST>) }
69       bb3: {
69       bb3: {
70 -         StorageDead(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
71 -         return;                          // scope 0 at $DIR/inline_into_box_place.rs:+2:2: +2:2
- +         _1 = ShallowInitBox(move _6, std::vec::Vec<u32>); // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         _7 = (((_1.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
- +         (*_7) = move _2;                 // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _1 = ShallowInitBox(move _6, std::vec::Vec<u32>); // scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         _7 = (((_1.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         (*_7) = move _2;                 // scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
75 +         StorageDead(_2);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:48: +1:49
76 +         _0 = const ();                   // scope 0 at $DIR/inline_into_box_place.rs:+0:11: +2:2
77 +         drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
79   
79   
80       bb4 (cleanup): {
81 -         resume;                          // scope 0 at $DIR/inline_into_box_place.rs:+0:1: +2:2
- +         drop(_2) -> bb2;                 // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+ +         drop(_2) -> bb2;                 // scope 4 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
84   }
85   


thread '[mir-opt] tests/mir-opt/inline/inline_into_box_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_into_box_place.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3492:21


failures:
    [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs
