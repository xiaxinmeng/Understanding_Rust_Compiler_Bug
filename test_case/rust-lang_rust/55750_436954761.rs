plain
travis_time:end:01ec3f25:start=1541670380935691582,finish=1541670443435476600,duration=62499785018
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:02] .................................................................................................... 100/4999
[00:52:05] .................................................................................................... 200/4999
[00:52:08] ........................................................................ii...................ii..... 300/4999
[00:52:11] ...........................................................................................iii...... 400/4999
[00:52:14] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:52:21] .................................................................................................... 700/4999
[00:52:27] .....................................................................i...........i.................. 800/4999
[00:52:31] ........................................................................................iiiii....... 900/4999
[00:52:34] ...........ii.iiii.................................................................................. 1000/4999
---
[00:53:12] .................................................................................................... 2200/4999
[00:53:16] .................................................................................................... 2300/4999
[00:53:20] .................................................................................................... 2400/4999
[00:53:24] .................................................................................................... 2500/4999
[00:53:27] ...................................................................iiiiiiiii........................ 2600/4999
[00:53:34] ...............................ii................................................................... 2800/4999
[00:53:37] .................................................................................................... 2900/4999
[00:53:41] .................................................................................................... 3000/4999
[00:53:44] ..........................i......................................................................... 3100/4999
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:24] 
[01:07:24] running 45 tests
[01:07:28] ERROR 2018-11-08T10:55:02Z: compiletest::runtest: None
[01:07:28] ERROR 2018-11-08T10:55:02Z: compiletest::runtest: None
[01:07:28] ERROR 2018-11-08T10:55:02Z: compiletest::runtest: None
[01:07:30] ERROR 2018-11-08T10:55:03Z: compiletest::runtest: None
[01:07:45] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:07:45] .............FFF..F..F.F..............F......
[01:07:45] 
[01:07:45] ---- [mir-opt] mir-opt/end_region_5.rs stdout ----
[01:07:45] ---- [mir-opt] mir-opt/end_region_5.rs stdout ----
[01:07:45] thread '[mir-opt] mir-opt/end_region_5.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:07:45] Current block: None
[01:07:45] Actual Line: "    let mut _3: [closure@28 d:&\'18s D];"
[01:07:45] Expected Line: "    let mut _3: [closure@NodeId(28) d:&\'18s D];"
[01:07:45] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:07:45] ... (elided)
[01:07:45] fn main() -> () {
[01:07:45] ... (elided)
[01:07:45] ... (elided)
[01:07:45]     let mut _0: ();
[01:07:45] ... (elided)
[01:07:45]     let _1: D;
[01:07:45] ... (elided)
[01:07:45]     let mut _2: ();
[01:07:45]     let mut _3: [closure@NodeId(28) d:&'18s D];
[01:07:45]     let mut _4: &'18s D;
[01:07:45]     bb0: {
[01:07:45]         StorageLive(_1);
[01:07:45]         _1 = D(const 0i32,);
[01:07:45]         FakeRead(ForLet, _1);
[01:07:45]         StorageLive(_3);
[01:07:45]     }
[01:07:45]     bb2: {
[01:07:45]     bb2: {
[01:07:45]         drop(_3) -> [return: bb5, unwind: bb3];
[01:07:45]     bb3: {
[01:07:45]         drop(_1) -> bb1;
[01:07:45]     }
[01:07:45]     bb4: {
[01:07:45]     bb4: {
[01:07:45]         drop(_3) -> bb3;
[01:07:45]     }
[01:07:45]     bb5: {
[01:07:45]         StorageDead(_3);
[01:07:45]         _0 = ();
[01:07:45]         drop(_1) -> [return: bb6, unwind: bb1];
[01:07:45]     bb6: {
[01:07:45]     bb6: {
[01:07:45]         StorageDead(_1);
[01:07:45]         return;
[01:07:45] }
[01:07:45] Actual:
[01:07:45] fn main() -> (){
[01:07:45] fn main() -> (){
[01:07:45]     let mut _0: ();
[01:07:45]     scope 1 {
[01:07:45]     scope 2 {
[01:07:45]         let _1: D;
[01:07:45]     }
[01:07:45]     }
[01:07:45]     let mut _2: ();
[01:07:45]     let mut _3: [closure@33 d:D];
[01:07:45]     bb0: {                              
[01:07:45]         StorageLive(_1);
[01:07:45]         _1 = D(const 0i32,);
[01:07:45]         FakeRead(ForLet, _1);
[01:07:45]         StorageLive(_3);
[01:07:45]         _3 = [closure@33] { d: move _1 };
[01:07:45]         _2 = const foo(move _3) -> [return: bb2, unwind: bb4];
[01:07:45]     bb1: {
[01:07:45]         resume;
[01:07:45]     }
[01:07:45]     }
[01:07:45]     bb2: {                              
[01:07:45]         drop(_3) -> [return: bb5, unwind: bb3];
[01:07:45]     bb3: {
[01:07:45]         drop(_1) -> bb1;
[01:07:45]     }
[01:07:45]     bb4: {
[01:07:45]     bb4: {
[01:07:45]         drop(_3) -> bb3;
[01:07:45]     }
[01:07:45]     bb5: {                              
[01:07:45]         StorageDead(_3);
[01:07:45]         _0 = ();
[01:07:45]         drop(_1) -> [return: bb6, unwind: bb1];
[01:07:45]     }
[01:07:45]     bb6: {                              
[01:07:45]         StorageDead(_1);
[01:07:45]         return;
[01:07:45] }', tools/compiletest/src/runtest.rs:2939:13
[01:07:45] 
[01:07:45] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[01:07:45] ---- [mir-opt] mir-opt/end_region_6.rs stdout ----
[01:07:45] thread '[mir-opt] mir-opt/end_region_6.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:07:45] Current block: None
[01:07:45] Actual Line: "    let mut _3: [closure@33 d:&\'24s D];"
[01:07:45] Expected Line: "    let mut _3: [closure@NodeId(33) d:&\'24s D];"
[01:07:45] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:07:45] ... (elided)
[01:07:45] fn main() -> () {
[01:07:45] fn main() -> () {
[01:07:45]     let mut _0: ();
[01:07:45] ... (elided)
[01:07:45]     let _1: D;
[01:07:45] ... (elided)
[01:07:45]     let mut _2: ();
[01:07:45]     let mut _3: [closure@NodeId(33) d:&'24s D];
[01:07:45]     let mut _4: &'24s D;
[01:07:45]     bb0: {
[01:07:45]         StorageLive(_1);
[01:07:45]         _1 = D(const 0i32,);
[01:07:45]         FakeRead(ForLet, _1);
[01:07:45]         StorageLive(_3);
[01:07:45]         StorageLive(_4);
[01:07:45]         _4 = &'24s _1;
[01:07:45]         _3 = [closure@NodeId(33)] { d: move _4 };
[01:07:45]         StorageDead(_4);
[01:07:45]         _2 = const foo(move _3) -> [return: bb2, unwind: bb3];
[01:07:45]     bb1: {
[01:07:45]         resume;
[01:07:45]     }
[01:07:45]     bb2: {
[01:07:45]     bb2: {
[01:07:45]         EndRegion('24s);
[01:07:45]         StorageDe: {
[01:07:45]        EndRegion('24s);
[01:07:45]        EndRegion('26_1rs);
[01:07:45]        drop(_1) -> bb1;
[01:07:45]    bb4: {
[01:07:45]    bb4: {
[01:07:45]        StorageDead(_1);
[01:07:45]        return;
[01:07:45] }
[01:07:45] Actual:
[01:07:45] fn main() -> (){
[01:07:45] fn main() -> (){
[01:07:45]     let mut _0: ();
[01:07:45]     scope 1 {
[01:07:45]         scope 3 {
[01:07:45]         scope 4 {
[01:07:45]         scope 4 {
[01:07:45]             let _2: &'26_1rs D;
[01:07:45]     }
[01:07:45]     scope 2 {
[01:07:45]         let _1: D;
[01:07:45]     }
[01:07:45]     }
[01:07:45]     let mut _3: ();
[01:07:45]     let mut _4: [closure@33 r:&'24s D];
[01:07:45]     bb0: {                              
[01:07:45]         StorageLive(_1);
[01:07:45]         _1 = D(const 0i32,);
[01:07:45]         FakeRead(ForLet, _1);
[01:07:45]         StorageLive(_2);
[01:07:45]         _2 = &'26_1rs _1;
[01:07:45]         FakeRead(ForLet, _2);
[01:07:45]         StorageLive(_4);
[01:07:45]         _4 = [closure@33] { r: _2 };
[01:07:45]         _3 = const foo(move _4) -> [return: bb2, unwind: bb3];
[01:07:45]     bb1: {
[01:07:45]         resume;
[01:07:45]     }
[01:07:45]     }
[01:07:45]     bb2: {                              
[01:07:45]         EndRegion('24s);
[01:07:45]         StorageDead(_4);
[01:07:45]         _0 = ();
[01:07:45]         EndRegion('26_1rs);
[01:07:45]         StorageDead(_2);
[01:07:45]         drop(_1) -> [return: bb4, unwind: bb1];
[01:07:45]     bb3: {
[01:07:45]     bb3: {
[01:07:45]         EndRegion('24s);
[01:07:45]         EndRegion('26_1rs);
[01:07:45]         drop(_1) -> bb1;
[01:07:45]     }
[01:07:45]     bb4: {                              
[01:07:45]         StorageDead(_1);
[01:07:45]         return;
[01:07:45] }', tools/compiletest/src/runtest.rs:2939:13
[01:07:45] 
[01:07:45] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:07:45] ---- [mir-opt] mir-opt/inline-closure-borrows-arg.rs stdout ----
[01:07:45] thread '[mir-opt] mir-opt/inline-closure-borrows-arg.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:07:45] Expected Line: "    _3 = [closure@NodeId(53)];"
[01:07:45] Test Name: rustc.foo.Inline.after.mir
[01:07:45] ... (elided)
[01:07:45] ... (elided)
[01:07:45] bb0: {
[01:07:45] ... (elided)
[01:07:45] ... (elided)
[01:07:45]     _3 = [closure@NodeId(53)];
[01:07:45] ... (elided)
[01:07:45]     _4 = &_3;
[01:07:45] ... (elided)
[01:07:45]     _6 = &(*_2);
[01:07:45] ... (elided)
[01:07:45]     _7 = &(*_2);
[01:07:45]     _5 = (move _6, move _7);
[01:07:45]     _8 = move (_5.0: &i32);
[01:07:45]     _9 = move (_5.1: &i32);
[01:07:45] ... (elided)
[01:07:45]     _0 = (*_8);
[01:07:45] ... (elided)
[01:07:45]     return;
[01:07:45] }
[01:07:45] ... (elided)
[01:07:45] Actual:
[01:07:45] fn foo(_1: T, _2: &i32) -> i32{
[01:07:45]     let mut _0: i32;
[01:07:45]     scope 1 {
[01:07:45]         scope 3 {
[01:07:45]     }
[01:07:45]     scope 2 {
[01:07:45]     scope 2 {
[01:07:45]         let _3: [closure@53];
[01:07:45]     scope 4 {
[01:07:45]     }
[01:07:45]     scope 5 {
[01:07:45]     }
[01:07:45]     }
[01:07:45]     let mut _4: &[closure@53];
[01:07:45]     let mut _5: (&i32, &i32);
[01:07:45]     let mut _6: &i32;
[01:07:45]     let mut _7: &i32;
[01:07:45]     let mut _8: &i32;
[01:07:45]     let mut _9: &i32;
[01:07:45]     bb0: {                              
[01:07:45]         StorageLive(_3);
[01:07:45]         _3 = [closure@53];
[01:07:45]         StorageLive(_4);
[01:07:45]         _4 = &_3;
[01:07:45]         StorageLive(_5);
[01:07:45]         StorageLive(_6);
[01:07:45]         _6 = &(*_2);
[01:07:45]         StorageLive(_7);
[01:07:45]         _7 = &(*_2);
[01:07:45]         _5 = (move _6, move _7);
[01:07:45]         _8 = move (_5.0: &i32);
[01:07:45]         _9 = move (_5.1: &i32);
[01:07:45]         _0 = (*_8);
[01:07:45]         StorageDead(_5);
[01:07:45]         StorageDead(_7);
[01:07:45]         StorageDead(_6);
[01:07:45]         StorageDead(_4);
[01:07:45]         StorageDead(_3);
[01:07:45]         return;
[01:07:45] }', tools/compiletest/src/runtest.rs:2939:13
[01:07:45] 
[01:07:45] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:07:45] ---- [mir-opt] mir-opt/inline-closure.rs stdout ----
[01:07:45] thread '[mir-opt] mir-opt/inline-closure.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:07:45] Expected Line: "    _3 = [closure@NodeId(39)];"
[01:07:45] Test Name: rustc.foo.Inline.after.mir
[01:07:45] ... (elided)
[01:07:45] ... (elided)
[01:07:45] bb0: {
[01:07:45] ... (elided)
[01:07:45] ... (elided)
[01:07:45]     _3 = [closure@NodeId(39)];
[01:07:45] ... (elided)
[01:07:45]     _4 = &_3;
[01:07:45] ... (elided)
[01:07:45]     _6 = _2;
[01:07:45] ... (elided)
[01:07:45]     _7 = _2;
[01:07:45]     _5 = (move _6, move _7);
[01:07:45]     _8 = move (_5.0: i32);
[01:07:45]     [mir-opt] mir-opt/inline-closure.rs
[01:07:45]     [mir-opt] mir-opt/retag.rs
[01:07:45] 
[01:07:45] test result: FAILED. 38 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out
[01:07:45] test result: FAILED. 38 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out
[01:07:45] 
[01:07:45] 
[01:07:45] 
[01:07:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:45] 
[01:07:45] 
[01:07:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:45] Build completed unsuccessfully in 0:19:34
[01:07:45] Build completed unsuccessfully in 0:19:34
[01:07:45] Makefile:58: recipe for target 'check' failed
[01:07:45] make: *** [check] Error 1
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6h5p1l2eb-tliqph-22tmsi8iacpi9
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130752 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123680 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
123608 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
---
travis_time:end:16fb993d:start=1541674521028841518,finish=1541674521035958272,duration=7116754
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:162991d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf tr
