plain
travis_time:end:0aaf9568:start=1549340804475426161,finish=1549340877801897200,duration=73326471039
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:12:40] ERROR 2019-02-05T05:40:47Z: compiletest::runtest: None
[01:12:41] ERROR 2019-02-05T05:40:47Z: compiletest::runtest: None
[01:12:45] ERROR 2019-02-05T05:40:52Z: compiletest::runtest: None
[01:12:54] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:12:54] ...F.............F..F....FF...........
[01:12:54] 
[01:12:54] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:12:54] ---- [mir-opt] mir-opt/box_expr.rs stdout ----
[01:12:54] thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:54] Current block: None
[01:12:54] Actual Line: "    let _1: std::boxed::Box<S>;"
[01:12:54] Expected Line: "    scope 1 {"
[01:12:54] Test Name: rustc.main.ElaborateDrops.before.mir
[01:12:54] ... (elided)
[01:12:54] ... (elided)
[01:12:54]     let mut _0: ();
[01:12:54]     scope 1 {
[01:12:54]     scope 2 {
[01:12:54]     scope 2 {
[01:12:54]         let _1: std::boxed::Box<S>;
[01:12:54]     }
[01:12:54]     let mut _2: std::boxed::Box<S>;
[01:12:54]     let mut _3: ();
[01:12:54]     let mut _4: std::boxed::Box<S>;
[01:12:54]     bb0: {
[01:12:54]         StorageLive(_1);
[01:12:54]         StorageLive(_2);
[01:12:54]         _2 = Box(S);
[01:12:54]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     bb2: {
[01:12:54]     bb2: {
[01:12:54]         _1 = move _2;
[01:12:54]         drop(_2) -> bb4;
[01:12:54]     }
[01:12:54]     bb3: {
[01:12:54]         drop(_2) -> bb1;
[01:12:54]     }
[01:12:54]     bb4: {
[01:12:54]         StorageDead(_2);
[01:12:54]         StorageLive(_4);
[01:12:54]         _4 = move _1;
[01:12:54]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:12:54]     bb5: {
[01:12:54]     bb5: {
[01:12:54]         drop(_4) -> [return: bb8, unwind: bb6];
[01:12:54]     bb6: {
[01:12:54]         drop(_1) -> bb1;
[01:12:54]     }
[01:12:54]     bb7: {
[01:12:54]     bb7: {
[01:12:54]         drop(_4) -> bb6;
[01:12:54]     }
[01:12:54]     bb8: {
[01:12:54]         StorageDead(_4);
[01:12:54]         _0 = ();
[01:12:54]         drop(_1) -> bb9;
[01:12:54]     bb9: {
[01:12:54]     bb9: {
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54] }
[01:12:54] Actual:
[01:12:54] fn main() -> (){
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     let _1: std::boxed::Box<S>;
[01:12:54]     scope 1 {
[01:12:54]     }
[01:12:54]     let mut _2: std::boxed::Box<S>;
[01:12:54]     let mut _3: ();
[01:12:54]     let mut _4: std::boxed::Box<S>;
[01:12:54]     bb0: {                              
[01:12:54]         StorageLive(_1);
[01:12:54]         StorageLive(_2);
[01:12:54]         _2 = Box(S);
[01:12:54]         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb2: {                              
[01:12:54]         _1 = move _2;
[01:12:54]         drop(_2) -> bb4;
[01:12:54]     bb3: {
[01:12:54]         drop(_2) -> bb1;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb4: {                              
[01:12:54]         StorageDead(_2);
[01:12:54]         StorageLive(_4);
[01:12:54]         _4 = move _1;
[01:12:54]         _3 = const std::mem::drop(move _4) -> [return: bb5, unwind: bb7];
[01:12:54]     }
[01:12:54]     bb5: {                              
[01:12:54]         drop(_4) -> [return: bb8, unwind: bb6];
[01:12:54]     bb6: {
[01:12:54]         drop(_1) -> bb1;
[01:12:54]     }
[01:12:54]     bb7: {
[01:12:54]     bb7: {
[01:12:54]         drop(_4) -> bb6;
[01:12:54]     }
[01:12:54]     bb8: {                              
[01:12:54]         StorageDead(_4);
[01:12:54]         _0 = ();
[01:12:54]         drop(_1) -> bb9;
[01:12:54]     }
[01:12:54]     bb9: {                              
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:54] 
[01:12:54] ---- [mir-opt] mir-opt/issue-41110.rs stdout ----
[01:12:54] ---- [mir-opt] mir-opt/issue-41110.rs stdout ----
[01:12:54] thread '[mir-opt] mir-opt/issue-41110.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:54] Current block: None
[01:12:54] Actual Line: "    let _1: ();"
[01:12:54] Expected Line: "   scope 1 {"
[01:12:54] Test Name: rustc.main.ElaborateDrops.after.mir
[01:12:54] ... (elided)
[01:12:54] ... (elided)
[01:12:54]    let mut _0: ();
[01:12:54]    scope 1 {
[01:12:54]    scope 2 {
[01:12:54]    scope 2 {
[01:12:54]        let _1: ();
[01:12:54]    let mut _2: S;
[01:12:54]    let mut _3: S;
[01:12:54]    let mut _4: S;
[01:12:54]    let mut _4: S;
[01:12:54]    let mut _5: bool;
[01:12:54]    bb0: {
[01:12:54] Actual:
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     let _1: ();
[01:12:54]     scope 1 {
[01:12:54]     let mut _2: S;
[01:12:54]     let mut _3: S;
[01:12:54]     let mut _4: S;
[01:12:54]     let mut _4: S;
[01:12:54]     let mut _5: bool;
[01:12:54]     bb0: {                              
[01:12:54]         _5 = const false;
[01:12:54]         StorageLive(_1);
[01:12:54]         StorageLive(_2);
[01:12:54]         _2 = S;
[01:12:54]         _2 = S;
[01:12:54]         StorageLive(_3);
[01:12:54]         StorageLive(_4);
[01:12:54]         _4 = S;
[01:12:54]         _3 = const S::id(move _4) -> [return: bb2, unwind: bb4];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb2: {                              
[01:12:54]         goto -> bb6;
[01:12:54]     bb3: {
[01:12:54]         goto -> bb12;
[01:12:54]     }
[01:12:54]     bb4: {
[01:12:54]     bb4: {
[01:12:54]         goto -> bb3;
[01:12:54]     }
[01:12:54]     bb5: {
[01:12:54]         goto -> bb3;
[01:12:54]     }
[01:12:54]     bb6: {                              
[01:12:54]         StorageDead(_4);
[01:12:54]         _5 = const false;
[01:12:54]         _1 = const S::other(move _2, move _3) -> [return: bb7, unwind: bb5];
[01:12:54]     }
[01:12:54]     bb7: {                              
[01:12:54]         goto -> bb8;
[01:12:54]     }
[01:12:54]     bb8: {                              
[01:12:54]         StorageDead(_3);
[01:12:54]         goto -> bb9;
[01:12:54]     }
[01:12:54]     bb9: {                              
[01:12:54]         _5 = const false;
[01:12:54]         StorageDead(_2);
[01:12:54]         _0 = ();
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54]     bb10: {
[01:12:54]         drop(_2) -> bb1;
[01:12:54]     }
[01:12:54]     bb11: {
[01:12:54]     bb11: {
[01:12:54]         _5 = const false;
[01:12:54]         goto -> bb10;
[01:12:54]     }
[01:12:54]     bb12: {
[01:12:54]         switchInt(_5) -> [false: bb1, otherwise: bb11];
[01:12:54] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:54] 
[01:12:54] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[01:12:54] ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
[01:12:54] thread '[mir-opt] mir-opt/issue-49232.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:54] Current block: None
[01:12:54] Actual Line: "    let _2: i32;"
[01:12:54] Expected Line: "    scope 1 {"
[01:12:54] Test Name: rustc.main.mir_map.0.mir
[01:12:54] ... (elided)
[01:12:54] fn main() -> (){
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     scope 1 {
[01:12:54]     scope 2 {
[01:12:54]         let _2: i32;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     let mut _1: ();
[01:12:54]     let mut _3: bool;
[01:12:54]     let mut _4: !;
[01:12:54]     let mut _5: ();
[01:12:54]     let mut _6: &i32;
[01:12:54]     bb0: {
[01:12:54]         goto -> bb1;
[01:12:54]     bb1: {
[01:12:54]     bb1: {
[01:12:54]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:12:54]     bb2: {
[01:12:54]         goto -> bb20;
[01:12:54]     }
[01:12:54]     bb3: {
[01:12:54]     bb3: {
[01:12:54]         StorageLive(_2);
[01:12:54]         StorageLive(_3);
[01:12:54]         _3 = const true;
[01:12:54]         FakeRead(ForMatchedPlace, _3);
[01:12:54]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[01:12:54]     bb4: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     bb5: {
[01:12:54]     bb5: {
[01:12:54]         _2 = const 4i32;
[01:12:54]         goto -> bb14;
[01:12:54]     }
[01:12:54]     bb6: {
[01:12:54]         _0 = ();
[01:12:54]         goto -> bb15;
[01:12:54]     }
[01:12:54]     bb7: {
[01:12:54]         falseEdges -> [real: bb12, imaginary: bb8];
[01:12:54]     bb8: {
[01:12:54]     bb8: {
[01:12:54]         falseEdges -> [real: bb13, imaginary: bb9];
[01:12:54]     bb9: {
[01:12:54]         unreachable;
[01:12:54]     }
[01:12:54]     bb10: {
---
[01:12:54]     }
[01:12:54]     bb13: {
[01:12:54]         goto -> bb6;
[01:12:54]     }
[01:12:54]     bb14: {
[01:12:54]         FakeRead(ForLet, _2);
[01:12:54]         StorageDead(_3);
[01:12:54]         StorageLive(_6);
[01:12:54]         _6 = &_2;
[01:12:54]         _5 = const std::mem::drop(move _6) -> [return: bb19, unwind: bb4];
[01:12:54]     bb15: {
[01:12:54]     bb15: {
[01:12:54]         StorageDead(_3);
[01:12:54]         goto -> bb16;
[01:12:54]     bb16: {
[01:12:54]     bb16: {
[01:12:54]         StorageDead(_2);
[01:12:54]         goto -> bb2;
[01:12:54]     bb17: {
[01:12:54]         _4 = ();
[01:12:54]         unreachable;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb18: {
[01:12:54]         StorageDead(_4);
[01:12:54]         goto -> bb14;
[01:12:54]     bb19: {
[01:12:54]     bb19: {
[01:12:54]         StorageDead(_6);
[01:12:54]         _1 = ();
[01:12:54]         StorageDead(_2);
[01:12:54]         goto -> bb1;
[01:12:54]     bb20: {
[01:12:54]         return;
[01:12:54]     }
[01:12:54] }
[01:12:54] }
[01:12:54] Actual:
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     let _2: i32;
[01:12:54]     scope 1 {
[01:12:54]     }
[01:12:54]     let mut _1: ();
[01:12:54]     let mut _3: bool;
[01:12:54]     let mut _4: !;
[01:12:54]     let mut _5: ();
[01:12:54]     let mut _6: &i32;
[01:12:54]     bb0: {                              
[01:12:54]         goto -> bb1;
[01:12:54]     }
[01:12:54]     bb1: {                              
[01:12:54]         falseUnwind -> [real: bb3, cleanup: bb4];
[01:12:54]     }
[01:12:54]     bb2: {                              
[01:12:54]         goto -> bb20;
[01:12:54]     }
[01:12:54]     bb3: {                              
[01:12:54]         StorageLive(_2);
[01:12:54]         StorageLive(_3);
[01:12:54]         _3 = const true;
[01:12:54]         FakeRead(ForMatchedPlace, _3);
[01:12:54]         switchInt(_3) -> [false: bb11, otherwise: bb10];
[01:12:54]     bb4: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb5: {                              
[01:12:54]         _2 = const 4i32;
[01:12:54]         goto -> bb14;
[01:12:54]     }
[01:12:54]     bb6: {                              
[01:12:54]         _0 = ();
[01:12:54]         goto -> bb15;
[01:12:54]     }
[01:12:54]     bb7: {                              
[01:12:54]         falseEdges -> [real: bb12, imaginary: bb8];
[01:12:54]     }
[01:12:54]     bb8: {                              
[01:12:54]         falseEdges -> [real: bb13, imaginary: bb9];
[01:12:54]     }
[01:12:54]     bb9: {                              
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb10: {                             
[01:12:54]         goto -> bb8;
[01:12:54]     }
[01:12:54]     bb11: {                             
[01:12:54]         goto -> bb7;
[01:12:54]     }
[01:12:54]     bb12: {                             
[01:12:54]         goto -> bb5;
[01:12:54]     }
[01:12:54]     bb13: {                             
[01:12:54]         goto -> bb6;
[01:12:54]     }
[01:12:54]     bb14: {                             
[01:12:54]         FakeRead(ForLet, _2);
[01:12:54]         StorageDead(_3);
[01:12:54]         StorageLive(_6);
[01:12:54]         _6 = &_2;
[01:12:54]         _5 = const std::mem::drop(move _6) -> [return: bb19, unwind: bb4];
[01:12:54]     }
[01:12:54]     bb15: {                             
[01:12:54]         StorageDead(_3);
[01:12:54]         goto -> bb16;
[01:12:54]     }
[01:12:54]     bb16: {                             
[01:12:54]         StorageDead(_2);
[01:12:54]         goto -> bb2;
[01:12:54]     }
[01:12:54]     bb17: {                             
[01:12:54]         _4 = ();
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb18: {                             
[01:12:54]         StorageDead(_4);
[01:12:54]         goto -> bb14;
[01:12:54]     }
[01:12:54]     bb19: {                             
[01:12:54]         StorageDead(_6);
[01:12:54]         _1 = ();
[01:12:54]         StorageDead(_2);
[01:12:54]         goto -> bb1;
[01:12:54]     }
[01:12:54]     bb20: {                             
[01:12:54]         return;
[01:12:54] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:54] 
[01:12:54] ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
[01:12:54] ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
[01:12:54] thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:12:54] Current block: None
[01:12:54] Actual Line: "    let mut _1: Packed;"
[01:12:54] Expected Line: "    scope 1 {"
[01:12:54] Test Name: rustc.main.EraseRegions.before.mir
[01:12:54] ... (elided)
[01:12:54] fn main() -> () {
[01:12:54] fn main() -> () {
[01:12:54]     let mut _0: ();
[01:12:54]     scope 1 {
[01:12:54]     scope 2 {
[01:12:54]     scope 2 {
[01:12:54]         let mut _1: Packed;
[01:12:54]     let mut _2: Aligned;
[01:12:54]     let mut _2: Aligned;
[01:12:54]     let mut _3: Droppy;
[01:12:54]     let mut _4: Aligned;
[01:12:54]     let mut _5: Droppy;
[01:12:54]     let mut _6: Aligned;
[01:12:54]     bb0: {
[01:12:54]         StorageLive(_1);
[01:12:54] ... (elided)
[01:12:54]         _1 = Packed(move _2,);
[01:12:54] ... (elided)
[01:12:54]         StorageLive(_6);
[01:12:54]         _6 = move (_1.0: Aligned);
[01:12:54]         drop(_6) -> [return: bb4, unwind: bb3];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     bb2: {
[01:12:54]     bb2: {
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54]     bb3: {
[01:12:54]     bb3: {
[01:12:54]         (_1.0: Aligned) = move _4;
[01:12:54]         drop(_1) -> bb1;
[01:12:54]     bb4: {
[01:12:54]     bb4: {
[01:12:54]         StorageDead(_6);
[01:12:54]         (_1.0: Aligned) = move _4;
[01:12:54]         StorageDead(_4);
[01:12:54]         _0 = ();
[01:12:54]         drop(_1) -> [return: bb2, unwind: bb1];
[01:12:54] }
[01:12:54] Actual:
[01:12:54] fn main() -> (){
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     let mut _1: Packed;
[01:12:54]     scope 1 {
[01:12:54]     let mut _2: Aligned;
[01:12:54]     let mut _2: Aligned;
[01:12:54]     let mut _3: Droppy;
[01:12:54]     let mut _4: Aligned;
[01:12:54]     let mut _5: Droppy;
[01:12:54]     let mut _6: Aligned;
[01:12:54]     bb0: {                              
[01:12:54]         StorageLive(_1);
[01:12:54]         StorageLive(_2);
[01:12:54]         StorageLive(_3);
[01:12:54]         _3 = Droppy(const 0usize,);
[01:12:54]         _2 = Aligned(move _3,);
[01:12:54]         StorageDead(_3);
[01:12:54]         _1 = Packed(move _2,);
[01:12:54]         StorageDead(_2);
[01:12:54]         StorageLive(_4);
[01:12:54]         StorageLive(_5);
[01:12:54]         _5 = Droppy(const 0usize,);
[01:12:54]         _4 = Aligned(move _5,);
[01:12:54]         StorageDead(_5);
[01:12:54]         StorageLive(_6);
[01:12:54]         _6 = move (_1.0: Aligned);
[01:12:54]         drop(_6) -> [return: bb4, unwind: bb3];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb2: {                              
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54]     bb3: {
[01:12:54]     bb3: {
[01:12:54]         (_1.0: Aligned) = move _4;
[01:12:54]         drop(_1) -> bb1;
[01:12:54]     }
[01:12:54]     bb4: {                              
[01:12:54]         StorageDead(_6);
[01:12:54]         (_1.0: Aligned) = move _4;
[01:12:54]         StorageDead(_4);
[01:12:54]         _0 = ();
[01:12:54]         drop(_1) -> [return: bb2, unwind: bb1];
[01:12:54] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:54] 
[01:12:54] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:12:54] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:12:54] thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:12:54] Expected Line: "let _2: &\'_#3r usize;"
[01:12:54] Test Name: rustc.main.nll.0.mir
[01:12:54] ... (elided)
[01:12:54] ... (elided)
[01:12:54] let _6: &'_#4r usize;
[01:12:54] ... (elided)
[01:12:54] let _2: &'_#3r usize;
[01:12:54] ... (elided)
[01:12:54] _2 = &'_#2r _1[_3];
[01:12:54] ... (elided)
[01:12:54] _6 = _2;
[01:12:54] Actual:
[01:12:54] | Free Region Mapping
[01:12:54] | '_#0r | Global | ['_#0r, '_#1r]
[01:12:54] | '_#1r | Local | ['_#1r]
[01:12:54] | Inferred Region Values
[01:12:54] | Inferred Region Values
[01:12:54] | '_#0r | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3], '_#0r, '_#1r}
[01:12:54] | '_#1r | U0 | {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3], '_#1r}
[01:12:54] | '_#2r | U0 | {bb2[0..=5], bb3[0..=1]}
[01:12:54] | '_#3r | U0 | {bb2[1..=5], bb3[0..=1]}
[01:12:54] | '_#4r | U0 | {bb2[4..=5], bb3[0..=1]}
[01:12:54] | Inference Constraints
[01:12:54] | Inference Constraints
[01:12:54] | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[01:12:54] | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=5], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[01:12:54] | '_#2r live at {bb2[0]}
[01:12:54] | '_#3r live at {bb2[1..=3]}
[01:12:54] | '_#4r live at {bb2[4..=5], bb3[0..=1]}
[01:12:54] | '_#2r: '_#3r due to Assignment at Single(bb2[0])
[01:12:54] | '_#3r: '_#4r due to Assignment at Single(bb2[3])
[01:12:54] fn main() -> (){
[01:12:54] fn main() -> (){
[01:12:54]     let mut _0: ();
[01:12:54]     span: /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:12:1: 21:2
[01:12:54]     lint_root: NodeId(0)
[01:12:54]     safety: Safe
[01:12:54]     let mut _1: [usize; 3];
[01:12:54]     scope 1 {
[01:12:54]         span: /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:13:5: 21:2
[01:12:54]         lint_root: NodeId(0)
[01:12:54]         safety: Safe
[01:12:54]         let _2: &'_#3r usize;
[01:12:54]         scope 2 {
[01:12:54]             span: /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:14:5: 21:2
[01:12:54]             lint_root: NodeId(0)
[01:12:54]             safety: Safe
[01:12:54]             let _6: &'_#4r usize;
[01:12:54]             scope 3 {
[01:12:54]                 span: /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:15:5: 21:2
[01:12:54]                 lint_root: NodeId(0)
[01:12:54]                 safety: Safe
[01:12:54]         }
[01:12:54]     }
[01:12:54]     let mut _3: usize;
[01:12:54]     let mut _4: usize;
[01:12:54]     let mut _4: usize;
[01:12:54]     let mut _5: bool;
[01:12:54]     let mut _7: bool;
[01:12:54]     let mut _8: usize;
[01:12:54]     let mut _9: bool;
[01:12:54]     bb0: {                              
[01:12:54]         StorageLive(_1);
[01:12:54]         _1 = [const 1usize, const 2usize, const 3usize];
[01:12:54]         FakeRead(ForLet, _1);
[01:12:54]         StorageLive(_2);
[01:12:54]         StorageLive(_3);
[01:12:54]         _3 = const 0usize;
[01:12:54]         _4 = Len(_1);
[01:12:54]         _5 = Lt(_3, _4);
[01:12:54]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:12:54]     bb1: {
[01:12:54]         resume;
[01:12:54]     }
[01:12:54]     }
[01:12:54]     bb2: {                              
[01:12:54]         _2 = &'_#2r _1[_3];
[01:12:54]         FakeRead(ForLet, _2);
[01:12:54]         StorageLive(_6);
[01:12:54]         _6 = _2;
[01:12:54]         FakeRead(ForLet, _6);
[01:12:54]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:12:54]     }
[01:12:54]     bb3: {                              
[01:12:54]         StorageLive(_8);
[01:12:54]         _8 = (*_6);
[01:12:54]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:12:54]     }
[01:12:54]     bb4: {                              
[01:12:54]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:12:54]     }
[01:12:54]     bb5: {                              
[01:12:54]         StorageDead(_8);
[01:12:54]         _0 = ();
[01:12:54]         goto -> bb7;
[01:12:54]     }
[01:12:54]     bb6: {                              
[01:12:54]         _0 = ();
[01:12:54]         goto -> bb7;
[01:12:54]     }
[01:12:54]     bb7: {                              
[01:12:54]         StorageDead(_6);
[01:12:54]         StorageDead(_2);
[01:12:54]         StorageDead(_1);
[01:12:54]         return;
[01:12:54] }', src/tools/compiletest/src/runtest.rs:2960:13
[01:12:54] 
[01:12:54] 
[01:12:54] failures:
---
[01:12:54] test result: FAILED. 33 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
[01:12:54] 
[01:12:54] 
[01:12:54] 
[01:12:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:54] 
[01:12:54] 
[01:12:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:54] Build completed unsuccessfully in 0:11:40
[01:12:54] Build completed unsuccessfully in 0:11:40
[01:12:54] make: *** [check] Error 1
[01:12:54] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d0d0c52
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 05:41:01 UTC 2019
---
travis_time:end:054621c0:start=1549345263008631251,finish=1549345263013930324,duration=5299073
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2425af10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fffd79c
travis_time:start:1fffd79c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0422e756
$ dmesg | grep -i kill
