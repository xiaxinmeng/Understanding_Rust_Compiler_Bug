plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 228 tests
...................................................................i.................... 88/228
.......................................................i...F............................ 176/228
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs stdout ----
10       let mut _5: std::boxed::Box<std::vec::Vec<u32>>; // in scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
11       let mut _6: ();                      // in scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
12       let mut _7: *const std::vec::Vec<u32>; // in scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- +     let mut _23: &mut std::vec::Vec<u32>; // in scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +     let mut _24: std::vec::Vec<u32>;     // in scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +     let mut _36: &mut std::vec::Vec<u32>; // in scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +     let mut _37: std::vec::Vec<u32>;     // in scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
15       scope 1 {
16           debug _x => _1;                  // in scope 1 at $DIR/inline_into_box_place.rs:+1:9: +1:11


45 +                     let mut _21: std::ptr::Alignment; // in scope 7 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
46 +                     let mut _22: usize;  // in scope 7 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
47 +                     scope 8 {
+ +                         scope 9 (inlined std::ptr::Alignment::new_unchecked) { // at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+ +                             debug align => _22; // in scope 9 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                             let mut _23: (usize,); // in scope 9 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                             let mut _24: usize; // in scope 9 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                             let mut _25: usize; // in scope 9 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                             let mut _26: usize; // in scope 9 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                             scope 10 {
+ +                                 scope 12 (inlined std::ptr::Alignment::new_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                     debug align => _26; // in scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                     let mut _27: bool; // in scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                     let mut _28: bool; // in scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                                     let mut _29: usize; // in scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                                     let _30: !; // in scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                     scope 13 (inlined core::num::<impl usize>::is_power_of_two) { // at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                                         debug self => _29; // in scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                         let mut _31: u32; // in scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                         let mut _32: usize; // in scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                         scope 14 (inlined core::num::<impl usize>::count_ones) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                             debug self => _32; // in scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                             let mut _33: u64; // in scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                             let mut _34: u64; // in scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                             let mut _35: usize; // in scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                         }
+ +                                     }
+ +                                 }
+ +                             }
+ +                             scope 11 {
+ +                             }
+ +                         }
49 +                 }
50 +             }

