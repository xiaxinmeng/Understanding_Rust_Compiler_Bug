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
 finished in 0.619 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 230 tests
...............................................F.................i...................... 88/230
......................................F.................i............................... 176/230
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii.......i......F.............F....F.F...F.

---- [mir-opt] tests/mir-opt/const_prop/discriminant.rs stdout ----
32   
33       bb2: {
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

thread '[mir-opt] tests/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3483:21

---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
100           StorageDead(_14);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
101           StorageDead(_13);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
102           StorageDead(_11);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_7);                 // scope 2 at $DIR/funky_arms.rs:+16:5: +16:6
103           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
105   

106       bb8: {
106       bb8: {
+           StorageDead(_7);                 // scope 2 at $DIR/funky_arms.rs:+16:5: +16:6
107           StorageLive(_18);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
108           _18 = &mut (*_1);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
109           StorageLive(_20);                // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
123       bb10: {
123       bb10: {
124           StorageDead(_6);                 // scope 1 at $DIR/funky_arms.rs:+19:1: +19:2
125           StorageDead(_4);                 // scope 0 at $DIR/funky_arms.rs:+19:1: +19:2
-           StorageDead(_7);                 // scope 0 at $DIR/funky_arms.rs:+19:1: +19:2
127           return;                          // scope 0 at $DIR/funky_arms.rs:+19:2: +19:2
129   }


thread '[mir-opt] tests/mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs stdout ----
---- [mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs stdout ----
47 -         StorageDead(_8);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:19: +2:20
48 -         StorageDead(_7);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+4:9: +4:10
49           StorageDead(_6);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
-           goto -> bb3;                     // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+1:5: +5:6
+           drop(_1) -> bb5;                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
52   
53       bb3: {


-           drop(_1) -> bb4;                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:1: +6:2
+           drop(_1) -> bb4;                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
56   
57       bb4: {


-           StorageDead(_1);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:1: +6:2
+           StorageDead(_1);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
+           goto -> bb6;                     // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+1:5: +5:6
+   
+       bb5: {
+       bb5: {
+           StorageDead(_1);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
+           goto -> bb6;                     // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+1:5: +5:6
+   
+       bb6: {
+       bb6: {
59           return;                          // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:2: +6:2
61   }


thread '[mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals-final.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/unreachable.rs stdout ----
62 -     }
63 - 
64 -     bb6: {
64 -     bb6: {
+           StorageDead(_1);                 // scope 0 at $DIR/unreachable.rs:+11:5: +11:6
65           _0 = const ();                   // scope 0 at $DIR/unreachable.rs:+11:6: +11:6
-           StorageDead(_1);                 // scope 0 at $DIR/unreachable.rs:+12:1: +12:2
67           return;                          // scope 0 at $DIR/unreachable.rs:+12:2: +12:2
69   }


thread '[mir-opt] tests/mir-opt/unreachable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/unreachable.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/unreachable_diverging.rs stdout ----
62       }
63   
64       bb6: {
64       bb6: {
+           StorageDead(_2);                 // scope 1 at $DIR/unreachable_diverging.rs:+7:5: +7:6
65           _0 = const ();                   // scope 1 at $DIR/unreachable_diverging.rs:+7:6: +7:6
66           StorageDead(_1);                 // scope 0 at $DIR/unreachable_diverging.rs:+8:1: +8:2
-           StorageDead(_2);                 // scope 0 at $DIR/unreachable_diverging.rs:+8:1: +8:2
68           return;                          // scope 0 at $DIR/unreachable_diverging.rs:+8:2: +8:2
70   }


thread '[mir-opt] tests/mir-opt/unreachable_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/unreachable_diverging.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/while_let_loops.rs stdout ----
30   
31       bb2: {
31       bb2: {
32           _1 = const 1_i32;                // scope 2 at $DIR/while_let_loops.rs:+3:9: +3:15
-           goto -> bb4;                     // scope 2 at $DIR/while_let_loops.rs:+4:9: +4:14
+           StorageDead(_2);                 // scope 1 at $DIR/while_let_loops.rs:+5:5: +5:6
+           goto -> bb4;                     // scope 1 at no-location
35   
36       bb3: {


+           StorageDead(_2);                 // scope 1 at $DIR/while_let_loops.rs:+5:5: +5:6
37           goto -> bb4;                     // scope 1 at no-location
39   

40       bb4: {
40       bb4: {
-           StorageDead(_2);                 // scope 1 at $DIR/while_let_loops.rs:+5:5: +5:6
42           StorageDead(_1);                 // scope 0 at $DIR/while_let_loops.rs:+6:1: +6:2
43           return;                          // scope 0 at $DIR/while_let_loops.rs:+6:2: +6:2


thread '[mir-opt] tests/mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/while_let_loops.change_loop_body.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/sroa.rs stdout ----
---- [mir-opt] tests/mir-opt/sroa.rs stdout ----
29           _5 = ((_2 as Some).0: usize);    // scope 1 at $DIR/sroa.rs:+1:17: +1:18
30           _0 = _5;                         // scope 1 at $DIR/sroa.rs:+1:32: +1:33
31           StorageDead(_5);                 // scope 0 at $DIR/sroa.rs:+1:34: +1:35
+           StorageDead(_2);                 // scope 0 at $DIR/sroa.rs:+1:34: +1:35
32           goto -> bb3;                     // scope 0 at $DIR/sroa.rs:+1:5: +1:46
34   

35       bb2: {
35       bb2: {
+           StorageDead(_2);                 // scope 0 at $DIR/sroa.rs:+1:34: +1:35
36           _0 = const 0_usize;              // scope 0 at $DIR/sroa.rs:+1:43: +1:44
37           goto -> bb3;                     // scope 0 at $DIR/sroa.rs:+1:5: +1:46

39   
40       bb3: {
40       bb3: {
-           StorageDead(_2);                 // scope 0 at $DIR/sroa.rs:+2:1: +2:2
42           return;                          // scope 0 at $DIR/sroa.rs:+2:2: +2:2
44   }


thread '[mir-opt] tests/mir-opt/sroa.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/sroa.enums.ScalarReplacementOfAggregates.diff', src/tools/compiletest/src/runtest.rs:3483:21

failures:
    [mir-opt] tests/mir-opt/const_prop/discriminant.rs
    [mir-opt] tests/mir-opt/funky_arms.rs
