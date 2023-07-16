plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:8c19d39c6d9d7e831f6e393b2a871216393a5761)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..ii.......i................................
failures:

---- [mir-opt] tests/mir-opt/intrinsic_asserts.rs stdout ----
8       let _3: ();                          // in scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
10       bb0: {
10       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
12 -         _1 = assert_inhabited::<()>() -> bb1; // scope 0 at $DIR/intrinsic_asserts.rs:+1:5: +1:47
13 -                                          // mir::Constant
14 -                                          // + span: $DIR/intrinsic_asserts.rs:7:5: 7:45
17       }
18   
19       bb1: {
19       bb1: {
-           StorageDead(_1);                 // scope 0 at $DIR/intrinsic_asserts.rs:+1:47: +1:48
-           StorageLive(_2);                 // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+1:47: +1:48
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
22 -         _2 = assert_zero_valid::<u8>() -> bb2; // scope 0 at $DIR/intrinsic_asserts.rs:+2:5: +2:48
23 -                                          // mir::Constant
24 -                                          // + span: $DIR/intrinsic_asserts.rs:8:5: 8:46
27       }
28   
29       bb2: {
29       bb2: {
-           StorageDead(_2);                 // scope 0 at $DIR/intrinsic_asserts.rs:+2:48: +2:49
-           StorageLive(_3);                 // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+2:48: +2:49
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
32 -         _3 = assert_mem_uninitialized_valid::<u8>() -> bb3; // scope 0 at $DIR/intrinsic_asserts.rs:+3:5: +3:61
33 -                                          // mir::Constant
34 -                                          // + span: $DIR/intrinsic_asserts.rs:9:5: 9:59
37       }
38   
39       bb3: {
39       bb3: {
-           StorageDead(_3);                 // scope 0 at $DIR/intrinsic_asserts.rs:+3:61: +3:62
+           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+3:61: +3:62
41           nop;                             // scope 0 at $DIR/intrinsic_asserts.rs:+0:20: +4:2
42           return;                          // scope 0 at $DIR/intrinsic_asserts.rs:+4:2: +4:2


thread '[mir-opt] tests/mir-opt/intrinsic_asserts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/intrinsic_asserts.removable.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/intrinsic_asserts.rs
