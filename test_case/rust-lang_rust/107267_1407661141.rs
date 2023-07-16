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
.......i................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] tests/mir-opt/const_prop/aggregate.rs stdout ----
4   fn foo(_1: u8) -> () {
5       debug x => _1;                       // in scope 0 at $DIR/aggregate.rs:+0:8: +0:9
6       let mut _0: ();                      // return place in scope 0 at $DIR/aggregate.rs:+0:15: +0:15
-       let _2: i32;                         // in scope 0 at $DIR/aggregate.rs:+1:9: +1:14
-       let mut _3: i32;                     // in scope 0 at $DIR/aggregate.rs:+1:17: +1:25
-       let mut _4: (i32, u8);               // in scope 0 at $DIR/aggregate.rs:+1:17: +1:23
-       let mut _5: u8;                      // in scope 0 at $DIR/aggregate.rs:+1:21: +1:22
-       let mut _7: i32;                     // in scope 0 at $DIR/aggregate.rs:+2:18: +2:26
-       let mut _8: (u8, i32);               // in scope 0 at $DIR/aggregate.rs:+2:18: +2:24
-       let mut _9: u8;                      // in scope 0 at $DIR/aggregate.rs:+2:19: +2:20
+       let _2: i32;                         // in scope 0 at $DIR/aggregate.rs:+2:9: +2:14
+       let mut _3: i32;                     // in scope 0 at $DIR/aggregate.rs:+2:17: +2:25
+       let mut _4: (i32, u8);               // in scope 0 at $DIR/aggregate.rs:+2:17: +2:23
+       let mut _5: u8;                      // in scope 0 at $DIR/aggregate.rs:+2:21: +2:22
+       let mut _7: i32;                     // in scope 0 at $DIR/aggregate.rs:+3:18: +3:26
+       let mut _8: (u8, i32);               // in scope 0 at $DIR/aggregate.rs:+3:18: +3:24
+       let mut _9: u8;                      // in scope 0 at $DIR/aggregate.rs:+3:19: +3:20
14       scope 1 {
-           debug first => _2;               // in scope 1 at $DIR/aggregate.rs:+1:9: +1:14
-           let _6: i32;                     // in scope 1 at $DIR/aggregate.rs:+2:9: +2:15
+           debug first => _2;               // in scope 1 at $DIR/aggregate.rs:+2:9: +2:14
+           let _6: i32;                     // in scope 1 at $DIR/aggregate.rs:+3:9: +3:15
17           scope 2 {
-               debug second => _6;          // in scope 2 at $DIR/aggregate.rs:+2:9: +2:15
+               debug second => _6;          // in scope 2 at $DIR/aggregate.rs:+3:9: +3:15
20       }
21   

22       bb0: {
22       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/aggregate.rs:+1:9: +1:14
-           StorageLive(_3);                 // scope 0 at $DIR/aggregate.rs:+1:17: +1:25
-           StorageLive(_4);                 // scope 0 at $DIR/aggregate.rs:+1:17: +1:23
-           StorageLive(_5);                 // scope 0 at $DIR/aggregate.rs:+1:21: +1:22
-           _5 = _1;                         // scope 0 at $DIR/aggregate.rs:+1:21: +1:22
-           _4 = (const 0_i32, move _5);     // scope 0 at $DIR/aggregate.rs:+1:17: +1:23
-           StorageDead(_5);                 // scope 0 at $DIR/aggregate.rs:+1:22: +1:23
- -         _3 = (_4.0: i32);                // scope 0 at $DIR/aggregate.rs:+1:17: +1:25
- -         _2 = Add(move _3, const 1_i32);  // scope 0 at $DIR/aggregate.rs:+1:17: +1:29
- +         _3 = const 0_i32;                // scope 0 at $DIR/aggregate.rs:+1:17: +1:25
- +         _2 = const 1_i32;                // scope 0 at $DIR/aggregate.rs:+1:17: +1:29
-           StorageDead(_3);                 // scope 0 at $DIR/aggregate.rs:+1:28: +1:29
-           StorageDead(_4);                 // scope 0 at $DIR/aggregate.rs:+1:29: +1:30
-           StorageLive(_6);                 // scope 1 at $DIR/aggregate.rs:+2:9: +2:15
-           StorageLive(_7);                 // scope 1 at $DIR/aggregate.rs:+2:18: +2:26
-           StorageLive(_8);                 // scope 1 at $DIR/aggregate.rs:+2:18: +2:24
-           StorageLive(_9);                 // scope 1 at $DIR/aggregate.rs:+2:19: +2:20
-           _9 = _1;                         // scope 1 at $DIR/aggregate.rs:+2:19: +2:20
-           _8 = (move _9, const 1_i32);     // scope 1 at $DIR/aggregate.rs:+2:18: +2:24
-           StorageDead(_9);                 // scope 1 at $DIR/aggregate.rs:+2:23: +2:24
- -         _7 = (_8.1: i32);                // scope 1 at $DIR/aggregate.rs:+2:18: +2:26
- -         _6 = Add(move _7, const 2_i32);  // scope 1 at $DIR/aggregate.rs:+2:18: +2:30
- +         _7 = const 1_i32;                // scope 1 at $DIR/aggregate.rs:+2:18: +2:26
- +         _6 = const 3_i32;                // scope 1 at $DIR/aggregate.rs:+2:18: +2:30
-           StorageDead(_7);                 // scope 1 at $DIR/aggregate.rs:+2:29: +2:30
-           StorageDead(_8);                 // scope 1 at $DIR/aggregate.rs:+2:30: +2:31
-           _0 = const ();                   // scope 0 at $DIR/aggregate.rs:+0:15: +3:2
-           StorageDead(_6);                 // scope 1 at $DIR/aggregate.rs:+3:1: +3:2
-           StorageDead(_2);                 // scope 0 at $DIR/aggregate.rs:+3:1: +3:2
-           return;                          // scope 0 at $DIR/aggregate.rs:+3:2: +3:2
+           StorageLive(_2);                 // scope 0 at $DIR/aggregate.rs:+2:9: +2:14
+           StorageLive(_3);                 // scope 0 at $DIR/aggregate.rs:+2:17: +2:25
+           StorageLive(_4);                 // scope 0 at $DIR/aggregate.rs:+2:17: +2:23
+           StorageLive(_5);                 // scope 0 at $DIR/aggregate.rs:+2:21: +2:22
+           _5 = _1;                         // scope 0 at $DIR/aggregate.rs:+2:21: +2:22
+           _4 = (const 0_i32, move _5);     // scope 0 at $DIR/aggregate.rs:+2:17: +2:23
+           StorageDead(_5);                 // scope 0 at $DIR/aggregate.rs:+2:22: +2:23
+ -         _3 = (_4.0: i32);                // scope 0 at $DIR/aggregate.rs:+2:17: +2:25
+ -         _2 = Add(move _3, const 1_i32);  // scope 0 at $DIR/aggregate.rs:+2:17: +2:29
+ +         _3 = const 0_i32;                // scope 0 at $DIR/aggregate.rs:+2:17: +2:25
+ +         _2 = const 1_i32;                // scope 0 at $DIR/aggregate.rs:+2:17: +2:29
+           StorageDead(_3);                 // scope 0 at $DIR/aggregate.rs:+2:28: +2:29
+           StorageDead(_4);                 // scope 0 at $DIR/aggregate.rs:+2:29: +2:30
+           StorageLive(_6);                 // scope 1 at $DIR/aggregate.rs:+3:9: +3:15
+           StorageLive(_7);                 // scope 1 at $DIR/aggregate.rs:+3:18: +3:26
+           StorageLive(_8);                 // scope 1 at $DIR/aggregate.rs:+3:18: +3:24
+           StorageLive(_9);                 // scope 1 at $DIR/aggregate.rs:+3:19: +3:20
+           _9 = _1;                         // scope 1 at $DIR/aggregate.rs:+3:19: +3:20
+           _8 = (move _9, const 1_i32);     // scope 1 at $DIR/aggregate.rs:+3:18: +3:24
+           StorageDead(_9);                 // scope 1 at $DIR/aggregate.rs:+3:23: +3:24
+ -         _7 = (_8.1: i32);                // scope 1 at $DIR/aggregate.rs:+3:18: +3:26
+ -         _6 = Add(move _7, const 2_i32);  // scope 1 at $DIR/aggregate.rs:+3:18: +3:30
+ +         _7 = const 1_i32;                // scope 1 at $DIR/aggregate.rs:+3:18: +3:26
+ +         _6 = const 3_i32;                // scope 1 at $DIR/aggregate.rs:+3:18: +3:30
+           StorageDead(_7);                 // scope 1 at $DIR/aggregate.rs:+3:29: +3:30
+           StorageDead(_8);                 // scope 1 at $DIR/aggregate.rs:+3:30: +3:31
+           _0 = const ();                   // scope 0 at $DIR/aggregate.rs:+0:15: +4:2
+           StorageDead(_6);                 // scope 1 at $DIR/aggregate.rs:+4:1: +4:2
+           StorageDead(_2);                 // scope 0 at $DIR/aggregate.rs:+4:1: +4:2
+           return;                          // scope 0 at $DIR/aggregate.rs:+4:2: +4:2
54   }
55   


thread '[mir-opt] tests/mir-opt/const_prop/aggregate.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/aggregate.foo.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/const_prop/aggregate.rs
