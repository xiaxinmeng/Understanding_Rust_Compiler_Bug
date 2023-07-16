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
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.............ii.......i..............................F..
failures:

---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
1 - // MIR for `variant_a::{closure#0}` before CopyProp
2 + // MIR for `variant_a::{closure#0}` after CopyProp
3   
-   fn variant_a::{closure#0}(_1: &mut [closure@$DIR/slice_filter.rs:8:25: 8:39], _2: &&(usize, usize, usize, usize)) -> bool {
+   fn variant_a::{closure#0}(_1: &mut [closure@$DIR/slice_filter.rs:9:25: 9:39], _2: &&(usize, usize, usize, usize)) -> bool {
5       let mut _0: bool;                    // return place in scope 0 at $DIR/slice_filter.rs:+0:40: +0:40
6       let _3: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:27: +0:28
7       let _4: &usize;                      // in scope 0 at $DIR/slice_filter.rs:+0:30: +0:31

42           debug b => _4;                   // in scope 1 at $DIR/slice_filter.rs:+0:30: +0:31
43           debug c => _5;                   // in scope 1 at $DIR/slice_filter.rs:+0:33: +0:34
44           debug d => _6;                   // in scope 1 at $DIR/slice_filter.rs:+0:36: +0:37
-           scope 2 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:40: 8:46
+           scope 2 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:9:40: 9:46
46               debug self => _9;            // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
47               debug other => _10;          // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
48               let mut _29: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL

56                   let mut _34: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
58           }
58           }
-           scope 4 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:60: 8:66
+           scope 4 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:9:60: 9:66
60               debug self => _18;           // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
61               debug other => _19;          // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
62               let mut _35: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL

70                   let mut _40: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
72           }
72           }
-           scope 6 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:50: 8:56
+           scope 6 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:9:50: 9:56
74               debug self => _13;           // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
75               debug other => _14;          // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
76               let mut _41: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL

84                   let mut _46: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
86           }
86           }
-           scope 8 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:8:70: 8:76
+           scope 8 (inlined cmp::impls::<impl PartialOrd for &usize>::le) { // at $DIR/slice_filter.rs:9:70: 9:76
88               debug self => _22;           // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
89               debug other => _23;          // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
90               let mut _47: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL

thread '[mir-opt] tests/mir-opt/slice_filter.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/slice_filter.variant_a-{closure#0}.CopyProp.diff', src/tools/compiletest/src/runtest.rs:3481:21


failures:
    [mir-opt] tests/mir-opt/slice_filter.rs
