plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........ii.......i...............................F.
failures:

---- [mir-opt] tests/mir-opt/sroa.rs stdout ----
45 +         _9 = move _6;                    // scope 0 at $DIR/sroa.rs:+1:30: +1:70
46 +         _10 = const "a";                 // scope 0 at $DIR/sroa.rs:+1:30: +1:70
47                                            // mir::Constant
-                                            // + span: $DIR/sroa.rs:60:52: 60:55
+                                            // + span: $DIR/sroa.rs:53:52: 53:55
49                                            // + literal: Const { ty: &str, val: Value(Slice(..)) }
50 +         _11 = move _7;                   // scope 0 at $DIR/sroa.rs:+1:30: +1:70
51 +         nop;                             // scope 0 at $DIR/sroa.rs:+1:30: +1:70

thread '[mir-opt] tests/mir-opt/sroa.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/sroa.flat.ScalarReplacementOfAggregates.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/sroa.rs
