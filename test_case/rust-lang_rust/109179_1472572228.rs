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

running 235 tests
.........................................................................i.............. 88/235
.............................................................i.......................... 176/235
...............ii..F....i..................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
24           _4 = &raw const (*_1);           // scope 1 at $DIR/lower_intrinsics.rs:+2:55: +2:56
25 -         _3 = option_payload_ptr::<usize>(move _4) -> bb1; // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57
26 -                                          // mir::Constant
- -                                          // + span: $DIR/lower_intrinsics.rs:87:18: 87:54
+ -                                          // + span: $DIR/lower_intrinsics.rs:99:18: 99:54
28 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const Option<usize>) -> *const usize {option_payload_ptr::<usize>}, val: Value(<ZST>) }
29 +         _3 = &raw const (((*_4) as Some).0: usize); // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57
30 +         goto -> bb1;                     // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57

37           _6 = &raw const (*_2);           // scope 2 at $DIR/lower_intrinsics.rs:+3:55: +3:56
38 -         _5 = option_payload_ptr::<String>(move _6) -> bb2; // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57
39 -                                          // mir::Constant
- -                                          // + span: $DIR/lower_intrinsics.rs:88:18: 88:54
+ -                                          // + span: $DIR/lower_intrinsics.rs:100:18: 100:54
41 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const Option<String>) -> *const String {option_payload_ptr::<String>}, val: Value(<ZST>) }
42 +         _5 = &raw const (((*_6) as Some).0: std::string::String); // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57
43 +         goto -> bb2;                     // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57

thread '[mir-opt] tests/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_intrinsics.option_payload.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3492:21


failures:
    [mir-opt] tests/mir-opt/lower_intrinsics.rs
