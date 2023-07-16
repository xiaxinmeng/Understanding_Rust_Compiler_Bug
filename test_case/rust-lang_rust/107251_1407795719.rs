plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
failures:

---- [mir-opt] tests/mir-opt/const_prop/discriminant.rs stdout ----
32   
33       bb2: {
34           _2 = const 42_i32;               // scope 2 at $DIR/discriminant.rs:+1:47: +1:49
+           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:+1:50: +1:51
35           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:+1:13: +1:64
37   

38       bb3: {
38       bb3: {
+           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:+1:50: +1:51
39           _2 = const 10_i32;               // scope 0 at $DIR/discriminant.rs:+1:59: +1:61
40           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:+1:13: +1:64

43       bb4: {
43       bb4: {
44           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/discriminant.rs:+1:13: +1:68
45           StorageDead(_2);                 // scope 0 at $DIR/discriminant.rs:+1:67: +1:68
-           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:+1:68: +1:69
47           _0 = const ();                   // scope 0 at $DIR/discriminant.rs:+0:11: +2:2
48           StorageDead(_1);                 // scope 0 at $DIR/discriminant.rs:+2:1: +2:2
49           return;                          // scope 0 at $DIR/discriminant.rs:+2:2: +2:2

thread '[mir-opt] tests/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/const_prop/discriminant.rs
