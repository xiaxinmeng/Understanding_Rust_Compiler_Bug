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
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
...............ii.......i..................................
failures:

---- [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs stdout ----
35 -                                          // + literal: Const { ty: fn() -> Vec<u32> {Vec::<u32>::new}, val: Value(<ZST>) }
36 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
37 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T, std::alloc::Global, CO_ALLOC_PREF>::NEW, [u32, Const { ty: usize, kind: Value(Leaf(0x0000000000000000)) }], None) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T, std::alloc::Global, CO_ALLOC_PREF>::NEW, [u32, Const { ty: usize, kind: Value(Leaf(0x00000000)) }], None) }
39 +         _2 = Vec::<u32> { buf: move _3, len: const 0_usize }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
40 +         StorageDead(_3);                 // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
41 +         _4 = SizeOf(std::vec::Vec<u32>); // scope 5 at $SRC_DIR/alloc/src/boxed.rs:LL:COL

thread '[mir-opt] tests/mir-opt/inline/inline_into_box_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_into_box_place.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3492:21


failures:
    [mir-opt] tests/mir-opt/inline/inline_into_box_place.rs
