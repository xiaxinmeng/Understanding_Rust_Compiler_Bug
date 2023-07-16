plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:55] 
[00:48:55] running 51 tests
[00:49:04] ERROR 2018-07-20T14:03:32Z: compiletest::runtest: None
[00:49:04] ERROR 2018-07-20T14:03:33Z: compiletest::runtest: None
[00:49:05] ERROR 2018-07-20T14:03:33Z: compiletest::runtest: None
[00:49:05] ERROR 2018-07-20T14:03:34Z: compiletest::runtest: None
[00:49:13] .............................FF.F..F...............
[00:49:13] 
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-call-subtlety.rs stdout ----
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-call-subtlety.rs stdout ----
[00:49:13] thread '[mir-opt] mir-opt/nll/liveness-call-subtlety.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:49:13] Current block: None
[00:49:13] Actual Line: "    | Live variables on exit from bb0: []"
[00:49:13] Expected Line: "           | Live variables on exit from bb0: [_1 (drop)]"
[00:49:13] Test Name: rustc.main.nll.0.mir
[00:49:13] Expected:
[00:49:13] ... (elided)
[00:49:13]    bb0: {
[00:49:13]            | Live variables on entry to bb0[0]: []
[00:49:13]        StorageLive(_1);
[00:49:13]            | Live variables on entry to bb0[1]: []
[00:49:13]        _1 = const <std::boxed::Box<T>>::new(const 22usize) -> [return: bb2, unwind: bb1];
[00:49:13]            | Live variables on exit from bb0: [_1 (drop)]
[00:49:13] Actual:
[00:49:13] | Free Region Mapping
[00:49:13] | Free Region Mapping
[00:49:13] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:49:13] | '_#1r    | Local    | ['_#1r]
[00:49:13] | Inferred Region Values
[00:49:13] | Inferred Region Values
[00:49:13] | '_#0r    | {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:49:13] | '_#1r    | {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:49:13] | Inference Constraints
[00:49:13] | Inference Constraints
[00:49:13] | '_#0r live at {'_#0r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:49:13] | '_#1r live at {'_#1r, bb0[0..=1], bb1[0], bb2[0..=1], bb3[0], bb4[0], bb5[0], bb6[0], bb7[0..=2], bb8[0..=1]}
[00:49:13] fn main() -> (){
[00:49:13]     let mut _0: ();
[00:49:13]     scope 1 {
[00:49:13]     scope 2 {
[00:49:13]     scope 2 {
[00:49:13]         let mut _1: std::boxed::Box<usize>;
[00:49:13]     }
[00:49:13]     let mut _2: std::boxed::Box<usize>;
[00:49:13]     bb0: {                              
[00:49:13]                                          | Live variables on entry to bb0[0]: []
[00:49:13]         StorageLive(_1);
[00:49:13]                                          | Live variables on entry to bb0[1]: []
[00:49:13]         _1 = const <std::boxed::Box<T>>::new(const 22usize) -> [return: bb2, unwind: bb1];
[00:49:13]     | Live variables on exit from bb0: []
[00:49:13]     bb1: {
[00:49:13]     bb1: {
[00:49:13]                                          | Live variables on entry to bb1[0]: []
[00:49:13]         resume;
[00:49:13]     | Live variables on exit from bb1: []
[00:49:13]     }
[00:49:13]     bb2: {                              
[00:49:13]                                          | Live variables on entry to bb2[0]: []
[00:49:13]         StorageLive(_2);
[00:49:13]                                          | Live variables on entry to bb2[1]: []
[00:49:13]         _2 = const can_panic() -> [return: bb3, unwind: bb4];
[00:49:13]     | Live variables on exit from bb2: []
[00:49:13]     }
[00:49:13]     bb3: {                              
[00:49:13]                                          | Live variables on entry to bb3[0]: []
[00:49:13]         replace(_1 <- move _2) -> [return: bb5, unwind: bb6];
[00:49:13]     | Live variables on exit from bb3: []
[00:49:13]     bb4: {
[00:49:13]     bb4: {
[00:49:13]                                          | Live variables on entry to bb4[0]: []
[00:49:13]         drop(_1) -> bb1;
[00:49:13]     | Live variables on exit from bb4: []
[00:49:13]     }
[00:49:13]     bb5: {                              
[00:49:13]                                          | Live variables on entry to bb5[0]: []
[00:49:13]         drop(_2) -> [return: bb7, unwind: bb4];
[00:49:13]     | Live variables on exit from bb5: []
[00:49:13]     bb6: {
[00:49:13]     bb6: {
[00:49:13]                                          | Live variables on entry to bb6[0]: []
[00:49:13]         drop(_2) -> bb4;
[00:49:13]     | Live variables on exit from bb6: []
[00:49:13]     }
[00:49:13]     bb7: {                              
[00:49:13]                                          | Live variables on entry to bb7[0]: []
[00:49:13]         StorageDead(_2);
[00:49:13]                                          | Live variables on entry to bb7[1]: []
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb7[2]: []
[00:49:13]         drop(_1) -> [return: bb8, unwind: bb1];
[00:49:13]     | Live variables on exit from bb7: []
[00:49:13]     }
[00:49:13]     bb8: {                              
[00:49:13]                                          | Live variables on entry to bb8[0]: []
[00:49:13]         StorageDead(_1);
[00:49:13]                                          | Live variables on entry to bb8[1]: []
[00:49:13]         return;
[00:49:13]     | Live variables on exit from bb8: []
[00:49:13] }', tools/compiletest/src/runtest.rs:2813:13
[00:49:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:13] 
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-drop-intra-block.rs stdout ----
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-drop-intra-block.rs stdout ----
[00:49:13] thread '[mir-opt] mir-opt/nll/liveness-drop-intra-block.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:49:13] Current block: None
[00:49:13] Actual Line: "                                         | Live variables on entry to bb3[1]: []"
[00:49:13] Expected Line: "           | Live variables on entry to bb3[1]: [_1]"
[00:49:13] Test Name: rustc.main.nll.0.mir
[00:49:13] Expected:
[00:49:13] ... (elided)
[00:49:13]    bb3: {
[00:49:13]            | Live variables on entry to bb3[0]: []
[00:49:13]        _1 = const 55usize;
[00:49:13]            | Live variables on entry to bb3[1]: [_1]
[00:49:13]        StorageLive(_3);
[00:49:13]            | Live variables on entry to bb3[2]: [_1]
[00:49:13]        StorageLive(_4);
[00:49:13]            | Live variables on entry to bb3[3]: [_1]
[00:49:13]        _4 = _1;
[00:49:13]            | Live variables on entry to bb3[4]: [_4]
[00:49:13]        _3 = const use_x(move _4) -> [return: bb4, unwind: bb1];
[00:49:13]            | Live variables on exit from bb3: [_3]
[00:49:13] Actual:
[00:49:13] | Free Region Mapping
[00:49:13] | Free Region Mapping
[00:49:13] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:49:13] | '_#1r    | Local    | ['_#1r]
[00:49:13] | Inferred Region Values
[00:49:13] | Inferred Region Values
[00:49:13] | '_#0r    | {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:49:13] | '_#1r    | {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:49:13] | Inference Constraints
[00:49:13] | Inference Constraints
[00:49:13] | '_#0r live at {'_#0r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:49:13] | '_#1r live at {'_#1r, bb0[0..=2], bb1[0], bb2[0], bb3[0..=4], bb4[0..=1], bb5[0..=3], bb6[0..=2]}
[00:49:13] fn main() -> (){
[00:49:13]     let mut _0: ();
[00:49:13]     scope 1 {
[00:49:13]     scope 2 {
[00:49:13]         let mut _1: usize;
[00:49:13]     }
[00:49:13]     }
[00:49:13]     let mut _2: ();
[00:49:13]     let mut _3: bool;
[00:49:13]     let mut _4: usize;
[00:49:13]     let mut _5: !;
[00:49:13]     bb0: {                              
[00:49:13]                                          | Live variables on entry to bb0[0]: []
[00:49:13]         StorageLive(_1);
[00:49:13]                                          | Live variables on entry to bb0[1]: []
[00:49:13]         _1 = const 22usize;
[00:49:13]                                          | Live variables on entry to bb0[2]: []
[00:49:13]         goto -> bb2;
[00:49:13]     | Live variables on exit from bb0: []
[00:49:13]     bb1: {
[00:49:13]     bb1: {
[00:49:13]                                          | Live variables on entry to bb1[0]: []
[00:49:13]         resume;
[00:49:13]     | Live variables on exit from bb1: []
[00:49:13]     }
[00:49:13]     bb2: {                              
[00:49:13]                                          | Live variables on entry to bb2[0]: []
[00:49:13]         falseUnwind -> [real: bb3, cleanup: bb1];
[00:49:13]     | Live variables on exit from bb2: []
[00:49:13]     }
[00:49:13]     bb3: {                              
[00:49:13]                                          | Live variables on entry to bb3[0]: []
[00:49:13]         _1 = const 55usize;
[00:49:13]                                          | Live variables on entry to bb3[1]: []
[00:49:13]         StorageLive(_3);
[00:49:13]                                          | Live variables on entry to bb3[2]: []
[00:49:13]         StorageLive(_4);
[00:49:13]                                          | Live variables on entry to bb3[3]: []
[00:49:13]         _4 = _1;
[00:49:13]                                          | Live variables on entry to bb3[4]: []
[00:49:13]         _3 = const use_x(move _4) -> [return: bb4, unwind: bb1];
[00:49:13]     | Live variables on exit from bb3: []
[00:49:13]     }
[00:49:13]     bb4: {                              
[00:49:13]                                          | Live variables on entry to bb4[0]: []
[00:49:13]         StorageDead(_4);
[00:49:13]                                          | Live variables on entry to bb4[1]: []
[00:49:13]         switchInt(move _3) -> [false: bb6, otherwise: bb5];
[00:49:13]     | Live variables on exit from bb4: []
[00:49:13]     }
[00:49:13]     bb5: {                              
[00:49:13]                                          | Live variables on entry to bb5[0]: []
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb5[1]: []
[00:49:13]         StorageDead(_3);
[00:49:13]                                          | Live variables on entry to bb5[2]: []
[00:49:13]         StorageDead(_1);
[00:49:13]                                          | Live variables on entry to bb5[3]: []
[00:49:13]         return;
[00:49:13]     | Live variables on exit from bb5: []
[00:49:13]     }
[00:49:13]     bb6: {                              
[00:49:13]                                          | Live variables on entry to bb6[0]: []
[00:49:13]         _2 = ();
[00:49:13]                                          | Live variables on entry to bb6[1]: []
[00:49:13]         StorageDead(_3);
[00:49:13]                                          | Live variables on entry to bb6[2]: []
[00:49:13]         goto -> bb2;
[00:49:13]     | Live variables on exit from bb6: []
[00:49:13] }', tools/compiletest/src/runtest.rs:2813:13
[00:49:13] 
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:49:13] ---- [mir-opt] mir-opt/nll/liveness-interblock.rs stdout ----
[00:49:13] thread '[mir-opt] mir-opt/nll/liveness-interblock.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:49:13] Current block: None
[00:49:13] Actual Line: "                                         | Live variables on entry to bb3[0]: []"
[00:49:13] Expected Line: "            | Live variables on entry to bb3[0]: [_1]"
[00:49:13] Test Name: rustc.main.nll.0.mir
[00:49:13] Expected:
[00:49:13] ... (elided)
[00:49:13]     bb3: {
[00:49:13]             | Live variables on entry to bb3[0]: [_1]
[00:49:13]         StorageLive(_4);
[00:49:13]             | Live variables on entry to bb3[1]: [_1]
[00:49:13]         _4 = _1;
[00:49:13]             | Live variables on entry to bb3[2]: [_4]
[00:49:13]         _3 = const make_live(move _4) -> [return: bb5, unwind: bb1];
[00:49:13]             | Live variables on exit from bb3: []
[00:49:13] Actual:
[00:49:13] | Free Region Mapping
[00:49:13] | Free Region Mapping
[00:49:13] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:49:13] | '_#1r    | Local    | ['_#1r]
[00:49:13] | Inferred Region Values
[00:49:13] | Inferred Region Values
[00:49:13] | '_#0r    | {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:49:13] | '_#1r    | {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:49:13] | Inference Constraints
[00:49:13] | Inference Constraints
[00:49:13] | '_#0r live at {'_#0r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:49:13] | '_#1r live at {'_#1r, bb0[0..=3], bb1[0], bb2[0], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=2]}
[00:49:13] fn main() -> (){
[00:49:13]     let mut _0: ();
[00:49:13]     scope 1 {
[00:49:13]     scope 2 {
[00:49:13]         let _1: usize;
[00:49:13]     }
[00:49:13]     }
[00:49:13]     let mut _2: bool;
[00:49:13]     let mut _3: ();
[00:49:13]     let mut _4: usize;
[00:49:13]     let mut _5: ();
[00:49:13]     bb0: {                              
[00:49:13]                                          | Live variables on entry to bb0[0]: []
[00:49:13]         StorageLive(_1);
[00:49:13]                                          | Live variables on entry to bb0[1]: []
[00:49:13]         _1 = const 5usize;
[00:49:13]                                          | Live variables on entry to bb0[2]: []
[00:49:13]         StorageLive(_2);
[00:49:13]                                          | Live variables on entry to bb0[3]: []
[00:49:13]         _2 = const cond() -> [return: bb2, unwind: bb1];
[00:49:13]     | Live variables on exit from bb0: []
[00:49:13]     bb1: {
[00:49:13]     bb1: {
[00:49:13]                                          | Live variables on entry to bb1[0]: []
[00:49:13]         resume;
[00:49:13]     | Live variables on exit from bb1: []
[00:49:13]     }
[00:49:13]     bb2: {                              
[00:49:13]                                          | Live variables on entry to bb2[0]: []
[00:49:13]         switchInt(move _2) -> [false: bb4, otherwise: bb3];
[00:49:13]     | Live variables on exit from bb2: []
[00:49:13]     }
[00:49:13]     bb3: {                              
[00:49:13]                                          | Live variables on entry to bb3[0]: []
[00:49:13]         StorageLive(_4);
[00:49:13]                                          | Live variables on entry to bb3[1]: []
[00:49:13]         _4 = _1;
[00:49:13]                                          | Live variables on entry to bb3[2]: []
[00:49:13]         _3 = const make_live(move _4) -> [return: bb5, unwind: bb1];
[00:49:13]     | Live variables on exit from bb3: []
[00:49:13]     }
[00:49:13]     bb4: {                              
[00:49:13]                                          | Live variables on entry to bb4[0]: []
[00:49:13]         _5 = const make_dead() -> [return: bb6, unwind: bb1];
[00:49:13]     | Live variables on exit from bb4: []
[00:49:13]     }
[00:49:13]     bb5: {                              
[00:49:13]                                          | Live variables on entry to bb5[0]: []
[00:49:13]         StorageDead(_4);
[00:49:13]                                          | Live variables on entry to bb5[1]: []
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb5[2]: []
[00:49:13]         goto -> bb7;
[00:49:13]     | Live variables on exit from bb5: []
[00:49:13]     }
[00:49:13]     bb6: {                              
[00:49:13]                                          | Live variables on entry to bb6[0]: []
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb6[1]: []
[00:49:13]         goto -> bb7;
[00:49:13]     | Live variables on exit from bb6: []
[00:49:13]     }
[00:49:13]     bb7: {                              
[00:49:13]                                          | Live variables on entry to bb7[0]: []
[00:49:13]         StorageDead(_2);
[00:49:13]                                          | Live variables on entry to bb7[1]: []
[00:49:13]         StorageDead(_1);
[00:49:13]                                          | Live variables on entry to bb7[2]: []
[00:49:13]         return;
[00:49:13]     | Live variables on exit from bb7: []
[00:49:13] }', tools/compiletest/src/runtest.rs:2813:13
[00:49:13] 
[00:49:13] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[00:49:13] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[00:49:13] thread '[mir-opt] mir-opt/nll/region-liveness-basic.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:49:13] Current block: None
[00:49:13] Actual Line: "                                         | Live variables on entry to bb2[0]: []"
[00:49:13] Expected Line: "           | Live variables on entry to bb2[0]: [_1, _3]"
[00:49:13] Test Name: rustc.main.nll.0.mir
[00:49:13] Expected:
[00:49:13] ... (elided)
[00:49:13]    bb2: {
[00:49:13]            | Live variables on entry to bb2[0]: [_1, _3]
[00:49:13]        _2 = &'_#2r _1[_3];
[00:49:13]            | Live variables on entry to bb2[1]: [_2]
[00:49:13]        switchInt(const true) -> [false: bb4, otherwise: bb3];
[00:49:13]            | Live variables on exit from bb2: [_2]
[00:49:13] Actual:
[00:49:13] | Free Region Mapping
[00:49:13] | Free Region Mapping
[00:49:13] | '_#0r    | Global   | ['_#0r, '_#1r]
[00:49:13] | '_#1r    | Local    | ['_#1r]
[00:49:13] | Inferred Region Values
[00:49:13] | Inferred Region Values
[00:49:13] | '_#0r    | {'_#0r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[00:49:13] | '_#1r    | {'_#1r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[00:49:13] | '_#2r    | {bb2[0..=1], bb3[0..=1]}
[00:49:13] | '_#3r    | {bb2[1], bb3[0..=1]}
[00:49:13] | Inference Constraints
[00:49:13] | Inference Constraints
[00:49:13] | '_#0r live at {'_#0r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[00:49:13] | '_#1r live at {'_#1r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[00:49:13] | '_#2r live at {bb2[0]}
[00:49:13] | '_#3r live at {bb2[1], bb3[0..=1]}
[00:49:13] | '_#2r: '_#3r due to Interesting(bb2[0])
[00:49:13] fn main() -> (){
[00:49:13]     let mut _0: ();
[00:49:13]     scope 1 {
[00:49:13]         scope 3 {
[00:49:13]         scope 4 {
[00:49:13]         scope 4 {
[00:49:13]             let _2: &'_#3r usize;
[00:49:13]     }
[00:49:13]     scope 2 {
[00:49:13]     scope 2 {
[00:49:13]         let mut _1: [usize; 3];
[00:49:13]     let mut _3: usize;
[00:49:13]     let mut _4: usize;
[00:49:13]     let mut _4: usize;
[00:49:13]     let mut _5: bool;
[00:49:13]     let mut _6: bool;
[00:49:13]     let mut _7: usize;
[00:49:13]     let mut _8: bool;
[00:49:13]     bb0: {                              
[00:49:13]                                          | Live variables on entry to bb0[0]: []
[00:49:13]         StorageLive(_1);
[00:49:13]                                          | Live variables on entry to bb0[1]: []
[00:49:13]         _1 = [const 1usize, const 2usize, const 3usize];
[00:49:13]                                          | Live variables on entry to bb0[2]: []
[00:49:13]         StorageLive(_2);
[00:49:13]                                          | Live variables on entry to bb0[3]: []
[00:49:13]         StorageLive(_3);
[00:49:13]                                          | Live variables on entry to bb0[4]: []
[00:49:13]         _3 = const 0usize;
[00:49:13]                                          | Live variables on entry to bb0[5]: []
[00:49:13]         _4 = Len(_1);
[00:49:13]                                          | Live variables on entry to bb0[6]: []
[00:49:13]         _5 = Lt(_3, _4);
[00:49:13]                                          | Live variables on entry to bb0[7]: []
[00:49:13]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[00:49:13]     | Live variables on exit from bb0: []
[00:49:13]     bb1: {
[00:49:13]     bb1: {
[00:49:13]                                          | Live variables on entry to bb1[0]: []
[00:49:13]         resume;
[00:49:13]     | Live variables on exit from bb1: []
[00:49:13]     }
[00:49:13]     bb2: {                              
[00:49:13]                                          | Live variables on entry to bb2[0]: []
[00:49:13]         _2 = &'_#2r _1[_3];
[00:49:13]                                          | Live variables on entry to bb2[1]: [0]
[00:49:13]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[00:49:13]     | Live variables on exit from bb2: [0]
[00:49:13]     }
[00:49:13]     bb3: {                              
[00:49:13]                                          | Live variables on entry to bb3[0]: [0]
[00:49:13]         StorageLive(_7);
[00:49:13]                                          | Live variables on entry to bb3[1]: [0]
[00:49:13]         _7 = (*_2);
[00:49:13]                                          | Live variables on entry to bb3[2]: []
[00:49:13]         _6 = const use_x(move _7) -> [return: bb5, unwind: bb1];
[00:49:13]     | Live variables on exit from bb3: []
[00:49:13]     }
[00:49:13]     bb4: {                              
[00:49:13]                                          | Live variables on entry to bb4[0]: []
[00:49:13]         _8 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[00:49:13]     | Live variables on exit from bb4: []
[00:49:13]     }
[00:49:13]     bb5: {                              
[00:49:13]                                          | Live variables on entry to bb5[0]: []
[00:49:13]         StorageDead(_7);
[00:49:13]                                          | Live variables on entry to bb5[1]: []
[00:49:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb5[2]: []
[00:49:13]         goto -> bb7;
[00:49:13]     | Live variables on exit from bb5: []
[00:49:13]     }
[00:49:13]     bb6: {                              
[00:49:13]                                          | Live variables on entry to bb6[0]: []
[00:49:13]         _0 = ();
[00:49:13]                                          | Live variables on entry to bb6[1]: []
[00:49:13]         goto -> bb7;
[00:49:13]     | Live variables on exit from bb6: []
[00:49:13]     }
[00:49:13]     bb7: {                              
[00:49:13]                                          | Live variables on entry to bb7[0]: []
[00:49:13]         nop;
[00:49:13]                                          | Live variables on entry to bb7[1]: []
[00:49:13]         StorageDead(_2);
[00:49:13]                                          | Live variables on entry to bb7[2]: []
[00:49:13]         StorageDead(_1);
[00:49:13]                                          | Live variables on entry to bb7[3]: []
[00:49:13]         return;
[00:49:13]     | Live variables on exit from bb7: []
[00:49:13] }', tools/compiletest/src/runtest.rs:2813:13
[00:49:13] 
[00:49:13] 
[00:49:13] failures:
---
[00:49:13] test result: FAILED. 47 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[00:49:13] 
[00:49:13] 
[00:49:13] 
[00:49:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:13] 
[00:49:13] 
[00:49:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:13] Build completed unsuccessfully in 0:09:26
[00:49:13] Build completed unsuccessfully in 0:09:26
[00:49:13] Makefile:58: recipe for target 'check' failed
[00:49:13] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e411493
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:033a50fc:start=1532095423428286964,finish=1532095423434273187,duration=5986223
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0744e790
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02467641
travis_time:start:02467641
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:26f57118
$ dmesg | grep -i kill
