plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:50] 
[00:55:50] running 51 tests
[00:55:59] ERROR 2018-07-20T05:46:15Z: compiletest::runtest: None
[00:56:00] ERROR 2018-07-20T05:46:15Z: compiletest::runtest: None
[00:56:00] ERROR 2018-07-20T05:46:16Z: compiletest::runtest: None
[00:56:01] ERROR 2018-07-20T05:46:17Z: compiletest::runtest: None
Free Region Mapping
[00:56:09] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:56:09] | '_#1r    | Local    | ['_#1r]
[00:56:09] | Inferred Region Values
[00:56:09] | Inferred Region Values
[00:56:09] | '_#0r    | {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:56:09] | '_#1r    | {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:56:09] | Inference Constraints
[00:56:09] | Inference Constraints
[00:56:09] | '_#0r live at {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:56:09] | '_#1r live at {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:56:09] fn main() -> (){
[00:56:09]     let mut _0: ();
[00:56:09]     scope 1 {
[00:56:09]     scope 2 {
[00:56:09]     scope 2 {
[00:56:09]         let mut _1: std::boxed::Box<usize>;
[00:56:09]     }
[00:56:09]     let mut _2: std::boxed::Box<usize>;
[00:56:09]     bb0: {                              
[00:56:09]                                          | Live variables on entry to bb0[0]: []
[00:56:09]         StorageLive(_1);
[00:56:09]                                          | Live variables on entry to bb0[1]: []
[00:56:09]         _1 = const <std::boxed::Box<T>>::new(const 22usize) -> [return: bb2, unwind: bb1];
[00:56:09]     | Live variables on exit from bb0: []
[00:56:09]     bb1: {
[00:56:09]     bb1: {
[00:56:09]                                          | Live variables on entry to bb1[0]: []
[00:56:09]         resume;
[00:56:09]     | Live variables on exit from bb1: []
[00:56:09]     }
[00:56:09]     bb2: {                              
[00:56:09]                                          | Live variables on entry to bb2[0]: []
[00:56:09]         StorageLive(_2);
[00:56:09]                                          | Live variables on entry to bb2[1]: []
[00:56:09]         _2 = const can_panic() -> [return: bb3, unwind: bb4];
[00:56:09]     | Live variables on exit from bb2: []
[00:56:09]     }
[00:56:09]     bb3: {                              
[00:56:09]                                          | Live variables on entry to bb3[0]: []
[00:56:09]         replace(_1 <- move _2) -> [return: bb5, unwind: bb6];
[00:56:09]     | Live variables on exit from bb3: []
[00:56:09]     bb4: {
[00:56:09]     bb4: {
[00:56:09]                                          | Live variables on entry to bb4[0]: []
[00:56:09]         drop(_1) -> bb1;
[00:56:09]     | Live variables on exit from bb4: []
[00:56:09]     }
[00:56:09]     bb5: {                              
[00:56:09]                                          | Live variables on entry to bb5[0]: []
[00:56:09]         drop(_2) -> [return: bb7, unwind: bb4];
[00:56:09]     | Live variables on exit from bb5: []
[00:56:09]     bb6: {
[00:56:09]     bb6: {
[00:56:09]                                          | Live variables on entry to bb6[0]: []
[00:56:09]         drop(_2) -> bb4;
[00:56:09]     | Live variables on exit from bb6: []
[00:56:09]     }
[00:56:09]     bb7: {                              
[00:56:09]                                          | Live variables on entry to bb7[0]: []
[00:56:09]         StorageDead(_2);
[00:56:09]                                        ive(_4);
[00:56:09]            | Live variables on entry to bb3[3]: [_1]
[00:56:09]        _4 = _1;
[00:56:09]            | Live variables on entry to bb3[4]: [_4]
[00:56:09]        _3 = const use_x(move _4) -> [return: bb4, unwind: bb1];
[00:56:09]            | Live variables on exit from bb3: [_3]
[00:56:09] Actual:
[00:56:09] | Free Region Mapping
[00:56:09] | Free Region Mapping
[00:56:09] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:56:09] | '_#1r    | Local    | ['_#1r]
[00:56:09] | Inferred Region Values
[00:56:09] | Inferred Region Values
[00:56:09] | '_#0r    | {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:56:09] | '_#1r    | {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:56:09] | Inference Constraints
[00:56:09] | Inference Constraints
[00:56:09] | '_#0r live at {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:56:09] | '_#1r live at {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:56:09] fn main() -> (){
[00:56:09]     let mut _0: ();
[00:56:09]     scope 1 {
[00:56:09]     scope 2 {
[00:56:09]         let mut _1: usize;
[00:56:09]     }
[00:56:09]     }
[00:56:09]     let mut _2: ();
[00:56:09]     let mut _3: bool;
[00:56:09]     let mut _4: usize;
[00:56:09]     let mut _5: !;
[00:56:09]     bb0: {                              
[00:56:09]                                          | Live variables on entry to bb0[0]: []
[00:56:09]         StorageLive(_1);
[00:56:09]                                          | Live variables on entry to bb0[1]: []
[00:56:09]         _1 = const 22usize;
[56:09]         StorageDead(_4);
[00:56:09]                                          | Live variables on entry to bb4[1]: []
[00:56:09]         switchInt(move _3) -> [false: bb6, otherwise: bb5];
[00:56:09]     | Live variables on exit from bb4: []
[00:56:09]     }
[00:56:09]     bb5: {                              
[00:56:09]                                          | Live variables on entry to bb5[0]: []
[00:56:09]         _0 = ();
[00:56:09]                                          | Live variables on entry to bb5[1]: []
[00:56:09]         StorageDead(_3);
[00:56:09]                                          | Live variables on entry to bb5[2]: []
[00:56:09]         StorageDead(_1);
[00:56:09]                                          | Live variables on entry to bb5[3]: []
[00:56:09]         return;
[00:56:09]     | Live variables on exit from bb5: []
[00:56:09]     }
[00:56:09]     bb6: {                              
[00:56:09]                                          | Live variables on entry to bb6[0]: []
[00:56:09]         _2 = ();
[00:56:09]                                          | Live variables on entry to bb6[1]: []
[00:56:09]         StorageDead(_3);
[00:56:09]                                          | Live variables on entry to bb6[2]: []
[00:56:09]         goto -> bb2;
[00:56:09]     | Live variables on exit from bb6: []
[00:56:09] }', tools/compiletest/src/runtest.rs:2813:13
[00:56:09] 
[00:56:09] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:56:09] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:56:09] thread '[mir-opt] mir-opt/nll/liveness-interblock.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:56:09] Current block: None
[00:56:09] Actual Line: "                                         | Live variables on entry to bb3[0]: []"
[00:56:09] Expected Line: "            | Live variables on entry to bb3[0]: [_1]"
[00:56:09] Test Name: rustc.main.nll.0.mir
[00:56:09] Expected:
[00:56:09] ... (elided)
[00:56:09]     bb3: {
[00:56:09]             | Live variables on entry to bb3[0]: [_1]
[00:56:09]         StorageLive(_4);
[00:56:09]             | Live variables on entry to bb3[1]: [_1]
[00:56:09]         _4 = _1;
[00:56:09]             | Live variables on entry to bb3[2]: [_4]
[00:56:09]         _3 = const make_live(move _4) -> [return: bb5, unwind: bb1];
[00:56:09]             | Live variables on exit from bb3: []
[00:56:09] Actual:
[00:56:09] | Free Region Mapping
[00:56:09] | Free Region Mapping
[00:56:09] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:56:09] | '_#1r    | Local    | ['_#1r]
[00:56:09] | Inferred Region Values
[00:56:09] | Inferred Region Values
[00:56:09] | '_#0r    | {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:56:09] | '_#1r    | {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:56:09] | Inference Constraints
[00:56:09] | Inference Constraints
[00:56:09] | '_#0r live at {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:56:09] | '_#1r live at {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:56:09] fn main() -> (){
[00:56:09]     let mut _0: ();
[00:56:09]     scope 1 {
[00:56:09]     scope 2 {
[00:56:09]     scope 2 {
[00:56:09]  rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:09] 
[00:56:09] 
[00:56:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:09] Build completed unsuccessfully in 0:10:18
[00:56:09] Build completed unsuccessfully in 0:10:18
[00:56:09] make: *** [check] Error 1
[00:56:09] Makefile:58: recipe for target 'check' failed
60840 ./src/llvm-emscripten/lib
58532 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
55236 ./src/llvm/test/MC
55016 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
