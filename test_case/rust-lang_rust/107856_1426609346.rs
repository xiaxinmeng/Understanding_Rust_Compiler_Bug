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
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........ii.......i.................................
failures:

---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
28           StorageDead(_3);                 // scope 0 at $DIR/slice_len.rs:+1:18: +1:19
29           StorageLive(_6);                 // scope 0 at $DIR/slice_len.rs:+1:31: +1:32
30           _6 = const 1_usize;              // scope 0 at $DIR/slice_len.rs:+1:31: +1:32
- -         _7 = Len((*_2));                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+           _7 = Len((*_2));                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
32 -         _8 = Lt(_6, _7);                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
33 -         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _7 = const 3_usize;              // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _8 = const true;                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 3_usize, const 1_usize) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+ +         _8 = Lt(const 1_usize, _7);      // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+ +         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, const 1_usize) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
38   
39       bb1: {


- -         _1 = (*_2)[_6];                  // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _1 = const 2_u32;                // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+           _1 = (*_2)[_6];                  // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
42           StorageDead(_6);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34
43           StorageDead(_4);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34
44           StorageDead(_2);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34

thread '[mir-opt] tests/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/slice_len.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/const_prop/slice_len.rs
