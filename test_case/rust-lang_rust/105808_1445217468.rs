plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
...............ii......i.................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] tests/mir-opt/casts.rs stdout ----
3 fn roundtrip(_1: *const u8) -> *const u8 {
4     debug x => _1;                       // in scope 0 at $DIR/casts.rs:+0:18: +0:19
5     let mut _0: *const u8;               // return place in scope 0 at $DIR/casts.rs:+0:35: +0:44
-     let mut _2: *mut u8;                 // in scope 0 at $DIR/casts.rs:+1:5: +1:17
8     bb0: {
8     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/casts.rs:+1:5: +1:17
-         _2 = _1 as *mut u8 (PtrToPtr);   // scope 0 at $DIR/casts.rs:+1:5: +1:17
-         _0 = move _2 as *const u8 (Pointer(MutToConstPointer)); // scope 0 at $DIR/casts.rs:+1:5: +1:17
-         StorageDead(_2);                 // scope 0 at $DIR/casts.rs:+1:16: +1:17
+         _0 = _1 as *const u8 (PtrToPtr); // scope 0 at $DIR/casts.rs:+1:5: +1:17
13         return;                          // scope 0 at $DIR/casts.rs:+2:2: +2:2
15 }


thread '[mir-opt] tests/mir-opt/casts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/casts.roundtrip.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3481:21


failures:
    [mir-opt] tests/mir-opt/casts.rs
