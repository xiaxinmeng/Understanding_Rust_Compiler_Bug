plain
travis_time:end:0bb31640:start=1552976406927820094,finish=1552976410099399707,duration=3171579613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:03] 
[01:19:03] running 39 tests
[01:19:05] ERROR 2019-03-19T07:39:26Z: compiletest::runtest: None
[01:19:08] ERROR 2019-03-19T07:39:28Z: compiletest::runtest: Some("    bb3: {")
[01:19:09] ERROR 2019-03-19T07:39:30Z: compiletest::runtest: None
[01:19:15] ERROR 2019-03-19T07:39:36Z: compiletest::runtest: Some("    bb1: {")
[01:19:16] ERROR 2019-03-19T07:39:36Z: compiletest::runtest: None
[01:19:22] ......F.......F.....F.....F..F.F.......
[01:19:22] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:19:22] ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/deaggregator_test_enum_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:19:22] Current block: None
[01:19:22] Actual Line: "        StorageLive(_5);"
[01:19:22] Expected Line: "     StorageLive(_4);"
[01:19:22] Test Name: rustc.test1.Deaggregator.before.mir
[01:19:22] ... (elided)
[01:19:22]  bb1: {
[01:19:22]  bb1: {
[01:19:22]      StorageLive(_4);
[01:19:22]      _4 = _2;
[01:19:22]      _0 = Foo::A(move _4,);
[01:19:22]      StorageDead(_4);
[01:19:22]      goto -> bb3;
[01:19:22]  bb2: {
[01:19:22]  bb2: {
[01:19:22]      StorageLive(_5);
[01:19:22]      _5 = _2;
[01:19:22]      _0 = Foo::B(move _5,);
[01:19:22]      StorageDead(_5);
[01:19:22]      goto -> bb3;
[01:19:22] Actual:
[01:19:22] | User Type Annotations
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/deaggregator_test_enum_2.rs:9:5: 13:6
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/deaggregator_test_enum_2.rs:9:5: 13:6
[01:19:22] |
[01:19:22] fn  test1(_1: bool, _2: i32) -> Foo {
[01:19:22]     let mut _0: Foo;
[01:19:22]     scope 1 {
[01:19:22]     scope 2 {
[01:19:22]     scope 2 {
[01:19:22]         let _4: bool as UserTypeProjection { base: UserType(0), projs: [] };
[01:19:22]     let mut _3: bool;
[01:19:22]     let mut _5: i32;
[01:19:22]     let mut _6: i32;
[01:19:22]     let mut _6: i32;
[01:19:22]     bb0: {                              
[01:19:22]         StorageLive(_3);
[01:19:22]         StorageLive(_4);
[01:19:22]         _4 = _1;
[01:19:22]         _3 = _4;
[01:19:22]         StorageDead(_4);
[01:19:22]         switchInt(_3) -> [false: bb2, otherwise: bb1];
[01:19:22]     }
[01:19:22]     bb1: {                              
[01:19:22]         StorageLive(_5);
[01:19:22]         _5 = _2;
[01:19:22]         _0 = Foo::A(move _5,);
[01:19:22]         StorageDead(_5);
[01:19:22]         goto -> bb3;
[01:19:22]     }
[01:19:22]     bb2: {                              
[01:19:22]         StorageLive(_6);
[01:19:22]         _6 = _2;
[01:19:22]         _0 = Foo::B(move _6,);
[01:19:22]         StorageDead(_6);
[01:19:22]         goto -> bb3;
[01:19:22]     }
[01:19:22]     bb3: {                              
[01:19:22]         StorageDead(_3);
[01:19:22]         return;
[01:19:22]     bb4: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/issue-38669.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/issue-38669.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:19:22] Current block:     bb3: {
[01:19:22] Actual Line: "        StorageLive(_5);"
[01:19:22] Expected Line: "        _4 = _1;"
[01:19:22] Test Name: rustc.main.SimplifyCfg-initial.after.mir
[01:19:22] ... (elided)
[01:19:22]     bb0: {
[01:19:22]     bb0: {
[01:19:22]         StorageLive(_1);
[01:19:22]         _1 = const false;
[01:19:22]         FakeRead(ForLet, _1);
[01:19:22]         goto -> bb2;
[01:19:22]     bb1: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22]     bb2: {
[01:19:22]     bb2: {
[01:19:22]         falseUnwind -> [real: bb3, cleanup: bb1];
[01:19:22]     bb3: {
[01:19:22]     bb3: {
[01:19:22]         StorageLive(_4);
[01:19:22]         _4 = _1;
[01:19:22]         switchInt(move _4) -> [false: bb5, otherwise: bb4];
[01:19:22]     bb4: {
[01:19:22]         _0 = ();
[01:19:22]         _0 = ();
[01:19:22]         StorageDead(_4);
[01:19:22]         StorageDead(_1);
[01:19:22]         return;
[01:19:22]     bb5: {
[01:19:22]         _3 = ();
[01:19:22]         _3 = ();
[01:19:22]         StorageDead(_4);
[01:19:22]         _2 = ();
[01:19:22]         goto -> bb2;
[01:19:22]     }
[01:19:22] Actual:
[01:19:22] Actual:
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/issue-38669.rs:6:9: 8:10
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/issue-38669.rs:6:9: 8:10
[01:19:22] |
[01:19:22] fn  main() -> () {
[01:19:22]     let mut _0: ();
[01:19:22]     scope 1 {
[01:19:22]         scope 3 {
[01:19:22]         scope 4 {
[01:19:22]         scope 4 {
[01:19:22]             let _5: bool as UserTypeProjection { base: UserType(0), projs: [] };
[01:19:22]     }
[01:19:22]     scope 2 {
[01:19:22]         let mut _1: bool;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     let mut _2: ();
[01:19:22]     let mut _3: ();
[01:19:22]     let mut _4: bool;
[01:19:22]     let mut _6: !;
[01:19:22]     bb0: {                              
[01:19:22]         StorageLive(_1);
[01:19:22]         _1 = const false;
[01:19:22]         FakeRead(ForLet, _1);
[01:19:22]         goto -> bb2;
[01:19:22]     bb1: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb2: {                              
[01:19:22]         falseUnwind -> [real: bb3, cleanup: bb1];
[01:19:22]     }
[01:19:22]     bb3: {                              
[01:19:22]         StorageLive(_4);
[01:19:22]         StorageLive(_5);
[01:19:22]         _5 = _1;
[01:19:22]         FakeRead(ForLet, _5);
[01:19:22]         AscribeUserType(_5, o, UserTypeProjection { base: UserType(1), projs: [] });
[01:19:22]         _4 = _5;
[01:19:22]         StorageDead(_5);
[01:19:22]         FakeRead(ForMatchedPlace, _4);
[01:19:22]         switchInt(_4) -> [false: bb5, otherwise: bb4];
[01:19:22]     }
[01:19:22]     bb4: {                              
[01:19:22]         falseEdges -> [real: bb7, imaginary: bb5];
[01:19:22]     }
[01:19:22]     bb5: {                              
[01:19:22]         falseEdges -> [real: bb8, imaginary: bb6];
[01:19:22]     }
[01:19:22]     bb6: {                              
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb7: {                              
[01:19:22]         _0 = ();
[01:19:22]         StorageDead(_4);
[01:19:22]         StorageDead(_1);
[01:19:22]         return;
[01:19:22]     }
[01:19:22]     bb8: {                              
[01:19:22]         _3 = ();
[01:19:22]         StorageDead(_4);
[01:19:22]         _2 = ();
[01:19:22]         goto -> bb2;
[01:19:22]     }
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/loop_test.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:19:22] Current block: None
[01:19:22] Actual Line: "        falseEdges -> [real: bb6, imaginary: bb4];"
[01:19:22] Expected Line: "       _1 = ();"
[01:19:22] Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
[01:19:22] ... (elided)
[01:19:22] ... (elided)
[01:19:22] ... (elided)
[01:19:22]    bb1: { // The cleanup block
[01:19:22]        resume;
[01:19:22] ... (elided)
[01:19:22] ... (elided)
[01:19:22]    bb3: { // Entry into the loop
[01:19:22]        _1 = ();
[01:19:22]        goto -> bb4;
[01:19:22]    }
[01:19:22]    bb4: { // The loop_block
[01:19:22]        falseUnwind -> [real: bb5, cleanup: bb1];
[01:19:22]    }
[01:19:22]    bb5: { // The loop body (body_block)
[01:19:22]        StorageLive(_5);
[01:19:22]        _5 = const 1i32;
[01:19:22]        FakeRead(ForLet, _5);
[01:19:22]        StorageDead(_5);
[01:19:22]        goto -> bb4;
[01:19:22] ... (elided)
[01:19:22] Actual:
[01:19:22] | User Type Annotations
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/loop_test.rs:9:5: 11:6
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/loop_test.rs:9:5: 11:6
[01:19:22] |
[01:19:22] fn  main() -> () {
[01:19:22]     let mut _0: ();
[01:19:22]     scope 1 {
[01:19:22]     scope 2 {
[01:19:22]     scope 2 {
[01:19:22]         let _3: bool as UserTypeProjection { base: UserType(0), projs: [] };
[01:19:22]     scope 3 {
[01:19:22]     }
[01:19:22]     scope 4 {
[01:19:22]         let _7: i32;
[01:19:22]         let _7: i32;
[01:19:22]     }
[01:19:22]     let mut _1: ();
[01:19:22]     let mut _2: bool;
[01:19:22]     let mut _4: !;
[01:19:22]     let mut _5: !;
[01:19:22]     let mut _6: ();
[01:19:22]     bb0: {                              
[01:19:22]         StorageLive(_2);
[01:19:22]         StorageLive(_3);
[01:19:22]         _3 = const true;
[01:19:22]         FakeRead(ForLet, _3);
[01:19:22]         AscribeUserType(_3, o, UserTypeProjection { base: UserType(1), projs: [] });
[01:19:22]         _2 = _3;
[01:19:22]         StorageDead(_3);
[01:19:22]         FakeRead(ForMatchedPlace, _2);
[01:19:22]         switchInt(_2) -> [false: bb3, otherwise: bb2];
[01:19:22]     bb1: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb2: {                              
[01:19:22]         falseEdges -> [real: bb5, imaginary: bb3];
[01:19:22]     }
[01:19:22]     bb3: {                              
[01:19:22]         falseEdges -> [real: bb6, imaginary: bb4];
[01:19:22]     }
[01:19:22]     bb4: {                              
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb5: {                              
[01:19:22]         _0 = ();
[01:19:22]         StorageDead(_2);
[01:19:22]         return;
[01:19:22]     }
[01:19:22]     bb6: {                              
[01:19:22]         _1 = ();
[01:19:22]         StorageDead(_2);
[01:19:22]         goto -> bb7;
[01:19:22]     }
[01:19:22]     bb7: {                              
[01:19:22]         falseUnwind -> [real: bb8, cleanup: bb1];
[01:19:22]     }
[01:19:22]     bb8: {                              
[01:19:22]         StorageLive(_7);
[01:19:22]         _7 = const 1i32;
[01:19:22]         FakeRead(ForLet, _7);
[01:19:22]         StorageDead(_7);
[01:19:22]         goto -> bb7;
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:19:22] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:19:22] Expected Line: "| \'_#2r    | U0 | {bb2[0..=5], bb3[0..=1]}"
[01:19:22] Test Name: rustc.main.nll.0.mir
[01:19:22] ... (elided)
[01:19:22] ... (elided)
[01:19:22] | '_#2r    | U0 | {bb2[0..=5], bb3[0..=1]}
[01:19:22] | '_#3r    | U0 | {bb2[1..=5], bb3[0..=1]}
[01:19:22] | '_#4r    | U0 | {bb2[4..=5], bb3[0..=1]}
[01:19:22] Actual:
[01:19:22] | Free Region Mapping
[01:19:22] | '_#0r | Global | ['_#0r, '_#1r]
[01:19:22] | '_#1r | Local | ['_#1r]
[01:19:22] | Inferred Region Values
[01:19:22] | Inferred Region Values
[01:19:22] | '_#0r | U0 | {bb0[0..=8], bb1[0], bb2[0..=13], bb3[0], bb4[0], bb5[0], bb6[0..=2], bb7[0..=2], bb8[0], bb9[0..=1], bb10[0..=4], '_#0r, '_#1r}
[01:19:22] | '_#1r | U0 | {bb0[0..=8], bb1[0], bb2[0..=13], bb3[0], bb4[0], bb5[0], bb6[0..=2], bb7[0..=2], bb8[0], bb9[0..=1], bb10[0..=4], '_#1r}
[01:19:22] | '_#2r | U0 | {bb2[0..=13], bb3[0], bb6[0..=1]}
[01:19:22] | '_#3r | U0 | {bb2[1..=13], bb3[0], bb6[0..=1]}
[01:19:22] | '_#4r | U0 | {bb2[4..=13], bb3[0], bb6[0..=1]}
[01:19:22] | Inference Constraints
[01:19:22] | Inference Constraints
[01:19:22] | '_#0r live at {bb0[0..=8], bb1[0], bb2[0..=13], bb3[0], bb4[0], bb5[0], bb6[0..=2], bb7[0..=2], bb8[0], bb9[0..=1], bb10[0..=4]}
[01:19:22] | '_#1r live at {bb0[0..=8], bb1[0], bb2[0..=13], bb3[0], bb4[0], bb5[0], bb6[0..=2], bb7[0..=2], bb8[0], bb9[0..=1], bb10[0..=4]}
[01:19:22] | '_#2r live at {bb2[0]}
[01:19:22] | '_#3r live at {bb2[1..=3]}
[01:19:22] | '_#4r live at {bb2[4..=13], bb3[0], bb6[0..=1]}
[01:19:22] | '_#2r: '_#3r due to Assignment at Single(bb2[0])
[01:19:22] | '_#3r: '_#4r due to Assignment at Single(bb2[3])
[01:19:22] | User Type Annotations
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:16:5: 20:6
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:16:5: 20:6
[01:19:22] |
[01:19:22] fn  main() -> () {
[01:19:22]     let mut _0: ();
[01:19:22]     scope 1 {
[01:19:22]         scope 3 {
[01:19:22]             scope 5 {
[01:19:22]                 scope 7 {
[01:19:22]                 scope 8 {
[01:19:22]                 scope 8 {
[01:19:22]                     let _8: bool as UserTypeProjection { base: UserType(0), projs: [] };
[01:19:22]             }
[01:19:22]             scope 6 {
[01:19:22]             scope 6 {
[01:19:22]                 let _6: &'_#4r usize;
[01:19:22]         }
[01:19:22]         scope 4 {
[01:19:22]         scope 4 {
[01:19:22]             let _2: &'_#3r usize;
[01:19:22]     }
[01:19:22]     scope 2 {
[01:19:22]     scope 2 {
[01:19:22]         let mut _1: [usize; 3];
[01:19:22]     let mut _3: usize;
[01:19:22]     let mut _4: usize;
[01:19:22]     let mut _5: bool;
[01:19:22]     let mut _7: bool;
[01:19:22]     let mut _7: bool;
[01:19:22]     let mut _9: bool;
[01:19:22]     let mut _10: usize;
[01:19:22]     let mut _11: bool;
[01:19:22]     bb0: {                              
[01:19:22]         StorageLive(_1);
[01:19:22]         _1 = [const 1usize, const 2usize, const 3usize];
[01:19:22]         FakeRead(ForLet, _1);
[01:19:22]         StorageLive(_2);
[01:19:22]         StorageLive(_3);
[01:19:22]         _4 = Len(_1);
[01:19:22]         _4 = Len(_1);
[01:19:22]         _5 = Lt(_3, _4);
[01:19:22]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:19:22]     bb1: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb2: {                              
[01:19:22]         _2 = &'_#2r _1[_3];
[01:19:22]         FakeRead(ForLet, _2);
[01:19:22]         StorageLive(_6);
[01:19:22]         _6 = _2;
[01:19:22]         FakeRead(ForLet, _6);
[01:19:22]         StorageLive(_7);
[01:19:22]         StorageLive(_8);
[01:19:22]         _8 = const true;
[01:19:22]         FakeRead(ForLet, _8);
[01:19:22]         AscribeUserType(_8, o, UserTypeProjection { base: UserType(1), projs: [] });
[01:19:22]         _7 = _8;
[01:19:22]         StorageDead(_8);
[01:19:22]         FakeRead(ForMatchedPlace, _7);
[01:19:22]         switchInt(_7) -> [false: bb4, otherwise: bb3];
[01:19:22]     }
[01:19:22]     bb3: {                              
[01:19:22]         falseEdges -> [real: bb6, imaginary: bb4];
[01:19:22]     }
[01:19:22]     bb4: {                              
[01:19:22]         falseEdges -> [real: bb8, imaginary: bb5];
[01:19:22]     }
[01:19:22]     bb5: {                              
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb6: {                              
[01:19:22]         StorageLive(_10);
[01:19:22]         _10 = (*_6);
[01:19:22]         _9 = const use_x(move _10) -> [return: bb7, unwind: bb1];
[01:19:22]     }
[01:19:22]     bb7: {                              
[01:19:22]         StorageDead(_10);
[01:19:22]         _0 = ();
[01:19:22]         goto -> bb10;
[01:19:22]     }
[01:19:22]     bb8: {                              
[01:19:22]         _11 = const use_x(const 22usize) -> [return: bb9, unwind: bb1];
[01:19:22]     }
[01:19:22]     bb9: {                              
[01:19:22]         _0 = ();
[01:19:22]         goto -> bb10;
[01:19:22]     }
[01:19:22]     bb10: {                             
[01:19:22]         StorageDead(_6);
[01:19:22]         StorageDead(_2);
[01:19:22]         StorageDead(_1);
[01:19:22]         StorageDead(_7);
[01:19:22]         return;
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
[01:19:22] ---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/simplify_cfg.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:19:22] Current block:     bb1: {
[01:19:22] Actual Line: "        StorageLive(_3);"
[01:19:22] Expected Line: "        _2 = const bar() -> bb3;"
[01:19:22] Test Name: rustc.main.SimplifyCfg-early-opt.before.mir
[01:19:22] ... (elided)
[01:19:22]     bb0: {
[01:19:22]         goto -> bb1;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb1: {
[01:19:22]         StorageLive(_2);
[01:19:22]         _2 = const bar() -> bb3;
[01:19:22] Actual:
[01:19:22] | User Type Annotations
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/simplify_cfg.rs:5:9: 7:10
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/simplify_cfg.rs:5:9: 7:10
[01:19:22] |
[01:19:22] fn  main() -> () {
[01:19:22]     let mut _0: ();
[01:19:22]     scope 1 {
[01:19:22]     scope 2 {
[01:19:22]     scope 2 {
[01:19:22]         let _3: bool as UserTypeProjection { base: UserType(0), projs: [] };
[01:19:22]     let mut _1: ();
[01:19:22]     let mut _2: bool;
[01:19:22]     let mut _4: !;
[01:19:22]     let mut _4: !;
[01:19:22]     bb0: {                              
[01:19:22]         goto -> bb1;
[01:19:22]     }
[01:19:22]     bb1: {                              
[01:19:22]         StorageLive(_2);
[01:19:22]         StorageLive(_3);
[01:19:22]         _3 = const bar() -> bb3;
[01:19:22]     bb2: {
[01:19:22]         resume;
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb3: {                              
[01:19:22]         nop;
[01:19:22]         _2 = _3;
[01:19:22]         _2 = _3;
[01:19:22]         StorageDead(_3);
[01:19:22]         nop;
[01:19:22]         switchInt(_2) -> [false: bb5, otherwise: bb4];
[01:19:22]     }
[01:19:22]     bb4: {                              
[01:19:22]         goto -> bb7;
[01:19:22]     }
[01:19:22]     bb5: {                              
[01:19:22]         goto -> bb8;
[01:19:22]     }
[01:19:22]     bb6: {                              
[01:19:22]     }
[01:19:22]     }
[01:19:22]     bb7: {                              
[01:19:22]         _0 = ();
[01:19:22]         StorageDead(_2);
[01:19:22]         return;
[01:19:22]     }
[01:19:22]     bb8: {                              
[01:19:22]         _1 = ();
[01:19:22]         StorageDead(_2);
[01:19:22]         goto -> bb0;
[01:19:22] }', src/tools/compiletest/src/runtest.rs:2984:13
[01:19:22] 
[01:19:22] ---- [mir-opt] mir-opt/simplify_if.rs stdout ----
[01:19:22] ---- [mir-opt] mir-opt/simplify_if.rs stdout ----
[01:19:22] thread '[mir-opt] mir-opt/simplify_if.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:19:22] Current block: None
[01:19:22] Actual Line: "        StorageLive(_1);"
[01:19:22] Expected Line: "    switchInt(const false) -> [false: bb3, otherwise: bb2];"
[01:19:22] Test Name: rustc.main.SimplifyBranches-initial.before.mir
[01:19:22] ... (elided)
[01:19:22] bb0: {
[01:19:22] bb0: {
[01:19:22]     switchInt(const false) -> [false: bb3, otherwise: bb2];
[01:19:22] }
[01:19:22] Actual:
[01:19:22] | User Type Annotations
[01:19:22] | 0: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/simplify_if.rs:2:5: 4:6
[01:19:22] | 1: Canonical { max_universe: U0, variables: [], value: Ty(bool) } at /checkout/src/test/mir-opt/simplify_if.rs:2:5: 4:6
[01:19:22] | 2: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Region(U0) }], value: TypeOf(DefId(2/0:4210 ~ core[1e18]::fmt[0]::{{impl}}[2]::new_v1[0]), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BrAnon(0))], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2/0:4209 ~ core[1e18]::fmt[0]::{{impl}}[2]), self_ty: std::fmt::Arguments<'_> }) }) } at <::std::macros::println macros>:2:29: 2:63
[01:19:22] |
[01:19:22] fn  main() -> () {
[01:19:22]     let mut _0: ();
[01:19:22]     scope 1 {
---
[01:19:22] test result: FAILED. 33 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:22] 
[01:19:22] 
[01:19:22] 
[01:19:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:22] 
[01:19:22] 
[01:19:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:22] Build completed unsuccessfully in 0:11:26
[01:19:22] Build completed unsuccessfully in 0:11:26
[01:19:22] Makefile:48: recipe for target 'check' failed
[01:19:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1383b79a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 19 07:39:44 UTC 2019
