plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:46] 
[00:56:46] running 50 tests
[00:56:59] ERROR 2018-06-29T12:04:08Z: compiletest::runtest: None
[00:57:08] | Inferred Region Values
[00:57:08] | '_#0r    | {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:57:08] | '_#1r    | {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:57:08] | Inference Constraints
[00:57:08] | Inference Constraints
[00:57:08] | '_#0r live at {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:57:08] | '_#1r live at {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:57:08] fn main() -> (){
[00:57:08]     let mut _0: ();
[00:57:08]     scope 1 {
[00:57:08]     scope 2 {
[00:57:08]     scope 2 {
[00:57:08]         let mut _1: std::boxed::Box<usize>;
[00:57:08]     }
[00:57:08]     let mut _2: std::boxed::Box<usize>;
[00:57:08]     bb0: {                              
[00:57:08]                                          | Live variables on entry to bb0[0]: []
[00:57:08]         StorageLive(_1);
[00:57:08]                                          | Live variables on entry to bb0[1]: []
[00:57:08]         _1 = const <std::boxed::Box<T>>::new(const 22usize) -> [return: bb2, unwind: bb1];
[00:57:08]     | Live variables on entry to bb0: [_1 (drop)]
[00:57:08]     bb1: {
[00:57:08]     bb1: {
[00:57:08]                                          | Live variables on entry to bb1[0]: []
[00:57:08]         resume;
[00:57:08]     | Live variables on entry to bb1: []
[00:57:08]     }
[00:57:08]     bb2: {                              
[00:57:08]                                          | Live variables on entry to bb2[0]: [_1try to bb3[3]: [_1]
[00:57:08]        _4 = _1;
[00:57:08]            | Live variables on entry to bb3[4]: [_4]
[00:57:08]        _3 = const use_x(move _4) -> [return: bb4, unwind: bb1];
[00:57:08] Actual:
[00:57:08] | Free Region Mapping
[00:57:08] | Free Region Mapping
[00:57:08] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:57:08] | '_#1r    | Local    | ['_#1r]
[00:57:08] | Inferred Region Values
[00:57:08] | Inferred Region Values
[00:57:08] | '_#0r    | {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:57:08] | '_#1r    | {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:57:08] | Inference Constraints
[00:57:08] | Inference Constraints
[00:57:08] | '_#0r live at {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:57:08] | '_#1r live at {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:57:08] fn main() -> (){
[00:57:08]     let mut _0: ();
[00:57:08]     scope 1 {
[00:57:08]     scope 2 {
[00:57:08]         let mut _1: usize;
[00:57:08]     }
[00:57:08]     }
[00:57:08]     let mut _2: ();
[00:57:08]     let mut _3: bool;
[00:57:08]     let mut _4: usize;
[00:57:08]     let mut _5: !;
[00:57:08]     bb0: {                              
[00:57:08]                                          | Live variables on entry to bb0[0]: []
[00:57:08]         StorageLive(_1);
[00:57:08]                                          | Live variables on entry to bb0[1]: []
[00:57:08]         _1 = const 22usize;
[00:57:08]                                          | Live variables on entry to bb0[2]: []
[00:57:08]         goto -y to bb4[1]: [_3]
[00:57:08]         switchInt(move _3) -> [false: bb6, otherwise: bb5];
[00:57:08]     | Live variables on entry to bb4: []
[00:57:08]     }
[00:57:08]     bb5: {                              
[00:57:08]                                          | Live variables on entry to bb5[0]: []
[00:57:08]         _0 = ();
[00:57:08]                                          | Live variables on entry to bb5[1]: []
[00:57:08]         StorageDead(_3);
[00:57:08]                                          | Live variables on entry to bb5[2]: []
[00:57:08]         StorageDead(_1);
[00:57:08]                                          | Live variables on entry to bb5[3]: []
[00:57:08]         return;
[00:57:08]     | Live variables on entry to bb5: []
[00:57:08]     }
[00:57:08]     bb6: {                              
[00:57:08]                                          | Live variables on entry to bb6[0]: []
[00:57:08]         _2 = ();
[00:57:08]                                          | Live variables on entry to bb6[1]: []
[00:57:08]         StorageDead(_3);
[00:57:08]                                          | Live variables on entry to bb6[2]: []
[00:57:08]         goto -> bb2;
[00:57:08]     | Live variables on entry to bb6: []
[00:57:08] }', tools/compiletest/src/runtest.rs:2816:13
[00:57:08] 
[00:57:08] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:57:08] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:57:08] thread '[mir-opt] mir-opt/nll/liveness-interblock.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[00:57:08] Expected Line: "    | Live variables on entry to bb3: [_1]"
[00:57:08] Test Name: rustc.main.nll.0.mir
[00:57:08] Expected:
[00:57:08] ... (elided)
[00:57:08]     | Live variables on entry to bb3: [_1]
[00:57:08]     bb3: {
[00:57:08]             | Live variables on entry to bb3[0]: [_1]
[00:57:08]         StorageLive(_4);
[00:57:08]             | Live variables on entry to bb3[1]: [_1]
[00:57:08]         _4 = _1;
[00:57:08]             | Live variables on entry to bb3[2]: [_4]
[00:57:08]         _3 = const make_live(move _4) -> [return: bb5, unwind: bb1];
[00:57:08] Actual:
[00:57:08] | Free Region Mapping
[00:57:08] | Free Region Mapping
[00:57:08] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:57:08] | '_#1r    | Local    | ['_#1r]
[00:57:08] | Inferred Region Values
[00:57:08] | Inferred Region Values
[00:57:08] | '_#0r    | {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:57:08] | '_#1r    | {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:57:08] | Inference Constraints
[00:57:08] | Inference Constraints
[00:57:08] | '_#0r live at {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:57:08] | '_#1r live at {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:57:08] fn main() -> (){
[00:57:08]     let mut _0: ();
[00:57:08]     scope 1 {
[00:57:08]     scope 2 {
[00:57:08]         let _1: usize;
[00:57:08]     }
[00:57:08]     }
[00:57:08]     let mut _2: bool;
[00:57:08]     let mut _3: ();
[00:57:08]     let mut _4: usize;
[00:57:08]     let mut _5: ();
[00:57:08]     bb0: {                              
[00:57:08]             bb3: []
[00:57:08]     }
[00:57:08]     bb4: {                              
[00:57:08]                                          | Live variables on entry to bb4[0]: []
[00:57:08]         _5 = const make_dead() -> [return: bb6, unwind: bb1];
[00:57:08]     | Live variables on entry to bb4: []
[00:57:08]     }
[00:57:08]     bb5: {                              
[00:57:08]                                          | Live variables on entry to bb5[0]: []
[00:57:08]         StorageDead(_4);
[00:57:08]                                          | Live variables on entry to bb5[1]: []
[00:57:08]         _0 = ();
[00:57:08]                                          | Live variables on entry to bb5[2]: []
[00:57:08]         goto -> bb7;
[00:57:08]     | Live variables on entry to bb5: []
[00:57:08]     }
[00:57:08]     bb6: {                              
[00:57:08]                                          | Live variables on entry to bb6[0]: []
[00:57:08]         _0 = ();
[00:57:08]                                          | Live variables on entry to bb6[1]: []
[00:57:08]         goto -> bb7;
[00:57:08]     | Live variables on entry to bb6: []
[00:57:08]     }
[00:57:08]     bb7: {                              
[00:57:08]                                          | Live variables on entry to bb7[0]: []
[00:57:08]         StorageDead(_2);
[00:57:08]                                          | Live variables on entry to bb7[1]: []
[00:57:08]         StorageDead(_1);
[00:57:08]                                          | Live variables on entry to bb7[2]: []
[00:57:08]         return;
[00:57:08]     | Live vari0..=1], bb7[0..=3]}
[00:57:08] | '_#2r live at {bb2[0]}
[00:57:08] | '_#3r live at {bb2[1], bb3[0..=1]}
[00:57:08] | '_#2r: '_#3r @ bb2[1] due to /checkout/src/test/mir-opt/nll/region-liveness-basic.rs:24:13: 24:18
[00:57:08] fn main() -> (){
[00:57:08]     let mut _0: ();
[00:57:08]     scope 1 {
[00:57:08]         scope 3 {
[00:57:08]         scope 4 {
[00:57:08]         scope 4 {
[00:57:08]             let _2: &'_#3r usize;
[00:57:08]     }
[00:57:08]     scope 2 {
[00:57:08]     scope 2 {
[00:57:08]         let mut _1: [usize; 3];
[00:57:08]     let mut _3: usize;
[00:57:08]     let mut _4: usize;
[00:57:08]     let mut _4: usize;
[00:57:08]     let mut _5: bool;
[00:57:08]     let mut _6: bool;
[00:57:08]     let mut _7: usize;
[00:57:08]     let mut _8: bool;
[00:57:08]     bb0: {                              
[00:57:08]                                          | Live variables on entry to bb0[0]: []
[00:57:08]         StorageLive(_1);
[00:57:08]                                          | Live variables on entry to bb0[1]: []
[00:57:08]         _1 = [const 1usize, const 2usize, const 3usize];
[00:57:08]                                          | Live variables on entry to bb0[2]: [_1]
[00:57:08]         StorageLive(_2);
[00:57:08]                                          | Live variables on entry to bb0[3]: [_1]
[00:57:08]         StorageLive(_3);
[00:57:08]                                          | Live variables on entry to bb0[4]: [_1]
[00:57:08]         _3 = const 0usize;
[00:57:08]                                          | Live variables on entry to bb0[5]: [_1, _3]
[00:57:08]         _4 = Len(_1);
[00:57:08]                                          | Live variables on entry to bb0[6]: [_1, _3, _4]
[00:57:08]         _5 = Lt(_3, _4);
[00:57:08]                                          | Live variables on entry to bb0[7]: [_1, _3, _4, _5]
[00:57:08]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[00:57:08]     | Live variables on entry to bb0: [_1, _3]
[00:57:08]     bb1: {
[00:57:08]     bb1: {
[00:57:08]                                          | Live variables on entry to bb1[0]: []
[00:57:08]         resume;
[00:57:08]     | Live variables on entry to bb1: []
[00:57:08]     }
[00:57:08]     bb2: {                              
[00:57:08]                                          | Live variables on entry to bb2[0]: [_1, _3]
[00:57:08]         _2 = &'_#2r _1[_3];
[00:57:08]                                          | Live variables on entry to bb2[1]: [_2]
[00:57:08]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[00:57:08]     | Live variables on entry to bb2: [_2]
[00:57:08]     }
[00:57:08]     bb3: {                              
[00:57:08]                                          | Live variables on entry to bb3[0]: [_2]
[00:57:08]         StorageLive(_7);
[00:57:08]                                          | Live variables on entry to bb3[1]: [_2]
[00:57:08]         _7 = (*_2);
[00:57:08]                                          | Live variables on entry to bb3[2]: [_7]
[00:57:08]         _6 = const use_x(move _7) -> [return: bb5, unwind: bb1];
[00:57:08]     | Live variables on entry to bb3: []
[00:57:08]     }
[   | Live variables on entry to bb7[3]: []
[00:57:08]         return;
[00:57:08]     | Live variables on entry to bb7: []
[00:57:08] }', tools/compiletest/src/runtest.rs:2816:13
[00:57:08] 
[00:57:08] 
[00:57:08] failures:
---
[00:57:08] test result: FAILED. 46 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[00:57:08] 
[00:57:08] 
[00:57:08] 
[00:57:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:08] 
[00:57:08] 
[00:57:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:08] Build completed unsuccessfully in 0:15:11
[00:57:08] Build completed unsuccessfully in 0:15:11
[00:57:08] make: *** [check] Error 1
[00:57:08] Makefile:58: recipe for target 'check' failed
33884 ./src/llvm-emscripten/lib/Target
32952 ./.git/modules/src/libcompiler_builtins
32188 ./.git/modules/src/libcompiler_builtins/modules
32184 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