51 +         }
51 +         }
52       }
- +     scope 9 (inlined Vec::<u32>::new) {  // at $DIR/inline_into_box_place.rs:8:33: 8:43
- +         let mut _25: alloc::raw_vec::RawVec<u32>; // in scope 9 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +     scope 15 (inlined Vec::<u32>::new) { // at $DIR/inline_into_box_place.rs:8:33: 8:43
+ +         let mut _38: alloc::raw_vec::RawVec<u32>; // in scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
56   
57       bb0: {


70 +         StorageLive(_21);                // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
71 +         StorageLive(_22);                // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
72 +         _22 = _10;                       // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
- +         _21 = std::ptr::Alignment::new_unchecked(move _22) -> bb8; // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+ +         StorageLive(_23);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageLive(_24);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         _24 = _22;                       // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         _23 = (move _24,);               // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageDead(_24);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageLive(_26);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         _26 = move (_23.0: usize);       // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageLive(_27);                // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageLive(_28);                // scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         StorageLive(_29);                // scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         _29 = _26;                       // scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         StorageLive(_31);                // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageLive(_32);                // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _32 = _29;                       // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageLive(_33);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageLive(_34);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageLive(_35);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _35 = _32;                       // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _34 = move _35 as u64 (IntToInt); // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_35);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _33 = ctpop::<u64>(move _34) -> bb11; // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
74                                            // mir::Constant
75 -                                          // + span: $DIR/inline_into_box_place.rs:8:29: 8:43
76 -                                          // + literal: Const { ty: unsafe fn(usize, usize) -> *mut u8 {alloc::alloc::exchange_malloc}, val: Value(<ZST>) }

- +                                          // + span: $SRC_DIR/core/src/alloc/layout.rs:LL:COL
- +                                          // + literal: Const { ty: unsafe fn(usize) -> std::ptr::Alignment {std::ptr::Alignment::new_unchecked}, val: Value(<ZST>) }
+ +                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +                                          // + literal: Const { ty: extern "rust-intrinsic" fn(u64) -> u64 {ctpop::<u64>}, val: Value(<ZST>) }
80   
81       bb1: {


- -         StorageLive(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- -         _5 = ShallowInitBox(move _4, std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- -         _7 = (((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- -         (*_7) = Vec::<u32>::new() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- -                                          // mir::Constant
- -                                          // + span: $DIR/inline_into_box_place.rs:8:33: 8:41
- -                                          // + user_ty: UserType(1)
- -                                          // + literal: Const { ty: fn() -> Vec<u32> {Vec::<u32>::new}, val: Value(<ZST>) }
90 +         StorageDead(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
91 +         return;                          // scope 0 at $DIR/inline_into_box_place.rs:+2:2: +2:2
-   
- -     bb2: {
- -     bb2: {
- -         _1 = move _5;                    // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- -         StorageDead(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
- -         _0 = const ();                   // scope 0 at $DIR/inline_into_box_place.rs:+0:11: +2:2
- -         drop(_1) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
+ +     }
+ + 
99 +     bb2 (cleanup): {
100 +         resume;                          // scope 0 at $DIR/inline_into_box_place.rs:+0:1: +2:2
-   
-       bb3: {
-       bb3: {
- -         StorageDead(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
- -         return;                          // scope 0 at $DIR/inline_into_box_place.rs:+2:2: +2:2
+ +     }
+ +     bb3: {
+ +     bb3: {
106 +         StorageDead(_13);                // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
107 +         StorageDead(_12);                // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
108 +         _14 = discriminant(_11);         // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL

109 +         switchInt(move _14) -> [0: bb6, 1: bb4, otherwise: bb5]; // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
-   
-   
- -     bb4 (cleanup): {
- -         resume;                          // scope 0 at $DIR/inline_into_box_place.rs:+0:1: +2:2
+ +     }
114 +     bb4: {
114 +     bb4: {
115 +         StorageLive(_17);                // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
116 +         StorageLive(_18);                // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
119 +                                          // mir::Constant
119 +                                          // mir::Constant
120 +                                          // + span: $SRC_DIR/alloc/src/alloc.rs:LL:COL
121 +                                          // + literal: Const { ty: fn(Layout) -> ! {handle_alloc_error}, val: Value(<ZST>) }
-   
-   
- -     bb5 (cleanup): {
- -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> bb4; // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
+ +     }
126 +     bb5: {
126 +     bb5: {
127 +         unreachable;                     // scope 4 at $SRC_DIR/alloc/src/alloc.rs:LL:COL


133 +         StorageLive(_16);                // scope 6 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
134 +         _16 = _15;                       // scope 6 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
135 +         _4 = NonNull::<[u8]>::as_mut_ptr(move _16) -> bb7; // scope 6 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
-                                            // mir::Constant
- -                                          // + span: $DIR/inline_into_box_place.rs:8:42: 8:43
- -                                          // + literal: Const { ty: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}, val: Value(<ZST>) }
+ +                                          // mir::Constant
139 +                                          // + span: $SRC_DIR/alloc/src/alloc.rs:LL:COL
140 +                                          // + literal: Const { ty: fn(NonNull<[u8]>) -> *mut u8 {NonNull::<[u8]>::as_mut_ptr}, val: Value(<ZST>) }


146 +         StorageDead(_8);                 // scope 3 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
147 +         StorageDead(_11);                // scope 3 at $SRC_DIR/alloc/src/alloc.rs:LL:COL
148 +         StorageDead(_19);                // scope 2 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- +         StorageLive(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- +         _5 = ShallowInitBox(move _4, std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- +         _7 = (((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         StorageLive(_23);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         _23 = &mut (*_7);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         StorageLive(_24);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         StorageLive(_25);                // scope 9 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         _25 = const _;                   // scope 9 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +                                          // mir::Constant
+           StorageLive(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
+           _5 = ShallowInitBox(move _4, std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
+           _7 = (((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: std::ptr::NonNull<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>); // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ -         (*_7) = Vec::<u32>::new() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         StorageLive(_36);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         _36 = &mut (*_7);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         StorageLive(_37);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         StorageLive(_38);                // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         _38 = const _;                   // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+                                            // mir::Constant
+ -                                          // + span: $DIR/inline_into_box_place.rs:8:33: 8:41
+ -                                          // + user_ty: UserType(1)
+ -                                          // + literal: Const { ty: fn() -> Vec<u32> {Vec::<u32>::new}, val: Value(<ZST>) }
+ -     }
+ -     bb2: {
+ -     bb2: {
158 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
159 +                                          // + user_ty: UserType(0)
160 +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T>::NEW, [u32], None) }

- +         _24 = Vec::<u32> { buf: move _25, len: const 0_usize }; // scope 9 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         StorageDead(_25);                // scope 9 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         (*_23) = move _24;               // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         StorageDead(_24);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         StorageDead(_23);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
- +         _1 = move _5;                    // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
- +         StorageDead(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
- +         _0 = const ();                   // scope 0 at $DIR/inline_into_box_place.rs:+0:11: +2:2
+ +         _37 = Vec::<u32> { buf: move _38, len: const 0_usize }; // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         StorageDead(_38);                // scope 15 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         (*_36) = move _37;               // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         StorageDead(_37);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+ +         StorageDead(_36);                // scope 0 at $DIR/inline_into_box_place.rs:+1:33: +1:43
+           _1 = move _5;                    // scope 0 at $DIR/inline_into_box_place.rs:+1:29: +1:43
+           StorageDead(_5);                 // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
+           _0 = const ();                   // scope 0 at $DIR/inline_into_box_place.rs:+0:11: +2:2
+ -         drop(_1) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
169 +         drop(_1) -> [return: bb1, unwind: bb2]; // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
- +     }
+       }
+   
+ -     bb3: {
+ -     bb3: {
+ -         StorageDead(_1);                 // scope 0 at $DIR/inline_into_box_place.rs:+2:1: +2:2
+ -         return;                          // scope 0 at $DIR/inline_into_box_place.rs:+2:2: +2:2
172 +     bb8: {
+ +         StorageDead(_25);                // scope 11 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
173 +         StorageDead(_22);                // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
174 +         _8 = Layout { size: move _20, align: move _21 }; // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
175 +         StorageDead(_21);                // scope 8 at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
189 +                                          // mir::Constant
189 +                                          // mir::Constant
190 +                                          // + span: $SRC_DIR/alloc/src/alloc.rs:LL:COL
191 +                                          // + literal: Const { ty: for<'a> fn(&'a std::alloc::Global, Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {<std::alloc::Global as Allocator>::allocate}, val: Value(<ZST>) }
+   
+   
+ -     bb4 (cleanup): {
+ -         resume;                          // scope 0 at $DIR/inline_into_box_place.rs:+0:1: +2:2
+ +     bb9: {
+ +         StorageLive(_30);                // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         _30 = core::panicking::panic_nounwind(const "unsafe precondition(s) violated: Alignment::new_unchecked requires a power of two"); // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                          // mir::Constant
+ +                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic_nounwind}, val: Value(<ZST>) }
+ +                                          // mir::Constant
+ +                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+   
+   
+ -     bb5 (cleanup): {
+ -         _6 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_5.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_5.1: std::alloc::Global)) -> bb4; // scope 0 at $DIR/inline_into_box_place.rs:+1:42: +1:43
+ +     bb10: {
+ +         StorageDead(_27);                // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageDead(_26);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageDead(_23);                // scope 10 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageLive(_25);                // scope 11 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         _25 = _22;                       // scope 11 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         _21 = transmute::<usize, std::ptr::Alignment>(move _25) -> bb8; // scope 11 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+                                            // mir::Constant
+ -                                          // + span: $DIR/inline_into_box_place.rs:8:42: 8:43
+ -                                          // + literal: Const { ty: unsafe fn(Unique<Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>}, val: Value(<ZST>) }
+ +                                          // + span: $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(usize) -> std::ptr::Alignment {transmute::<usize, std::ptr::Alignment>}, val: Value(<ZST>) }
+ +     }
+ +     bb11: {
+ +     bb11: {
+ +         StorageDead(_34);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _31 = move _33 as u32 (IntToInt); // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_33);                // scope 14 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_32);                // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         _28 = Eq(move _31, const 1_u32); // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_31);                // scope 13 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ +         StorageDead(_29);                // scope 12 at $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ +         _27 = Not(move _28);             // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         StorageDead(_28);                // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+ +         switchInt(move _27) -> [0: bb10, otherwise: bb9]; // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
193   }
194   


thread '[mir-opt] tests/mir-opt/inline/inline_into_box_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_into_box_place.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs
